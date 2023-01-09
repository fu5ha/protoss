use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{parse::Parse, parse_quote, spanned::Spanned, Error, Ident, ItemStruct, Meta, Token};

mod kw {
    syn::custom_keyword!(id);
    syn::custom_keyword!(since_ev);
}

pub struct FieldAttrs {
    id: u16,
    since_ev: u16,
}

impl Parse for FieldAttrs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<kw::id>()?;
        input.parse::<Token![=]>()?;
        let id_lit = input.parse::<syn::LitInt>()?;
        let id = id_lit.base10_parse::<u16>()?;

        input.parse::<Token![,]>()?;

        input.parse::<kw::since_ev>()?;
        input.parse::<Token![=]>()?;
        let since_ev_lit = input.parse::<syn::LitInt>()?;
        let since_ev = since_ev_lit.base10_parse::<u16>()?;

        Ok(Self { id, since_ev })
    }
}

#[derive(Clone)]
struct FieldDescriptor {
    attr_span: Span,
    id: u16,
    since_ev: u16,
    ident: syn::Ident,
    ty: syn::Type,
    archived_ty: syn::Type,
    with_ty: Option<syn::Type>,
}

impl FieldDescriptor {
    fn try_from_field(field: &syn::Field) -> syn::Result<Self> {
        let field_attr = field
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("field"))
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    field,
                    "all fields must be annotated with a single `#[field(...)]` attribute",
                )
            })?;

        let FieldAttrs { id, since_ev } = field_attr.parse_args()?;

        let base_ty = field.ty.clone();

        let (archived_ty, with_ty) =
            if let Some(with_attr) = field.attrs.iter().find(|attr| attr.path.is_ident("with")) {
                let with_ty: syn::Type = with_attr.parse_args()?;
                let archived_ty =
                    parse_quote!(::rkyv::Archived<::rkyv::with::With<#base_ty, #with_ty>>);
                let with_ty = Some(with_ty);
                (archived_ty, with_ty)
            } else {
                let archived_ty = parse_quote!(::rkyv::Archived<#base_ty>);
                let with_ty = None;
                (archived_ty, with_ty)
            };

        let ident = field
            .ident
            .clone()
            .ok_or_else(|| syn::Error::new_spanned(field, "only named fields are supported"))?;

        Ok(Self {
            attr_span: field_attr.span(),
            id,
            since_ev,
            ident,
            ty: base_ty,
            archived_ty,
            with_ty,
        })
    }

    fn archived(&self) -> ArchivedFieldDescriptor {
        ArchivedFieldDescriptor::DataField {
            ident: self.ident.clone(),
            ty: self.archived_ty.clone(),
        }
    }
}

#[derive(Clone)]
enum ArchivedFieldDescriptor {
    DataField { ident: syn::Ident, ty: syn::Type },
    EvolutionSeparatorField { ev: u16, types: Vec<syn::Type> },
}

struct EvolutionDescriptor {
    ev: u16,
    fields: Vec<FieldDescriptor>,
    archived_fields: Vec<ArchivedFieldDescriptor>,
}

fn collect_evolutions(
    fields: &syn::Fields,
) -> Result<(Vec<EvolutionDescriptor>, Vec<syn::Field>), Error> {
    let mut field_descriptors = Vec::new();
    let mut stripped_fields = Vec::new();
    let mut latest_evolution = 0;
    match fields {
        syn::Fields::Named(ref fields) => {
            for field in fields.named.iter() {
                let field_desc = FieldDescriptor::try_from_field(field)?;
                latest_evolution = latest_evolution.max(field_desc.since_ev);
                field_descriptors.push(field_desc);

                let mut stripped = field.clone();
                stripped.attrs = field
                    .attrs
                    .iter()
                    .filter(|attr| !attr.path.is_ident("field"))
                    .cloned()
                    .collect();
                stripped_fields.push(stripped);
            }
        }
        _ => {
            return Err(Error::new_spanned(
                fields,
                "protoss may only be used on structs with named fields",
            ))
        }
    };

    field_descriptors.sort_by_key(|desc| desc.id);

    let mut evolution_descriptors: Vec<_> = (0..latest_evolution + 1)
        .into_iter()
        .map(|ev| EvolutionDescriptor {
            ev,
            fields: Vec::new(),
            archived_fields: Vec::new(),
        })
        .collect();

    fn push_archived(
        evolution_descriptors: &mut [EvolutionDescriptor],
        ev: u16,
        archived_desc: &ArchivedFieldDescriptor,
    ) {
        for desc in &mut evolution_descriptors[(ev as usize)..] {
            desc.archived_fields.push(archived_desc.clone());
        }
    }

    fn push_raw(
        evolution_descriptors: &mut [EvolutionDescriptor],
        ev: u16,
        field: &FieldDescriptor,
    ) {
        for desc in &mut evolution_descriptors[(ev as usize)..] {
            desc.fields.push(field.clone());
        }
    }

    let mut current_ev = 0;
    for (i, field) in field_descriptors.iter().enumerate() {
        if field.since_ev > current_ev {
            let separator = ArchivedFieldDescriptor::EvolutionSeparatorField {
                ev: current_ev,
                types: field_descriptors[0..i]
                    .iter()
                    .map(|desc| desc.archived_ty.clone())
                    .collect(),
            };
            push_archived(&mut evolution_descriptors, current_ev, &separator);

            current_ev = field.since_ev;
        } else if field.since_ev < current_ev {
            return Err(Error::new(field.attr_span, "the field with previous id has a higher initial evolution; fields cannot be added to previous evolutions"));
        }

        push_archived(&mut evolution_descriptors, current_ev, &field.archived());
        push_raw(&mut evolution_descriptors, current_ev, field);
    }

    Ok((evolution_descriptors, stripped_fields))
}

fn ev_struct_name(base_name: &syn::Ident, ev: u16) -> syn::Ident {
    syn::Ident::new(&format!("{}Ev{}", base_name, ev), base_name.span())
}

fn archived_ev_struct_name(base_name: &syn::Ident, ev: u16) -> syn::Ident {
    syn::Ident::new(&format!("Archived{}Ev{}", base_name, ev), base_name.span())
}

fn ev_pad_field_name(ev: u16) -> syn::Ident {
    Ident::new(&format!("_pad_ev{}", ev), Span::call_site())
}

pub fn expand(_attr: &Option<Meta>, input: &ItemStruct) -> Result<TokenStream, Error> {
    let base_name = &input.ident;
    let vis = &input.vis;
    let generics = &input.generics;

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let where_clause = where_clause.unwrap();

    let (ev_attr_attrs, base_attrs) = input
        .attrs
        .iter()
        .cloned()
        .partition::<Vec<syn::Attribute>, _>(|attr| {
            attr.path.is_ident("evolution_attr") || attr.path.is_ident("archived_evolution_attr")
        });
    let mut ev_attrs = Vec::new();
    let mut archived_ev_attrs = Vec::new();
    for ev_attr in ev_attr_attrs {
        if let Meta::List(list) = ev_attr.parse_meta()? {
            for nested in list.nested.iter() {
                if let syn::NestedMeta::Meta(meta) = nested {
                    if ev_attr.path.is_ident("evolution_attr") {
                        ev_attrs.push(meta.clone());
                    } else if ev_attr.path.is_ident("archived_evolution_attr") {
                        archived_ev_attrs.push(meta.clone());
                    }
                } else {
                    return Err(Error::new_spanned(
                        nested,
                        "[archived_]evolution_attr arguments must be metas",
                    ));
                }
            }
        } else {
            return Err(Error::new_spanned(
                ev_attr,
                "[archived_]evolution_attr may only be structured list attributes",
            ));
        }
    }

    let probe_name = Ident::new(&format!("{}Probe", base_name), base_name.span());

    let (evolutions, stripped_fields) = collect_evolutions(&input.fields)?;

    let evolution_structs = evolutions.iter().map(|desc| {
        let struct_name = ev_struct_name(base_name, desc.ev);
        let archived_name = archived_ev_struct_name(base_name, desc.ev);
        let archived_name_string = format!("{}", archived_name);
        let fields = desc.fields.iter().map(|field| {
            let with_attr = field.with_ty.as_ref().map(|with_ty| quote!(#[with(#with_ty)])).unwrap_or(quote!());
            let ident = &field.ident;
            let ty = &field.ty;
            quote!(#with_attr pub #ident: #ty)
        });

        quote! {
            #(#[#ev_attrs])*
            #[derive(::protoss::rkyv_impl::rkyv::Archive, ::protoss::rkyv_impl::rkyv::Serialize, ::protoss::rkyv_impl::rkyv::Deserialize)]
            #[archive(as = #archived_name_string)]
            #vis struct #struct_name #generics {
                #(#fields,)*
            }
        }
    });

    let archived_evolution_structs = evolutions.iter().map(|desc| {
        let archived_name = archived_ev_struct_name(base_name, desc.ev);
        let fields = desc.archived_fields.iter().map(|f| match f {
            ArchivedFieldDescriptor::DataField { ident, ty } => quote!(pub #ident: #ty),
            ArchivedFieldDescriptor::EvolutionSeparatorField { ev, types } => {
                let ident = ev_pad_field_name(*ev);

                quote!(#ident: ::protoss::rkyv_impl::PadToAlign<(#(#types,)*)>)
            }
        });

        let unarchived_field_tys = desc.fields.iter().map(|field| &field.ty);

        let field_where_clause = quote!(where #(#unarchived_field_tys: ::rkyv::Archive,)*);

        let field_args = desc.archived_fields.iter().filter_map(|field| match field {
            ArchivedFieldDescriptor::DataField { ident, ty } => Some(quote!(#ident: #ty)),
            ArchivedFieldDescriptor::EvolutionSeparatorField { .. } => None,
        });

        let field_constructors = desc.archived_fields.iter().map(|field| match field {
            ArchivedFieldDescriptor::DataField { ident, .. } => quote!(#ident),
            ArchivedFieldDescriptor::EvolutionSeparatorField { ev, .. } => {
                let ident = ev_pad_field_name(*ev);

                quote!(#ident: Default::default())
            }
        });

        quote! {
            #[repr(C)]
            #(#[#archived_ev_attrs])*
            #vis struct #archived_name #generics #field_where_clause {
                #(#fields,)*
            }

            impl #impl_generics #archived_name #ty_generics #where_clause {
                pub fn new(#(#field_args,)*) -> Self {
                    Self {
                        #(#field_constructors,)*
                    }
                }
            }
        }
    });

    let impl_evolutions = evolutions.iter().map(|desc| {
        let struct_name = ev_struct_name(base_name, desc.ev);
        let ev = desc.ev;

        quote! {
            unsafe impl #impl_generics ::protoss::Evolution for #struct_name #ty_generics #where_clause {
                type Base = #base_name;
                const VERSION: ::protoss::Version = ::protoss::Version::new(#ev);
                const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<<Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived>() as ::protoss::ProbeMetadata;
            }
        }
    });

    let impl_probe = {
        let evolution_map = evolutions.iter().map(|desc| {
            let evolution_name = ev_struct_name(base_name, desc.ev);
            quote! { <#evolution_name as ::protoss::Evolution>::METADATA => Some(<#evolution_name as ::protoss::Evolution>::VERSION) }
        });

        let field_accessors = evolutions.last().unwrap().fields.iter().map(|field| {
            let name = &field.ident;
            let archived_ty = &field.archived_ty;
            let required_ev_name = ev_struct_name(base_name, field.since_ev);
            quote! {
                pub fn #name(&self) -> Option<&#archived_ty> {
                    if let Some(archived_ev) = ::protoss::Probe::probe_as::<#required_ev_name>(self) {
                        Some(&archived_ev.#name)
                    } else {
                        None
                    }
                }
            }
        });

        quote! {
            impl ::protoss::Pointee for #probe_name {
                type Metadata = ::protoss::ProbeMetadata;
            }

            unsafe impl ::protoss::Probe for #probe_name {
                type Base = #base_name;

                #[inline(always)]
                unsafe fn as_version_unchecked<EV: ::protoss::Evolution<Base = #base_name>>(&self) -> &<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived {
                    &*self.data.as_ptr().cast::<<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived>()
                }

                fn probe_as<EV: ::protoss::Evolution<Base = #base_name>>(&self) -> Option<&<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived> {
                    let data_size = ::core::mem::size_of_val(&self.data);
                    let version_size = ::core::mem::size_of::<<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived>();
                    if version_size <= data_size {
                        Some(unsafe { self.as_version_unchecked::<EV>() })
                    } else {
                        None
                    }
                }

                fn version(&self) -> Option<::protoss::Version> {
                    match ::core::mem::size_of_val(&self.data) as ::protoss::ProbeMetadata {
                        #(#evolution_map,)*
                        _ => None,
                    }
                }
            }

            impl #probe_name {
                #(#field_accessors)*
            }
        }
    };

    let impl_evolving = {
        let evolution_map = evolutions.iter().map(|desc| {
            let evolution_name = ev_struct_name(base_name, desc.ev);
            quote! { <#evolution_name as ::protoss::Evolution>::VERSION => Ok(<#evolution_name as ::protoss::Evolution>::METADATA) }
        });

        let latest_ev_name = ev_struct_name(base_name, evolutions.last().unwrap().ev);

        quote! {
            unsafe impl #impl_generics ::protoss::Evolving for #base_name #ty_generics #where_clause {
                type Probe = #probe_name;
                type LatestEvolution = #latest_ev_name;
                fn probe_metadata(version: ::protoss::Version) -> Result<::protoss::ProbeMetadata, ::protoss::Error> {
                    match version {
                        #(#evolution_map,)*
                        _ => Err(::protoss::Error::TriedToGetProbeMetadataForNonExistentVersion)
                    }
                }
            }
        }
    };

    Ok(quote! {
        #[derive(::protoss::rkyv_impl::rkyv::Archive, ::protoss::rkyv_impl::rkyv::Serialize)]
        #[archive(as = "<<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived")]
        #(#base_attrs)*
        #vis struct #base_name #generics {
            #(#stripped_fields,)*
        }

        #(#evolution_structs)*

        #(#archived_evolution_structs)*

        #(#impl_evolutions)*

        #impl_evolving

        #[repr(transparent)]
        pub struct #probe_name {
            data: [u8]
        }

        #impl_probe
    })
}
