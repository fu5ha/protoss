use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Error, Ident, ItemStruct, Meta, parse::Parse, Token, spanned::Spanned};

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
}

impl FieldDescriptor {
    fn try_from_field(field: &syn::Field) -> syn::Result<Self> {
        let field_attr = field
            .attrs
            .get(0)
            .filter(|attr| attr.path.is_ident("field"))
            .ok_or_else(|| syn::Error::new_spanned(field, "all fields must be annotated with a single `#[field(...)]` attribute"))?;

        let FieldAttrs { id, since_ev } = field_attr.parse_args()?;

        let ident = field.ident.clone().ok_or_else(|| syn::Error::new_spanned(field, "only named fields are supported"))?;

        Ok(Self {
            attr_span: field_attr.span(),
            id,
            since_ev,
            ident,
            ty: field.ty.clone(),
        })
    }

    fn rendered(&self) -> RenderedFieldDescriptor {
        RenderedFieldDescriptor::DataField {
            ident: self.ident.clone(),
            ty: self.ty.clone(),
        }
    }
}

#[derive(Clone)]
enum RenderedFieldDescriptor {
    DataField {
        ident: syn::Ident,
        ty: syn::Type,
    },
    EvolutionSeparatorField {
        ev: u16,
        types: Vec<syn::Type>
    },
}

struct EvolutionDescriptor {
    ev: u16,
    fields: Vec<FieldDescriptor>,
    rendered_fields: Vec<RenderedFieldDescriptor>,
}

fn collect_evolutions(fields: &syn::Fields) -> Result<(Vec<EvolutionDescriptor>, Vec<syn::Field>), Error> {
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
                stripped.attrs = field.attrs.iter().filter(|attr| !attr.path.is_ident("field")).cloned().collect();
                stripped_fields.push(stripped);
            }
        },
        _ => return Err(Error::new_spanned(fields, "protoss may only be used on structs with named fields")),
    };

    field_descriptors.sort_by_key(|desc| desc.id);

    let mut evolution_descriptors: Vec<_> = (0..latest_evolution + 1).into_iter().map(|ev| EvolutionDescriptor { ev, fields: Vec::new(), rendered_fields: Vec::new() }).collect();

    fn push_rendered(evolution_descriptors: &mut [EvolutionDescriptor], ev: u16, rendered_desc: &RenderedFieldDescriptor) {
        for desc in &mut evolution_descriptors[(ev as usize)..] {
            desc.rendered_fields.push(rendered_desc.clone());
        }
    }

    fn push_raw(evolution_descriptors: &mut [EvolutionDescriptor], ev: u16, field: &FieldDescriptor) {
        for desc in &mut evolution_descriptors[(ev as usize)..] {
            desc.fields.push(field.clone());
        }
    }

    let mut current_ev = 0;
    for (i, field) in field_descriptors.iter().enumerate() {
        if field.since_ev > current_ev {
            let separator = RenderedFieldDescriptor::EvolutionSeparatorField {
                ev: current_ev,
                types: field_descriptors[0..i].iter().map(|desc| desc.ty.clone()).collect(),
            };
            push_rendered(&mut evolution_descriptors, current_ev, &separator);

            current_ev = field.since_ev;
        } else if field.since_ev < current_ev {
            return Err(Error::new(field.attr_span, "the field with previous id has a higher initial evolution; fields cannot be added to previous evolutions"));
        }

        push_rendered(&mut evolution_descriptors, current_ev, &field.rendered());
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

    let attrs = &input.attrs;

    let probe_name = Ident::new(&format!("{}Probe", base_name), base_name.span());

    let (evolutions, stripped_fields) = collect_evolutions(&input.fields)?;

    let evolution_structs = evolutions.iter().map(|desc| {
        let struct_name = ev_struct_name(base_name, desc.ev);
        let archived_name = archived_ev_struct_name(base_name, desc.ev);
        let archived_name_string = format!("{}", archived_name);
        let (field_idents, field_types): (Vec<syn::Ident>, Vec<syn::Type>) = desc.rendered_fields.iter().filter_map(|f| {
            match f {
                RenderedFieldDescriptor::DataField { ident, ty } => Some((ident.clone(), ty.clone())),
                RenderedFieldDescriptor::EvolutionSeparatorField { .. } => None,
            }
        }).unzip();

        quote! {
            #(#attrs)*
            #[derive(::protoss::rkyv_impl::rkyv::Archive, ::protoss::rkyv_impl::rkyv::Serialize, ::protoss::rkyv_impl::rkyv::Deserialize)]
            #[archive(as = #archived_name_string)]
            #vis struct #struct_name #generics {
                #(pub #field_idents: #field_types,)*
            }
        }
    });

    let archived_evolution_structs = evolutions.iter().map(|desc| {
        let archived_name = archived_ev_struct_name(base_name, desc.ev);
        let fields = desc.rendered_fields.iter().map(|f| {
            match f {
                RenderedFieldDescriptor::DataField { ident, ty } => quote!(pub #ident: #ty),
                RenderedFieldDescriptor::EvolutionSeparatorField { ev, types } => {
                    let ident = ev_pad_field_name(*ev);

                    quote!(#ident: ::protoss::rkyv_impl::PadToAlign<(#(#types,)*)>)
                },
            }
        });

        let field_args = desc.fields.iter().map(|field| {
            let ident = &field.ident;
            let ty = &field.ty;
            quote!(#ident: #ty)
        });

        let field_constructors = desc.rendered_fields.iter().map(|field| {
            match field {
                RenderedFieldDescriptor::DataField { ident, .. } => quote!(#ident),
                RenderedFieldDescriptor::EvolutionSeparatorField { ev, ..} => {
                    let ident = ev_pad_field_name(*ev);

                    quote!(#ident: Default::default())
                },
            }
        });

        quote! {
            #[repr(C)]
            #[derive(Debug, PartialEq)]
            #vis struct #archived_name #generics {
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
            let ty = &field.ty;
            let required_ev_name = ev_struct_name(base_name, field.since_ev);
            quote! {
                pub fn #name(&self) -> Option<&#ty> {
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
