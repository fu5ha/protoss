#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
mod derive {
    mod v1 {
        #[archive(
            as = "<<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived"
        )]
        pub struct Test {
            pub b: u8,
            pub c: u32,
            pub a: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`Test`]
        pub struct TestResolver
        where
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
            a: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for Test
            where
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = <<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived;
                type Resolver = TestResolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for Test
            where
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestResolver {
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv0")]
        pub struct TestEv0 {
            pub a: u32,
            pub b: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv0`]
        pub struct TestEv0Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv0
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv0;
                type Resolver = TestEv0Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv0
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv0Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv0, __D> for Archived<TestEv0>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv0, __D::Error> {
                    Ok(TestEv0 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv1")]
        pub struct TestEv1 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv1`]
        pub struct TestEv1Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv1
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv1;
                type Resolver = TestEv1Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv1
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv1Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv1, __D> for Archived<TestEv1>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv1, __D::Error> {
                    Ok(TestEv1 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                        c: Deserialize::<u32, __D>::deserialize(&self.c, deserializer)?,
                    })
                }
            }
        };
        #[repr(C)]
        pub struct ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ArchivedTestEv0",
                    "a",
                    &&self.a,
                    "b",
                    &&self.b,
                    "_pad_ev0",
                    &&self._pad_ev0,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv0) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
            }
        }
        impl ArchivedTestEv0 {
            pub fn new(a: ::rkyv::Archived<u32>, b: ::rkyv::Archived<u8>) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
            pub c: ::rkyv::Archived<u32>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "ArchivedTestEv1",
                    "a",
                    &&self.a,
                    "b",
                    &&self.b,
                    "_pad_ev0",
                    &&self._pad_ev0,
                    "c",
                    &&self.c,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv1) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
                    && self.c == other.c
            }
        }
        impl ArchivedTestEv1 {
            pub fn new(
                a: ::rkyv::Archived<u32>,
                b: ::rkyv::Archived<u8>,
                c: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                    c,
                }
            }
        }
        unsafe impl ::protoss::Evolution for TestEv0 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(0u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestEv1 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(1u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolving for Test {
            type Probe = TestProbe;
            type LatestEvolution = TestEv1;
            fn probe_metadata(
                version: ::protoss::Version,
            ) -> Result<::protoss::ProbeMetadata, ::protoss::Error> {
                match version {
                    <TestEv0 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv0 as ::protoss::Evolution>::METADATA)
                    }
                    <TestEv1 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv1 as ::protoss::Evolution>::METADATA)
                    }
                    _ => {
                        Err(
                            ::protoss::Error::TriedToGetProbeMetadataForNonExistentVersion,
                        )
                    }
                }
            }
        }
        #[repr(transparent)]
        pub struct TestProbe {
            data: [u8],
        }
        impl ::protoss::Pointee for TestProbe {
            type Metadata = ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Probe for TestProbe {
            type Base = Test;
            #[inline(always)]
            unsafe fn as_version_unchecked<EV: ::protoss::Evolution<Base = Test>>(
                &self,
            ) -> &<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived {
                &*self
                    .data
                    .as_ptr()
                    .cast::<<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived>()
            }
            fn probe_as<EV: ::protoss::Evolution<Base = Test>>(
                &self,
            ) -> Option<&<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived> {
                let data_size = ::core::mem::size_of_val(&self.data);
                let version_size = ::core::mem::size_of::<
                    <EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
                >();
                if version_size <= data_size {
                    Some(unsafe { self.as_version_unchecked::<EV>() })
                } else {
                    None
                }
            }
            fn version(&self) -> Option<::protoss::Version> {
                match ::core::mem::size_of_val(&self.data) as ::protoss::ProbeMetadata {
                    <TestEv0 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv0 as ::protoss::Evolution>::VERSION)
                    }
                    <TestEv1 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv1 as ::protoss::Evolution>::VERSION)
                    }
                    _ => None,
                }
            }
        }
        impl TestProbe {
            pub fn a(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv0>(self) {
                    Some(&archived_ev.a)
                } else {
                    None
                }
            }
            pub fn b(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv0>(self) {
                    Some(&archived_ev.b)
                } else {
                    None
                }
            }
            pub fn c(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv1>(self) {
                    Some(&archived_ev.c)
                } else {
                    None
                }
            }
        }
        #[archive(
            as = "<<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived"
        )]
        pub struct TestParent {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParent`]
        pub struct TestParentResolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParent
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = <<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived;
                type Resolver = TestParentResolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParent
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentResolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestParentEv0")]
        pub struct TestParentEv0 {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParentEv0`]
        pub struct TestParentEv0Resolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParentEv0
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestParentEv0;
                type Resolver = TestParentEv0Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParentEv0
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentEv0Resolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestParentEv0, __D>
            for Archived<TestParentEv0>
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Archive,
                Archived<
                    ::rkyv::with::With<Test, protoss::Evolve>,
                >: Deserialize<::rkyv::with::With<Test, protoss::Evolve>, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestParentEv0, __D::Error> {
                    Ok(TestParentEv0 {
                        test: Deserialize::<
                            ::rkyv::with::With<Test, protoss::Evolve>,
                            __D,
                        >::deserialize(&self.test, deserializer)?
                            .into_inner(),
                        parent_a: Deserialize::<
                            u8,
                            __D,
                        >::deserialize(&self.parent_a, deserializer)?,
                    })
                }
            }
        };
        #[repr(C)]
        pub struct ArchivedTestParentEv0
        where
            Test: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
            pub parent_a: ::rkyv::Archived<u8>,
        }
        impl ArchivedTestParentEv0 {
            pub fn new(
                test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                parent_a: ::rkyv::Archived<u8>,
            ) -> Self {
                Self { test, parent_a }
            }
        }
        unsafe impl ::protoss::Evolution for TestParentEv0 {
            type Base = TestParent;
            const VERSION: ::protoss::Version = ::protoss::Version::new(0u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolving for TestParent {
            type Probe = TestParentProbe;
            type LatestEvolution = TestParentEv0;
            fn probe_metadata(
                version: ::protoss::Version,
            ) -> Result<::protoss::ProbeMetadata, ::protoss::Error> {
                match version {
                    <TestParentEv0 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestParentEv0 as ::protoss::Evolution>::METADATA)
                    }
                    _ => {
                        Err(
                            ::protoss::Error::TriedToGetProbeMetadataForNonExistentVersion,
                        )
                    }
                }
            }
        }
        #[repr(transparent)]
        pub struct TestParentProbe {
            data: [u8],
        }
        impl ::protoss::Pointee for TestParentProbe {
            type Metadata = ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Probe for TestParentProbe {
            type Base = TestParent;
            #[inline(always)]
            unsafe fn as_version_unchecked<EV: ::protoss::Evolution<Base = TestParent>>(
                &self,
            ) -> &<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived {
                &*self
                    .data
                    .as_ptr()
                    .cast::<<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived>()
            }
            fn probe_as<EV: ::protoss::Evolution<Base = TestParent>>(
                &self,
            ) -> Option<&<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived> {
                let data_size = ::core::mem::size_of_val(&self.data);
                let version_size = ::core::mem::size_of::<
                    <EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
                >();
                if version_size <= data_size {
                    Some(unsafe { self.as_version_unchecked::<EV>() })
                } else {
                    None
                }
            }
            fn version(&self) -> Option<::protoss::Version> {
                match ::core::mem::size_of_val(&self.data) as ::protoss::ProbeMetadata {
                    <TestParentEv0 as ::protoss::Evolution>::METADATA => {
                        Some(<TestParentEv0 as ::protoss::Evolution>::VERSION)
                    }
                    _ => None,
                }
            }
        }
        impl TestParentProbe {
            pub fn test(
                &self,
            ) -> Option<&::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv0>(self) {
                    Some(&archived_ev.test)
                } else {
                    None
                }
            }
            pub fn parent_a(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv0>(self) {
                    Some(&archived_ev.parent_a)
                } else {
                    None
                }
            }
        }
    }
    mod v2 {
        #[archive(
            as = "<<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived"
        )]
        pub struct Test {
            pub a: u32,
            pub b: u8,
            pub c: u32,
            pub d: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`Test`]
        pub struct TestResolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
            d: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for Test
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = <<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived;
                type Resolver = TestResolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.d), pos + fp, resolver.d, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for Test
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestResolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                        d: Serialize::<__S>::serialize(&self.d, serializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv0")]
        pub struct TestEv0 {
            pub a: u32,
            pub b: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv0`]
        pub struct TestEv0Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv0
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv0;
                type Resolver = TestEv0Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv0
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv0Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv0, __D> for Archived<TestEv0>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv0, __D::Error> {
                    Ok(TestEv0 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv1")]
        pub struct TestEv1 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv1`]
        pub struct TestEv1Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv1
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv1;
                type Resolver = TestEv1Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv1
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv1Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv1, __D> for Archived<TestEv1>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv1, __D::Error> {
                    Ok(TestEv1 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                        c: Deserialize::<u32, __D>::deserialize(&self.c, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv2")]
        pub struct TestEv2 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
            pub d: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv2`]
        pub struct TestEv2Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
            d: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv2
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv2;
                type Resolver = TestEv2Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.d), pos + fp, resolver.d, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv2
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv2Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                        d: Serialize::<__S>::serialize(&self.d, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv2, __D> for Archived<TestEv2>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv2, __D::Error> {
                    Ok(TestEv2 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                        c: Deserialize::<u32, __D>::deserialize(&self.c, deserializer)?,
                        d: Deserialize::<u8, __D>::deserialize(&self.d, deserializer)?,
                    })
                }
            }
        };
        #[repr(C)]
        pub struct ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ArchivedTestEv0",
                    "a",
                    &&self.a,
                    "b",
                    &&self.b,
                    "_pad_ev0",
                    &&self._pad_ev0,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv0) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
            }
        }
        impl ArchivedTestEv0 {
            pub fn new(a: ::rkyv::Archived<u32>, b: ::rkyv::Archived<u8>) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
            pub c: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>, ::rkyv::Archived<u32>),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "ArchivedTestEv1",
                    "a",
                    &&self.a,
                    "b",
                    &&self.b,
                    "_pad_ev0",
                    &&self._pad_ev0,
                    "c",
                    &&self.c,
                    "_pad_ev1",
                    &&self._pad_ev1,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv1) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
                    && self.c == other.c && self._pad_ev1 == other._pad_ev1
            }
        }
        impl ArchivedTestEv1 {
            pub fn new(
                a: ::rkyv::Archived<u32>,
                b: ::rkyv::Archived<u8>,
                c: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                    c,
                    _pad_ev1: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
            pub c: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>, ::rkyv::Archived<u32>),
            >,
            pub d: ::rkyv::Archived<u8>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &["a", "b", "_pad_ev0", "c", "_pad_ev1", "d"];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.a,
                    &&self.b,
                    &&self._pad_ev0,
                    &&self.c,
                    &&self._pad_ev1,
                    &&self.d,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ArchivedTestEv2",
                    names,
                    values,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv2) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
                    && self.c == other.c && self._pad_ev1 == other._pad_ev1
                    && self.d == other.d
            }
        }
        impl ArchivedTestEv2 {
            pub fn new(
                a: ::rkyv::Archived<u32>,
                b: ::rkyv::Archived<u8>,
                c: ::rkyv::Archived<u32>,
                d: ::rkyv::Archived<u8>,
            ) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                    c,
                    _pad_ev1: Default::default(),
                    d,
                }
            }
        }
        unsafe impl ::protoss::Evolution for TestEv0 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(0u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestEv1 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(1u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestEv2 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(2u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolving for Test {
            type Probe = TestProbe;
            type LatestEvolution = TestEv2;
            fn probe_metadata(
                version: ::protoss::Version,
            ) -> Result<::protoss::ProbeMetadata, ::protoss::Error> {
                match version {
                    <TestEv0 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv0 as ::protoss::Evolution>::METADATA)
                    }
                    <TestEv1 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv1 as ::protoss::Evolution>::METADATA)
                    }
                    <TestEv2 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv2 as ::protoss::Evolution>::METADATA)
                    }
                    _ => {
                        Err(
                            ::protoss::Error::TriedToGetProbeMetadataForNonExistentVersion,
                        )
                    }
                }
            }
        }
        #[repr(transparent)]
        pub struct TestProbe {
            data: [u8],
        }
        impl ::protoss::Pointee for TestProbe {
            type Metadata = ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Probe for TestProbe {
            type Base = Test;
            #[inline(always)]
            unsafe fn as_version_unchecked<EV: ::protoss::Evolution<Base = Test>>(
                &self,
            ) -> &<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived {
                &*self
                    .data
                    .as_ptr()
                    .cast::<<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived>()
            }
            fn probe_as<EV: ::protoss::Evolution<Base = Test>>(
                &self,
            ) -> Option<&<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived> {
                let data_size = ::core::mem::size_of_val(&self.data);
                let version_size = ::core::mem::size_of::<
                    <EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
                >();
                if version_size <= data_size {
                    Some(unsafe { self.as_version_unchecked::<EV>() })
                } else {
                    None
                }
            }
            fn version(&self) -> Option<::protoss::Version> {
                match ::core::mem::size_of_val(&self.data) as ::protoss::ProbeMetadata {
                    <TestEv0 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv0 as ::protoss::Evolution>::VERSION)
                    }
                    <TestEv1 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv1 as ::protoss::Evolution>::VERSION)
                    }
                    <TestEv2 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv2 as ::protoss::Evolution>::VERSION)
                    }
                    _ => None,
                }
            }
        }
        impl TestProbe {
            pub fn a(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv0>(self) {
                    Some(&archived_ev.a)
                } else {
                    None
                }
            }
            pub fn b(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv0>(self) {
                    Some(&archived_ev.b)
                } else {
                    None
                }
            }
            pub fn c(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv1>(self) {
                    Some(&archived_ev.c)
                } else {
                    None
                }
            }
            pub fn d(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv2>(self) {
                    Some(&archived_ev.d)
                } else {
                    None
                }
            }
        }
        #[archive(
            as = "<<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived"
        )]
        pub struct TestParent {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
            pub parent_b: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParent`]
        pub struct TestParentResolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
            parent_b: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParent
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = <<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived;
                type Resolver = TestParentResolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_b),
                        pos + fp,
                        resolver.parent_b,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParent
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentResolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                        parent_b: Serialize::<
                            __S,
                        >::serialize(&self.parent_b, serializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestParentEv0")]
        pub struct TestParentEv0 {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParentEv0`]
        pub struct TestParentEv0Resolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParentEv0
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestParentEv0;
                type Resolver = TestParentEv0Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParentEv0
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentEv0Resolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestParentEv0, __D>
            for Archived<TestParentEv0>
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Archive,
                Archived<
                    ::rkyv::with::With<Test, protoss::Evolve>,
                >: Deserialize<::rkyv::with::With<Test, protoss::Evolve>, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestParentEv0, __D::Error> {
                    Ok(TestParentEv0 {
                        test: Deserialize::<
                            ::rkyv::with::With<Test, protoss::Evolve>,
                            __D,
                        >::deserialize(&self.test, deserializer)?
                            .into_inner(),
                        parent_a: Deserialize::<
                            u8,
                            __D,
                        >::deserialize(&self.parent_a, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestParentEv1")]
        pub struct TestParentEv1 {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
            pub parent_b: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParentEv1`]
        pub struct TestParentEv1Resolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
            parent_b: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParentEv1
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestParentEv1;
                type Resolver = TestParentEv1Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_b),
                        pos + fp,
                        resolver.parent_b,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParentEv1
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentEv1Resolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                        parent_b: Serialize::<
                            __S,
                        >::serialize(&self.parent_b, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestParentEv1, __D>
            for Archived<TestParentEv1>
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Archive,
                Archived<
                    ::rkyv::with::With<Test, protoss::Evolve>,
                >: Deserialize<::rkyv::with::With<Test, protoss::Evolve>, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestParentEv1, __D::Error> {
                    Ok(TestParentEv1 {
                        test: Deserialize::<
                            ::rkyv::with::With<Test, protoss::Evolve>,
                            __D,
                        >::deserialize(&self.test, deserializer)?
                            .into_inner(),
                        parent_a: Deserialize::<
                            u8,
                            __D,
                        >::deserialize(&self.parent_a, deserializer)?,
                        parent_b: Deserialize::<
                            u32,
                            __D,
                        >::deserialize(&self.parent_b, deserializer)?,
                    })
                }
            }
        };
        #[repr(C)]
        pub struct ArchivedTestParentEv0
        where
            Test: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
            pub parent_a: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                ),
            >,
        }
        impl ArchivedTestParentEv0 {
            pub fn new(
                test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                parent_a: ::rkyv::Archived<u8>,
            ) -> Self {
                Self {
                    test,
                    parent_a,
                    _pad_ev0: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestParentEv1
        where
            Test: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
            pub parent_a: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                ),
            >,
            pub parent_b: ::rkyv::Archived<u32>,
        }
        impl ArchivedTestParentEv1 {
            pub fn new(
                test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                parent_a: ::rkyv::Archived<u8>,
                parent_b: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    test,
                    parent_a,
                    _pad_ev0: Default::default(),
                    parent_b,
                }
            }
        }
        unsafe impl ::protoss::Evolution for TestParentEv0 {
            type Base = TestParent;
            const VERSION: ::protoss::Version = ::protoss::Version::new(0u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestParentEv1 {
            type Base = TestParent;
            const VERSION: ::protoss::Version = ::protoss::Version::new(1u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolving for TestParent {
            type Probe = TestParentProbe;
            type LatestEvolution = TestParentEv1;
            fn probe_metadata(
                version: ::protoss::Version,
            ) -> Result<::protoss::ProbeMetadata, ::protoss::Error> {
                match version {
                    <TestParentEv0 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestParentEv0 as ::protoss::Evolution>::METADATA)
                    }
                    <TestParentEv1 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestParentEv1 as ::protoss::Evolution>::METADATA)
                    }
                    _ => {
                        Err(
                            ::protoss::Error::TriedToGetProbeMetadataForNonExistentVersion,
                        )
                    }
                }
            }
        }
        #[repr(transparent)]
        pub struct TestParentProbe {
            data: [u8],
        }
        impl ::protoss::Pointee for TestParentProbe {
            type Metadata = ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Probe for TestParentProbe {
            type Base = TestParent;
            #[inline(always)]
            unsafe fn as_version_unchecked<EV: ::protoss::Evolution<Base = TestParent>>(
                &self,
            ) -> &<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived {
                &*self
                    .data
                    .as_ptr()
                    .cast::<<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived>()
            }
            fn probe_as<EV: ::protoss::Evolution<Base = TestParent>>(
                &self,
            ) -> Option<&<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived> {
                let data_size = ::core::mem::size_of_val(&self.data);
                let version_size = ::core::mem::size_of::<
                    <EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
                >();
                if version_size <= data_size {
                    Some(unsafe { self.as_version_unchecked::<EV>() })
                } else {
                    None
                }
            }
            fn version(&self) -> Option<::protoss::Version> {
                match ::core::mem::size_of_val(&self.data) as ::protoss::ProbeMetadata {
                    <TestParentEv0 as ::protoss::Evolution>::METADATA => {
                        Some(<TestParentEv0 as ::protoss::Evolution>::VERSION)
                    }
                    <TestParentEv1 as ::protoss::Evolution>::METADATA => {
                        Some(<TestParentEv1 as ::protoss::Evolution>::VERSION)
                    }
                    _ => None,
                }
            }
        }
        impl TestParentProbe {
            pub fn test(
                &self,
            ) -> Option<&::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv0>(self) {
                    Some(&archived_ev.test)
                } else {
                    None
                }
            }
            pub fn parent_a(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv0>(self) {
                    Some(&archived_ev.parent_a)
                } else {
                    None
                }
            }
            pub fn parent_b(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv1>(self) {
                    Some(&archived_ev.parent_b)
                } else {
                    None
                }
            }
        }
    }
    mod v3 {
        #[archive(
            as = "<<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived"
        )]
        pub struct Test {
            pub a: u32,
            pub b: u8,
            pub c: u32,
            pub d: u8,
            pub e: String,
            pub f: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`Test`]
        pub struct TestResolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
            d: ::rkyv::Resolver<u8>,
            e: ::rkyv::Resolver<String>,
            f: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for Test
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                String: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = <<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived;
                type Resolver = TestResolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.d), pos + fp, resolver.d, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).e;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.e), pos + fp, resolver.e, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).f;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.f), pos + fp, resolver.f, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for Test
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                String: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestResolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                        d: Serialize::<__S>::serialize(&self.d, serializer)?,
                        e: Serialize::<__S>::serialize(&self.e, serializer)?,
                        f: Serialize::<__S>::serialize(&self.f, serializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv0")]
        pub struct TestEv0 {
            pub a: u32,
            pub b: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv0`]
        pub struct TestEv0Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv0
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv0;
                type Resolver = TestEv0Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv0
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv0Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv0, __D> for Archived<TestEv0>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv0, __D::Error> {
                    Ok(TestEv0 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv1")]
        pub struct TestEv1 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv1`]
        pub struct TestEv1Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv1
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv1;
                type Resolver = TestEv1Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv1
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv1Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv1, __D> for Archived<TestEv1>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv1, __D::Error> {
                    Ok(TestEv1 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                        c: Deserialize::<u32, __D>::deserialize(&self.c, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv2")]
        pub struct TestEv2 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
            pub d: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv2`]
        pub struct TestEv2Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
            d: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv2
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv2;
                type Resolver = TestEv2Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.d), pos + fp, resolver.d, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv2
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv2Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                        d: Serialize::<__S>::serialize(&self.d, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv2, __D> for Archived<TestEv2>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv2, __D::Error> {
                    Ok(TestEv2 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                        c: Deserialize::<u32, __D>::deserialize(&self.c, deserializer)?,
                        d: Deserialize::<u8, __D>::deserialize(&self.d, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv3")]
        pub struct TestEv3 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
            pub d: u8,
            pub e: String,
            pub f: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv3`]
        pub struct TestEv3Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
            d: ::rkyv::Resolver<u8>,
            e: ::rkyv::Resolver<String>,
            f: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv3
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                String: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv3;
                type Resolver = TestEv3Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.d), pos + fp, resolver.d, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).e;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.e), pos + fp, resolver.e, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).f;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.f), pos + fp, resolver.f, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv3
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                String: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv3Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                        d: Serialize::<__S>::serialize(&self.d, serializer)?,
                        e: Serialize::<__S>::serialize(&self.e, serializer)?,
                        f: Serialize::<__S>::serialize(&self.f, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv3, __D> for Archived<TestEv3>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                String: Archive,
                Archived<String>: Deserialize<String, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv3, __D::Error> {
                    Ok(TestEv3 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                        c: Deserialize::<u32, __D>::deserialize(&self.c, deserializer)?,
                        d: Deserialize::<u8, __D>::deserialize(&self.d, deserializer)?,
                        e: Deserialize::<
                            String,
                            __D,
                        >::deserialize(&self.e, deserializer)?,
                        f: Deserialize::<u32, __D>::deserialize(&self.f, deserializer)?,
                    })
                }
            }
        };
        #[repr(C)]
        pub struct ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ArchivedTestEv0",
                    "a",
                    &&self.a,
                    "b",
                    &&self.b,
                    "_pad_ev0",
                    &&self._pad_ev0,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv0) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
            }
        }
        impl ArchivedTestEv0 {
            pub fn new(a: ::rkyv::Archived<u32>, b: ::rkyv::Archived<u8>) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
            pub c: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>, ::rkyv::Archived<u32>),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "ArchivedTestEv1",
                    "a",
                    &&self.a,
                    "b",
                    &&self.b,
                    "_pad_ev0",
                    &&self._pad_ev0,
                    "c",
                    &&self.c,
                    "_pad_ev1",
                    &&self._pad_ev1,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv1) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
                    && self.c == other.c && self._pad_ev1 == other._pad_ev1
            }
        }
        impl ArchivedTestEv1 {
            pub fn new(
                a: ::rkyv::Archived<u32>,
                b: ::rkyv::Archived<u8>,
                c: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                    c,
                    _pad_ev1: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
            pub c: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>, ::rkyv::Archived<u32>),
            >,
            pub d: ::rkyv::Archived<u8>,
            _pad_ev2: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                ),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "a",
                    "b",
                    "_pad_ev0",
                    "c",
                    "_pad_ev1",
                    "d",
                    "_pad_ev2",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.a,
                    &&self.b,
                    &&self._pad_ev0,
                    &&self.c,
                    &&self._pad_ev1,
                    &&self.d,
                    &&self._pad_ev2,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ArchivedTestEv2",
                    names,
                    values,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv2) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
                    && self.c == other.c && self._pad_ev1 == other._pad_ev1
                    && self.d == other.d && self._pad_ev2 == other._pad_ev2
            }
        }
        impl ArchivedTestEv2 {
            pub fn new(
                a: ::rkyv::Archived<u32>,
                b: ::rkyv::Archived<u8>,
                c: ::rkyv::Archived<u32>,
                d: ::rkyv::Archived<u8>,
            ) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                    c,
                    _pad_ev1: Default::default(),
                    d,
                    _pad_ev2: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestEv3
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
            pub c: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>, ::rkyv::Archived<u32>),
            >,
            pub d: ::rkyv::Archived<u8>,
            _pad_ev2: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                ),
            >,
            pub e: ::rkyv::Archived<String>,
            pub f: ::rkyv::Archived<u32>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv3
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "a",
                    "b",
                    "_pad_ev0",
                    "c",
                    "_pad_ev1",
                    "d",
                    "_pad_ev2",
                    "e",
                    "f",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.a,
                    &&self.b,
                    &&self._pad_ev0,
                    &&self.c,
                    &&self._pad_ev1,
                    &&self.d,
                    &&self._pad_ev2,
                    &&self.e,
                    &&self.f,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ArchivedTestEv3",
                    names,
                    values,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv3
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv3
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv3) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
                    && self.c == other.c && self._pad_ev1 == other._pad_ev1
                    && self.d == other.d && self._pad_ev2 == other._pad_ev2
                    && self.e == other.e && self.f == other.f
            }
        }
        impl ArchivedTestEv3 {
            pub fn new(
                a: ::rkyv::Archived<u32>,
                b: ::rkyv::Archived<u8>,
                c: ::rkyv::Archived<u32>,
                d: ::rkyv::Archived<u8>,
                e: ::rkyv::Archived<String>,
                f: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                    c,
                    _pad_ev1: Default::default(),
                    d,
                    _pad_ev2: Default::default(),
                    e,
                    f,
                }
            }
        }
        unsafe impl ::protoss::Evolution for TestEv0 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(0u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestEv1 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(1u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestEv2 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(2u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestEv3 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(3u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolving for Test {
            type Probe = TestProbe;
            type LatestEvolution = TestEv3;
            fn probe_metadata(
                version: ::protoss::Version,
            ) -> Result<::protoss::ProbeMetadata, ::protoss::Error> {
                match version {
                    <TestEv0 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv0 as ::protoss::Evolution>::METADATA)
                    }
                    <TestEv1 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv1 as ::protoss::Evolution>::METADATA)
                    }
                    <TestEv2 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv2 as ::protoss::Evolution>::METADATA)
                    }
                    <TestEv3 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv3 as ::protoss::Evolution>::METADATA)
                    }
                    _ => {
                        Err(
                            ::protoss::Error::TriedToGetProbeMetadataForNonExistentVersion,
                        )
                    }
                }
            }
        }
        #[repr(transparent)]
        pub struct TestProbe {
            data: [u8],
        }
        impl ::protoss::Pointee for TestProbe {
            type Metadata = ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Probe for TestProbe {
            type Base = Test;
            #[inline(always)]
            unsafe fn as_version_unchecked<EV: ::protoss::Evolution<Base = Test>>(
                &self,
            ) -> &<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived {
                &*self
                    .data
                    .as_ptr()
                    .cast::<<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived>()
            }
            fn probe_as<EV: ::protoss::Evolution<Base = Test>>(
                &self,
            ) -> Option<&<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived> {
                let data_size = ::core::mem::size_of_val(&self.data);
                let version_size = ::core::mem::size_of::<
                    <EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
                >();
                if version_size <= data_size {
                    Some(unsafe { self.as_version_unchecked::<EV>() })
                } else {
                    None
                }
            }
            fn version(&self) -> Option<::protoss::Version> {
                match ::core::mem::size_of_val(&self.data) as ::protoss::ProbeMetadata {
                    <TestEv0 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv0 as ::protoss::Evolution>::VERSION)
                    }
                    <TestEv1 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv1 as ::protoss::Evolution>::VERSION)
                    }
                    <TestEv2 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv2 as ::protoss::Evolution>::VERSION)
                    }
                    <TestEv3 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv3 as ::protoss::Evolution>::VERSION)
                    }
                    _ => None,
                }
            }
        }
        impl TestProbe {
            pub fn a(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv0>(self) {
                    Some(&archived_ev.a)
                } else {
                    None
                }
            }
            pub fn b(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv0>(self) {
                    Some(&archived_ev.b)
                } else {
                    None
                }
            }
            pub fn c(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv1>(self) {
                    Some(&archived_ev.c)
                } else {
                    None
                }
            }
            pub fn d(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv2>(self) {
                    Some(&archived_ev.d)
                } else {
                    None
                }
            }
            pub fn e(&self) -> Option<&::rkyv::Archived<String>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv3>(self) {
                    Some(&archived_ev.e)
                } else {
                    None
                }
            }
            pub fn f(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv3>(self) {
                    Some(&archived_ev.f)
                } else {
                    None
                }
            }
        }
        #[archive(
            as = "<<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived"
        )]
        pub struct TestParent {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
            pub parent_b: u32,
            #[with(rkyv::with::CopyOptimize)]
            pub parent_c: Vec<u32>,
            pub parent_d: String,
            pub parent_e: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParent`]
        pub struct TestParentResolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
            parent_b: ::rkyv::Resolver<u32>,
            parent_c: ::rkyv::Resolver<
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
            >,
            parent_d: ::rkyv::Resolver<String>,
            parent_e: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParent
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: ::rkyv::Archive,
                String: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = <<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived;
                type Resolver = TestParentResolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_b),
                        pos + fp,
                        resolver.parent_b,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<
                            _,
                            rkyv::with::CopyOptimize,
                        >::cast((&self.parent_c)),
                        pos + fp,
                        resolver.parent_c,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_d),
                        pos + fp,
                        resolver.parent_d,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_e;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_e),
                        pos + fp,
                        resolver.parent_e,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParent
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: Serialize<__S>,
                String: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentResolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                        parent_b: Serialize::<
                            __S,
                        >::serialize(&self.parent_b, serializer)?,
                        parent_c: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<
                                _,
                                rkyv::with::CopyOptimize,
                            >::cast(&self.parent_c),
                            serializer,
                        )?,
                        parent_d: Serialize::<
                            __S,
                        >::serialize(&self.parent_d, serializer)?,
                        parent_e: Serialize::<
                            __S,
                        >::serialize(&self.parent_e, serializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestParentEv0")]
        pub struct TestParentEv0 {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParentEv0`]
        pub struct TestParentEv0Resolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParentEv0
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestParentEv0;
                type Resolver = TestParentEv0Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParentEv0
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentEv0Resolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestParentEv0, __D>
            for Archived<TestParentEv0>
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Archive,
                Archived<
                    ::rkyv::with::With<Test, protoss::Evolve>,
                >: Deserialize<::rkyv::with::With<Test, protoss::Evolve>, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestParentEv0, __D::Error> {
                    Ok(TestParentEv0 {
                        test: Deserialize::<
                            ::rkyv::with::With<Test, protoss::Evolve>,
                            __D,
                        >::deserialize(&self.test, deserializer)?
                            .into_inner(),
                        parent_a: Deserialize::<
                            u8,
                            __D,
                        >::deserialize(&self.parent_a, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestParentEv1")]
        pub struct TestParentEv1 {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
            pub parent_b: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParentEv1`]
        pub struct TestParentEv1Resolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
            parent_b: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParentEv1
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestParentEv1;
                type Resolver = TestParentEv1Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_b),
                        pos + fp,
                        resolver.parent_b,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParentEv1
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentEv1Resolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                        parent_b: Serialize::<
                            __S,
                        >::serialize(&self.parent_b, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestParentEv1, __D>
            for Archived<TestParentEv1>
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Archive,
                Archived<
                    ::rkyv::with::With<Test, protoss::Evolve>,
                >: Deserialize<::rkyv::with::With<Test, protoss::Evolve>, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestParentEv1, __D::Error> {
                    Ok(TestParentEv1 {
                        test: Deserialize::<
                            ::rkyv::with::With<Test, protoss::Evolve>,
                            __D,
                        >::deserialize(&self.test, deserializer)?
                            .into_inner(),
                        parent_a: Deserialize::<
                            u8,
                            __D,
                        >::deserialize(&self.parent_a, deserializer)?,
                        parent_b: Deserialize::<
                            u32,
                            __D,
                        >::deserialize(&self.parent_b, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestParentEv2")]
        pub struct TestParentEv2 {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
            pub parent_b: u32,
            #[with(rkyv::with::CopyOptimize)]
            pub parent_c: Vec<u32>,
            pub parent_d: String,
            pub parent_e: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParentEv2`]
        pub struct TestParentEv2Resolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
            parent_b: ::rkyv::Resolver<u32>,
            parent_c: ::rkyv::Resolver<
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
            >,
            parent_d: ::rkyv::Resolver<String>,
            parent_e: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParentEv2
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: ::rkyv::Archive,
                String: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestParentEv2;
                type Resolver = TestParentEv2Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_b),
                        pos + fp,
                        resolver.parent_b,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<
                            _,
                            rkyv::with::CopyOptimize,
                        >::cast((&self.parent_c)),
                        pos + fp,
                        resolver.parent_c,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_d),
                        pos + fp,
                        resolver.parent_d,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_e;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_e),
                        pos + fp,
                        resolver.parent_e,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParentEv2
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: Serialize<__S>,
                String: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentEv2Resolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                        parent_b: Serialize::<
                            __S,
                        >::serialize(&self.parent_b, serializer)?,
                        parent_c: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<
                                _,
                                rkyv::with::CopyOptimize,
                            >::cast(&self.parent_c),
                            serializer,
                        )?,
                        parent_d: Serialize::<
                            __S,
                        >::serialize(&self.parent_d, serializer)?,
                        parent_e: Serialize::<
                            __S,
                        >::serialize(&self.parent_e, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestParentEv2, __D>
            for Archived<TestParentEv2>
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Archive,
                Archived<
                    ::rkyv::with::With<Test, protoss::Evolve>,
                >: Deserialize<::rkyv::with::With<Test, protoss::Evolve>, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: Archive,
                Archived<
                    ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
                >: Deserialize<
                    ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
                    __D,
                >,
                String: Archive,
                Archived<String>: Deserialize<String, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestParentEv2, __D::Error> {
                    Ok(TestParentEv2 {
                        test: Deserialize::<
                            ::rkyv::with::With<Test, protoss::Evolve>,
                            __D,
                        >::deserialize(&self.test, deserializer)?
                            .into_inner(),
                        parent_a: Deserialize::<
                            u8,
                            __D,
                        >::deserialize(&self.parent_a, deserializer)?,
                        parent_b: Deserialize::<
                            u32,
                            __D,
                        >::deserialize(&self.parent_b, deserializer)?,
                        parent_c: Deserialize::<
                            ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
                            __D,
                        >::deserialize(&self.parent_c, deserializer)?
                            .into_inner(),
                        parent_d: Deserialize::<
                            String,
                            __D,
                        >::deserialize(&self.parent_d, deserializer)?,
                        parent_e: Deserialize::<
                            u32,
                            __D,
                        >::deserialize(&self.parent_e, deserializer)?,
                    })
                }
            }
        };
        #[repr(C)]
        pub struct ArchivedTestParentEv0
        where
            Test: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
            pub parent_a: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                ),
            >,
        }
        impl ArchivedTestParentEv0 {
            pub fn new(
                test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                parent_a: ::rkyv::Archived<u8>,
            ) -> Self {
                Self {
                    test,
                    parent_a,
                    _pad_ev0: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestParentEv1
        where
            Test: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
            pub parent_a: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                ),
            >,
            pub parent_b: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                ),
            >,
        }
        impl ArchivedTestParentEv1 {
            pub fn new(
                test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                parent_a: ::rkyv::Archived<u8>,
                parent_b: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    test,
                    parent_a,
                    _pad_ev0: Default::default(),
                    parent_b,
                    _pad_ev1: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestParentEv2
        where
            Test: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            Vec<u32>: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
            pub parent_a: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                ),
            >,
            pub parent_b: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                ),
            >,
            pub parent_c: ::rkyv::Archived<
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
            >,
            pub parent_d: ::rkyv::Archived<String>,
            pub parent_e: ::rkyv::Archived<u32>,
        }
        impl ArchivedTestParentEv2 {
            pub fn new(
                test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                parent_a: ::rkyv::Archived<u8>,
                parent_b: ::rkyv::Archived<u32>,
                parent_c: ::rkyv::Archived<
                    ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
                >,
                parent_d: ::rkyv::Archived<String>,
                parent_e: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    test,
                    parent_a,
                    _pad_ev0: Default::default(),
                    parent_b,
                    _pad_ev1: Default::default(),
                    parent_c,
                    parent_d,
                    parent_e,
                }
            }
        }
        unsafe impl ::protoss::Evolution for TestParentEv0 {
            type Base = TestParent;
            const VERSION: ::protoss::Version = ::protoss::Version::new(0u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestParentEv1 {
            type Base = TestParent;
            const VERSION: ::protoss::Version = ::protoss::Version::new(1u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestParentEv2 {
            type Base = TestParent;
            const VERSION: ::protoss::Version = ::protoss::Version::new(2u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolving for TestParent {
            type Probe = TestParentProbe;
            type LatestEvolution = TestParentEv2;
            fn probe_metadata(
                version: ::protoss::Version,
            ) -> Result<::protoss::ProbeMetadata, ::protoss::Error> {
                match version {
                    <TestParentEv0 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestParentEv0 as ::protoss::Evolution>::METADATA)
                    }
                    <TestParentEv1 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestParentEv1 as ::protoss::Evolution>::METADATA)
                    }
                    <TestParentEv2 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestParentEv2 as ::protoss::Evolution>::METADATA)
                    }
                    _ => {
                        Err(
                            ::protoss::Error::TriedToGetProbeMetadataForNonExistentVersion,
                        )
                    }
                }
            }
        }
        #[repr(transparent)]
        pub struct TestParentProbe {
            data: [u8],
        }
        impl ::protoss::Pointee for TestParentProbe {
            type Metadata = ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Probe for TestParentProbe {
            type Base = TestParent;
            #[inline(always)]
            unsafe fn as_version_unchecked<EV: ::protoss::Evolution<Base = TestParent>>(
                &self,
            ) -> &<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived {
                &*self
                    .data
                    .as_ptr()
                    .cast::<<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived>()
            }
            fn probe_as<EV: ::protoss::Evolution<Base = TestParent>>(
                &self,
            ) -> Option<&<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived> {
                let data_size = ::core::mem::size_of_val(&self.data);
                let version_size = ::core::mem::size_of::<
                    <EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
                >();
                if version_size <= data_size {
                    Some(unsafe { self.as_version_unchecked::<EV>() })
                } else {
                    None
                }
            }
            fn version(&self) -> Option<::protoss::Version> {
                match ::core::mem::size_of_val(&self.data) as ::protoss::ProbeMetadata {
                    <TestParentEv0 as ::protoss::Evolution>::METADATA => {
                        Some(<TestParentEv0 as ::protoss::Evolution>::VERSION)
                    }
                    <TestParentEv1 as ::protoss::Evolution>::METADATA => {
                        Some(<TestParentEv1 as ::protoss::Evolution>::VERSION)
                    }
                    <TestParentEv2 as ::protoss::Evolution>::METADATA => {
                        Some(<TestParentEv2 as ::protoss::Evolution>::VERSION)
                    }
                    _ => None,
                }
            }
        }
        impl TestParentProbe {
            pub fn test(
                &self,
            ) -> Option<&::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv0>(self) {
                    Some(&archived_ev.test)
                } else {
                    None
                }
            }
            pub fn parent_a(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv0>(self) {
                    Some(&archived_ev.parent_a)
                } else {
                    None
                }
            }
            pub fn parent_b(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv1>(self) {
                    Some(&archived_ev.parent_b)
                } else {
                    None
                }
            }
            pub fn parent_c(
                &self,
            ) -> Option<
                &::rkyv::Archived<::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>>,
            > {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv2>(self) {
                    Some(&archived_ev.parent_c)
                } else {
                    None
                }
            }
            pub fn parent_d(&self) -> Option<&::rkyv::Archived<String>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv2>(self) {
                    Some(&archived_ev.parent_d)
                } else {
                    None
                }
            }
            pub fn parent_e(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv2>(self) {
                    Some(&archived_ev.parent_e)
                } else {
                    None
                }
            }
        }
    }
    mod v4 {
        #[archive(
            as = "<<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived"
        )]
        pub struct Test {
            pub a: u32,
            pub b: u8,
            pub c: u32,
            pub d: u8,
            pub e: String,
            pub f: u32,
            pub g: Vec<String>,
            pub h: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`Test`]
        pub struct TestResolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            Vec<String>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
            d: ::rkyv::Resolver<u8>,
            e: ::rkyv::Resolver<String>,
            f: ::rkyv::Resolver<u32>,
            g: ::rkyv::Resolver<Vec<String>>,
            h: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for Test
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                String: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                Vec<String>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = <<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived;
                type Resolver = TestResolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.d), pos + fp, resolver.d, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).e;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.e), pos + fp, resolver.e, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).f;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.f), pos + fp, resolver.f, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).g;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.g), pos + fp, resolver.g, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).h;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.h), pos + fp, resolver.h, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for Test
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                String: Serialize<__S>,
                u32: Serialize<__S>,
                Vec<String>: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestResolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                        d: Serialize::<__S>::serialize(&self.d, serializer)?,
                        e: Serialize::<__S>::serialize(&self.e, serializer)?,
                        f: Serialize::<__S>::serialize(&self.f, serializer)?,
                        g: Serialize::<__S>::serialize(&self.g, serializer)?,
                        h: Serialize::<__S>::serialize(&self.h, serializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv0")]
        pub struct TestEv0 {
            pub a: u32,
            pub b: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv0`]
        pub struct TestEv0Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv0
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv0;
                type Resolver = TestEv0Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv0
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv0Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv0, __D> for Archived<TestEv0>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv0, __D::Error> {
                    Ok(TestEv0 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv1")]
        pub struct TestEv1 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv1`]
        pub struct TestEv1Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv1
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv1;
                type Resolver = TestEv1Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv1
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv1Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv1, __D> for Archived<TestEv1>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv1, __D::Error> {
                    Ok(TestEv1 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                        c: Deserialize::<u32, __D>::deserialize(&self.c, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv2")]
        pub struct TestEv2 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
            pub d: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv2`]
        pub struct TestEv2Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
            d: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv2
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv2;
                type Resolver = TestEv2Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.d), pos + fp, resolver.d, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv2
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv2Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                        d: Serialize::<__S>::serialize(&self.d, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv2, __D> for Archived<TestEv2>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv2, __D::Error> {
                    Ok(TestEv2 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                        c: Deserialize::<u32, __D>::deserialize(&self.c, deserializer)?,
                        d: Deserialize::<u8, __D>::deserialize(&self.d, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv3")]
        pub struct TestEv3 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
            pub d: u8,
            pub e: String,
            pub f: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv3`]
        pub struct TestEv3Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
            d: ::rkyv::Resolver<u8>,
            e: ::rkyv::Resolver<String>,
            f: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv3
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                String: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv3;
                type Resolver = TestEv3Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.d), pos + fp, resolver.d, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).e;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.e), pos + fp, resolver.e, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).f;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.f), pos + fp, resolver.f, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv3
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                String: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv3Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                        d: Serialize::<__S>::serialize(&self.d, serializer)?,
                        e: Serialize::<__S>::serialize(&self.e, serializer)?,
                        f: Serialize::<__S>::serialize(&self.f, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv3, __D> for Archived<TestEv3>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                String: Archive,
                Archived<String>: Deserialize<String, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv3, __D::Error> {
                    Ok(TestEv3 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                        c: Deserialize::<u32, __D>::deserialize(&self.c, deserializer)?,
                        d: Deserialize::<u8, __D>::deserialize(&self.d, deserializer)?,
                        e: Deserialize::<
                            String,
                            __D,
                        >::deserialize(&self.e, deserializer)?,
                        f: Deserialize::<u32, __D>::deserialize(&self.f, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestEv4")]
        pub struct TestEv4 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
            pub d: u8,
            pub e: String,
            pub f: u32,
            pub g: Vec<String>,
            pub h: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestEv4`]
        pub struct TestEv4Resolver
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            Vec<String>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            a: ::rkyv::Resolver<u32>,
            b: ::rkyv::Resolver<u8>,
            c: ::rkyv::Resolver<u32>,
            d: ::rkyv::Resolver<u8>,
            e: ::rkyv::Resolver<String>,
            f: ::rkyv::Resolver<u32>,
            g: ::rkyv::Resolver<Vec<String>>,
            h: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestEv4
            where
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                String: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                Vec<String>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestEv4;
                type Resolver = TestEv4Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.a), pos + fp, resolver.a, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.b), pos + fp, resolver.b, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.c), pos + fp, resolver.c, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.d), pos + fp, resolver.d, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).e;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.e), pos + fp, resolver.e, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).f;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.f), pos + fp, resolver.f, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).g;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.g), pos + fp, resolver.g, fo);
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).h;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve((&self.h), pos + fp, resolver.h, fo);
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestEv4
            where
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                u8: Serialize<__S>,
                String: Serialize<__S>,
                u32: Serialize<__S>,
                Vec<String>: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestEv4Resolver {
                        a: Serialize::<__S>::serialize(&self.a, serializer)?,
                        b: Serialize::<__S>::serialize(&self.b, serializer)?,
                        c: Serialize::<__S>::serialize(&self.c, serializer)?,
                        d: Serialize::<__S>::serialize(&self.d, serializer)?,
                        e: Serialize::<__S>::serialize(&self.e, serializer)?,
                        f: Serialize::<__S>::serialize(&self.f, serializer)?,
                        g: Serialize::<__S>::serialize(&self.g, serializer)?,
                        h: Serialize::<__S>::serialize(&self.h, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestEv4, __D> for Archived<TestEv4>
            where
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                String: Archive,
                Archived<String>: Deserialize<String, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                Vec<String>: Archive,
                Archived<Vec<String>>: Deserialize<Vec<String>, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestEv4, __D::Error> {
                    Ok(TestEv4 {
                        a: Deserialize::<u32, __D>::deserialize(&self.a, deserializer)?,
                        b: Deserialize::<u8, __D>::deserialize(&self.b, deserializer)?,
                        c: Deserialize::<u32, __D>::deserialize(&self.c, deserializer)?,
                        d: Deserialize::<u8, __D>::deserialize(&self.d, deserializer)?,
                        e: Deserialize::<
                            String,
                            __D,
                        >::deserialize(&self.e, deserializer)?,
                        f: Deserialize::<u32, __D>::deserialize(&self.f, deserializer)?,
                        g: Deserialize::<
                            Vec<String>,
                            __D,
                        >::deserialize(&self.g, deserializer)?,
                        h: Deserialize::<u8, __D>::deserialize(&self.h, deserializer)?,
                    })
                }
            }
        };
        #[repr(C)]
        pub struct ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "ArchivedTestEv0",
                    "a",
                    &&self.a,
                    "b",
                    &&self.b,
                    "_pad_ev0",
                    &&self._pad_ev0,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv0
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv0) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
            }
        }
        impl ArchivedTestEv0 {
            pub fn new(a: ::rkyv::Archived<u32>, b: ::rkyv::Archived<u8>) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
            pub c: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>, ::rkyv::Archived<u32>),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "ArchivedTestEv1",
                    "a",
                    &&self.a,
                    "b",
                    &&self.b,
                    "_pad_ev0",
                    &&self._pad_ev0,
                    "c",
                    &&self.c,
                    "_pad_ev1",
                    &&self._pad_ev1,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv1
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv1) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
                    && self.c == other.c && self._pad_ev1 == other._pad_ev1
            }
        }
        impl ArchivedTestEv1 {
            pub fn new(
                a: ::rkyv::Archived<u32>,
                b: ::rkyv::Archived<u8>,
                c: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                    c,
                    _pad_ev1: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
            pub c: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>, ::rkyv::Archived<u32>),
            >,
            pub d: ::rkyv::Archived<u8>,
            _pad_ev2: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                ),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "a",
                    "b",
                    "_pad_ev0",
                    "c",
                    "_pad_ev1",
                    "d",
                    "_pad_ev2",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.a,
                    &&self.b,
                    &&self._pad_ev0,
                    &&self.c,
                    &&self._pad_ev1,
                    &&self.d,
                    &&self._pad_ev2,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ArchivedTestEv2",
                    names,
                    values,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv2
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv2) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
                    && self.c == other.c && self._pad_ev1 == other._pad_ev1
                    && self.d == other.d && self._pad_ev2 == other._pad_ev2
            }
        }
        impl ArchivedTestEv2 {
            pub fn new(
                a: ::rkyv::Archived<u32>,
                b: ::rkyv::Archived<u8>,
                c: ::rkyv::Archived<u32>,
                d: ::rkyv::Archived<u8>,
            ) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                    c,
                    _pad_ev1: Default::default(),
                    d,
                    _pad_ev2: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestEv3
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
            pub c: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>, ::rkyv::Archived<u32>),
            >,
            pub d: ::rkyv::Archived<u8>,
            _pad_ev2: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                ),
            >,
            pub e: ::rkyv::Archived<String>,
            pub f: ::rkyv::Archived<u32>,
            _pad_ev3: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<String>,
                    ::rkyv::Archived<u32>,
                ),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv3
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "a",
                    "b",
                    "_pad_ev0",
                    "c",
                    "_pad_ev1",
                    "d",
                    "_pad_ev2",
                    "e",
                    "f",
                    "_pad_ev3",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.a,
                    &&self.b,
                    &&self._pad_ev0,
                    &&self.c,
                    &&self._pad_ev1,
                    &&self.d,
                    &&self._pad_ev2,
                    &&self.e,
                    &&self.f,
                    &&self._pad_ev3,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ArchivedTestEv3",
                    names,
                    values,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv3
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv3
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv3) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
                    && self.c == other.c && self._pad_ev1 == other._pad_ev1
                    && self.d == other.d && self._pad_ev2 == other._pad_ev2
                    && self.e == other.e && self.f == other.f
                    && self._pad_ev3 == other._pad_ev3
            }
        }
        impl ArchivedTestEv3 {
            pub fn new(
                a: ::rkyv::Archived<u32>,
                b: ::rkyv::Archived<u8>,
                c: ::rkyv::Archived<u32>,
                d: ::rkyv::Archived<u8>,
                e: ::rkyv::Archived<String>,
                f: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                    c,
                    _pad_ev1: Default::default(),
                    d,
                    _pad_ev2: Default::default(),
                    e,
                    f,
                    _pad_ev3: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestEv4
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            Vec<String>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub a: ::rkyv::Archived<u32>,
            pub b: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>),
            >,
            pub c: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (::rkyv::Archived<u32>, ::rkyv::Archived<u8>, ::rkyv::Archived<u32>),
            >,
            pub d: ::rkyv::Archived<u8>,
            _pad_ev2: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                ),
            >,
            pub e: ::rkyv::Archived<String>,
            pub f: ::rkyv::Archived<u32>,
            _pad_ev3: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<String>,
                    ::rkyv::Archived<u32>,
                ),
            >,
            pub g: ::rkyv::Archived<Vec<String>>,
            pub h: ::rkyv::Archived<u8>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for ArchivedTestEv4
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            Vec<String>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let names: &'static _ = &[
                    "a",
                    "b",
                    "_pad_ev0",
                    "c",
                    "_pad_ev1",
                    "d",
                    "_pad_ev2",
                    "e",
                    "f",
                    "_pad_ev3",
                    "g",
                    "h",
                ];
                let values: &[&dyn ::core::fmt::Debug] = &[
                    &&self.a,
                    &&self.b,
                    &&self._pad_ev0,
                    &&self.c,
                    &&self._pad_ev1,
                    &&self.d,
                    &&self._pad_ev2,
                    &&self.e,
                    &&self.f,
                    &&self._pad_ev3,
                    &&self.g,
                    &&self.h,
                ];
                ::core::fmt::Formatter::debug_struct_fields_finish(
                    f,
                    "ArchivedTestEv4",
                    names,
                    values,
                )
            }
        }
        impl ::core::marker::StructuralPartialEq for ArchivedTestEv4
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            Vec<String>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for ArchivedTestEv4
        where
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            Vec<String>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            #[inline]
            fn eq(&self, other: &ArchivedTestEv4) -> bool {
                self.a == other.a && self.b == other.b && self._pad_ev0 == other._pad_ev0
                    && self.c == other.c && self._pad_ev1 == other._pad_ev1
                    && self.d == other.d && self._pad_ev2 == other._pad_ev2
                    && self.e == other.e && self.f == other.f
                    && self._pad_ev3 == other._pad_ev3 && self.g == other.g
                    && self.h == other.h
            }
        }
        impl ArchivedTestEv4 {
            pub fn new(
                a: ::rkyv::Archived<u32>,
                b: ::rkyv::Archived<u8>,
                c: ::rkyv::Archived<u32>,
                d: ::rkyv::Archived<u8>,
                e: ::rkyv::Archived<String>,
                f: ::rkyv::Archived<u32>,
                g: ::rkyv::Archived<Vec<String>>,
                h: ::rkyv::Archived<u8>,
            ) -> Self {
                Self {
                    a,
                    b,
                    _pad_ev0: Default::default(),
                    c,
                    _pad_ev1: Default::default(),
                    d,
                    _pad_ev2: Default::default(),
                    e,
                    f,
                    _pad_ev3: Default::default(),
                    g,
                    h,
                }
            }
        }
        unsafe impl ::protoss::Evolution for TestEv0 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(0u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestEv1 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(1u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestEv2 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(2u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestEv3 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(3u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestEv4 {
            type Base = Test;
            const VERSION: ::protoss::Version = ::protoss::Version::new(4u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolving for Test {
            type Probe = TestProbe;
            type LatestEvolution = TestEv4;
            fn probe_metadata(
                version: ::protoss::Version,
            ) -> Result<::protoss::ProbeMetadata, ::protoss::Error> {
                match version {
                    <TestEv0 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv0 as ::protoss::Evolution>::METADATA)
                    }
                    <TestEv1 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv1 as ::protoss::Evolution>::METADATA)
                    }
                    <TestEv2 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv2 as ::protoss::Evolution>::METADATA)
                    }
                    <TestEv3 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv3 as ::protoss::Evolution>::METADATA)
                    }
                    <TestEv4 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestEv4 as ::protoss::Evolution>::METADATA)
                    }
                    _ => {
                        Err(
                            ::protoss::Error::TriedToGetProbeMetadataForNonExistentVersion,
                        )
                    }
                }
            }
        }
        #[repr(transparent)]
        pub struct TestProbe {
            data: [u8],
        }
        impl ::protoss::Pointee for TestProbe {
            type Metadata = ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Probe for TestProbe {
            type Base = Test;
            #[inline(always)]
            unsafe fn as_version_unchecked<EV: ::protoss::Evolution<Base = Test>>(
                &self,
            ) -> &<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived {
                &*self
                    .data
                    .as_ptr()
                    .cast::<<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived>()
            }
            fn probe_as<EV: ::protoss::Evolution<Base = Test>>(
                &self,
            ) -> Option<&<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived> {
                let data_size = ::core::mem::size_of_val(&self.data);
                let version_size = ::core::mem::size_of::<
                    <EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
                >();
                if version_size <= data_size {
                    Some(unsafe { self.as_version_unchecked::<EV>() })
                } else {
                    None
                }
            }
            fn version(&self) -> Option<::protoss::Version> {
                match ::core::mem::size_of_val(&self.data) as ::protoss::ProbeMetadata {
                    <TestEv0 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv0 as ::protoss::Evolution>::VERSION)
                    }
                    <TestEv1 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv1 as ::protoss::Evolution>::VERSION)
                    }
                    <TestEv2 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv2 as ::protoss::Evolution>::VERSION)
                    }
                    <TestEv3 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv3 as ::protoss::Evolution>::VERSION)
                    }
                    <TestEv4 as ::protoss::Evolution>::METADATA => {
                        Some(<TestEv4 as ::protoss::Evolution>::VERSION)
                    }
                    _ => None,
                }
            }
        }
        impl TestProbe {
            pub fn a(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv0>(self) {
                    Some(&archived_ev.a)
                } else {
                    None
                }
            }
            pub fn b(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv0>(self) {
                    Some(&archived_ev.b)
                } else {
                    None
                }
            }
            pub fn c(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv1>(self) {
                    Some(&archived_ev.c)
                } else {
                    None
                }
            }
            pub fn d(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv2>(self) {
                    Some(&archived_ev.d)
                } else {
                    None
                }
            }
            pub fn e(&self) -> Option<&::rkyv::Archived<String>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv3>(self) {
                    Some(&archived_ev.e)
                } else {
                    None
                }
            }
            pub fn f(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv3>(self) {
                    Some(&archived_ev.f)
                } else {
                    None
                }
            }
            pub fn g(&self) -> Option<&::rkyv::Archived<Vec<String>>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv4>(self) {
                    Some(&archived_ev.g)
                } else {
                    None
                }
            }
            pub fn h(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev) = ::protoss::Probe::probe_as::<TestEv4>(self) {
                    Some(&archived_ev.h)
                } else {
                    None
                }
            }
        }
        #[archive(
            as = "<<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived"
        )]
        pub struct TestParent {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
            pub parent_b: u32,
            #[with(rkyv::with::CopyOptimize)]
            pub parent_c: Vec<u32>,
            pub parent_d: String,
            pub parent_e: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParent`]
        pub struct TestParentResolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
            parent_b: ::rkyv::Resolver<u32>,
            parent_c: ::rkyv::Resolver<
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
            >,
            parent_d: ::rkyv::Resolver<String>,
            parent_e: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParent
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: ::rkyv::Archive,
                String: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = <<Self as ::protoss::Evolving>::LatestEvolution as ::protoss::rkyv_impl::rkyv::Archive>::Archived;
                type Resolver = TestParentResolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_b),
                        pos + fp,
                        resolver.parent_b,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<
                            _,
                            rkyv::with::CopyOptimize,
                        >::cast((&self.parent_c)),
                        pos + fp,
                        resolver.parent_c,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_d),
                        pos + fp,
                        resolver.parent_d,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_e;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_e),
                        pos + fp,
                        resolver.parent_e,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParent
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: Serialize<__S>,
                String: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentResolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                        parent_b: Serialize::<
                            __S,
                        >::serialize(&self.parent_b, serializer)?,
                        parent_c: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<
                                _,
                                rkyv::with::CopyOptimize,
                            >::cast(&self.parent_c),
                            serializer,
                        )?,
                        parent_d: Serialize::<
                            __S,
                        >::serialize(&self.parent_d, serializer)?,
                        parent_e: Serialize::<
                            __S,
                        >::serialize(&self.parent_e, serializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestParentEv0")]
        pub struct TestParentEv0 {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParentEv0`]
        pub struct TestParentEv0Resolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParentEv0
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
            {
                type Archived = ArchivedTestParentEv0;
                type Resolver = TestParentEv0Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParentEv0
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentEv0Resolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestParentEv0, __D>
            for Archived<TestParentEv0>
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Archive,
                Archived<
                    ::rkyv::with::With<Test, protoss::Evolve>,
                >: Deserialize<::rkyv::with::With<Test, protoss::Evolve>, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestParentEv0, __D::Error> {
                    Ok(TestParentEv0 {
                        test: Deserialize::<
                            ::rkyv::with::With<Test, protoss::Evolve>,
                            __D,
                        >::deserialize(&self.test, deserializer)?
                            .into_inner(),
                        parent_a: Deserialize::<
                            u8,
                            __D,
                        >::deserialize(&self.parent_a, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestParentEv1")]
        pub struct TestParentEv1 {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
            pub parent_b: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParentEv1`]
        pub struct TestParentEv1Resolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
            parent_b: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParentEv1
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestParentEv1;
                type Resolver = TestParentEv1Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_b),
                        pos + fp,
                        resolver.parent_b,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParentEv1
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentEv1Resolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                        parent_b: Serialize::<
                            __S,
                        >::serialize(&self.parent_b, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestParentEv1, __D>
            for Archived<TestParentEv1>
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Archive,
                Archived<
                    ::rkyv::with::With<Test, protoss::Evolve>,
                >: Deserialize<::rkyv::with::With<Test, protoss::Evolve>, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestParentEv1, __D::Error> {
                    Ok(TestParentEv1 {
                        test: Deserialize::<
                            ::rkyv::with::With<Test, protoss::Evolve>,
                            __D,
                        >::deserialize(&self.test, deserializer)?
                            .into_inner(),
                        parent_a: Deserialize::<
                            u8,
                            __D,
                        >::deserialize(&self.parent_a, deserializer)?,
                        parent_b: Deserialize::<
                            u32,
                            __D,
                        >::deserialize(&self.parent_b, deserializer)?,
                    })
                }
            }
        };
        #[archive(as = "ArchivedTestParentEv2")]
        pub struct TestParentEv2 {
            #[with(protoss::Evolve)]
            pub test: Test,
            pub parent_a: u8,
            pub parent_b: u32,
            #[with(rkyv::with::CopyOptimize)]
            pub parent_c: Vec<u32>,
            pub parent_d: String,
            pub parent_e: u32,
        }
        #[automatically_derived]
        ///The resolver for an archived [`TestParentEv2`]
        pub struct TestParentEv2Resolver
        where
            ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            test: ::rkyv::Resolver<::rkyv::with::With<Test, protoss::Evolve>>,
            parent_a: ::rkyv::Resolver<u8>,
            parent_b: ::rkyv::Resolver<u32>,
            parent_c: ::rkyv::Resolver<
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
            >,
            parent_d: ::rkyv::Resolver<String>,
            parent_e: ::rkyv::Resolver<u32>,
        }
        #[automatically_derived]
        const _: () = {
            use ::core::marker::PhantomData;
            use ::rkyv::{out_field, Archive, Archived};
            impl Archive for TestParentEv2
            where
                ::rkyv::with::With<Test, protoss::Evolve>: ::rkyv::Archive,
                u8: ::rkyv::Archive,
                u32: ::rkyv::Archive,
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: ::rkyv::Archive,
                String: ::rkyv::Archive,
                u32: ::rkyv::Archive,
            {
                type Archived = ArchivedTestParentEv2;
                type Resolver = TestParentEv2Resolver;
                #[allow(clippy::unit_arg)]
                #[inline]
                unsafe fn resolve(
                    &self,
                    pos: usize,
                    resolver: Self::Resolver,
                    out: *mut Self::Archived,
                ) {
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).test;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<_, protoss::Evolve>::cast((&self.test)),
                        pos + fp,
                        resolver.test,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_a;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_a),
                        pos + fp,
                        resolver.parent_a,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_b;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_b),
                        pos + fp,
                        resolver.parent_b,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_c;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        ::rkyv::with::With::<
                            _,
                            rkyv::with::CopyOptimize,
                        >::cast((&self.parent_c)),
                        pos + fp,
                        resolver.parent_c,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_d;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_d),
                        pos + fp,
                        resolver.parent_d,
                        fo,
                    );
                    let (fp, fo) = {
                        #[allow(unused_unsafe)]
                        unsafe {
                            let fo = &raw mut (*out).parent_e;
                            (fo.cast::<u8>().offset_from(out.cast::<u8>()) as usize, fo)
                        }
                    };
                    ::rkyv::Archive::resolve(
                        (&self.parent_e),
                        pos + fp,
                        resolver.parent_e,
                        fo,
                    );
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Fallible, Serialize};
            impl<__S: Fallible + ?Sized> Serialize<__S> for TestParentEv2
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Serialize<__S>,
                u8: Serialize<__S>,
                u32: Serialize<__S>,
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: Serialize<__S>,
                String: Serialize<__S>,
                u32: Serialize<__S>,
            {
                #[inline]
                fn serialize(
                    &self,
                    serializer: &mut __S,
                ) -> ::core::result::Result<Self::Resolver, __S::Error> {
                    Ok(TestParentEv2Resolver {
                        test: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<_, protoss::Evolve>::cast(&self.test),
                            serializer,
                        )?,
                        parent_a: Serialize::<
                            __S,
                        >::serialize(&self.parent_a, serializer)?,
                        parent_b: Serialize::<
                            __S,
                        >::serialize(&self.parent_b, serializer)?,
                        parent_c: Serialize::<
                            __S,
                        >::serialize(
                            ::rkyv::with::With::<
                                _,
                                rkyv::with::CopyOptimize,
                            >::cast(&self.parent_c),
                            serializer,
                        )?,
                        parent_d: Serialize::<
                            __S,
                        >::serialize(&self.parent_d, serializer)?,
                        parent_e: Serialize::<
                            __S,
                        >::serialize(&self.parent_e, serializer)?,
                    })
                }
            }
        };
        #[automatically_derived]
        const _: () = {
            use ::rkyv::{Archive, Archived, Deserialize, Fallible};
            impl<__D: Fallible + ?Sized> Deserialize<TestParentEv2, __D>
            for Archived<TestParentEv2>
            where
                ::rkyv::with::With<Test, protoss::Evolve>: Archive,
                Archived<
                    ::rkyv::with::With<Test, protoss::Evolve>,
                >: Deserialize<::rkyv::with::With<Test, protoss::Evolve>, __D>,
                u8: Archive,
                Archived<u8>: Deserialize<u8, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>: Archive,
                Archived<
                    ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
                >: Deserialize<
                    ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
                    __D,
                >,
                String: Archive,
                Archived<String>: Deserialize<String, __D>,
                u32: Archive,
                Archived<u32>: Deserialize<u32, __D>,
            {
                #[inline]
                fn deserialize(
                    &self,
                    deserializer: &mut __D,
                ) -> ::core::result::Result<TestParentEv2, __D::Error> {
                    Ok(TestParentEv2 {
                        test: Deserialize::<
                            ::rkyv::with::With<Test, protoss::Evolve>,
                            __D,
                        >::deserialize(&self.test, deserializer)?
                            .into_inner(),
                        parent_a: Deserialize::<
                            u8,
                            __D,
                        >::deserialize(&self.parent_a, deserializer)?,
                        parent_b: Deserialize::<
                            u32,
                            __D,
                        >::deserialize(&self.parent_b, deserializer)?,
                        parent_c: Deserialize::<
                            ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
                            __D,
                        >::deserialize(&self.parent_c, deserializer)?
                            .into_inner(),
                        parent_d: Deserialize::<
                            String,
                            __D,
                        >::deserialize(&self.parent_d, deserializer)?,
                        parent_e: Deserialize::<
                            u32,
                            __D,
                        >::deserialize(&self.parent_e, deserializer)?,
                    })
                }
            }
        };
        #[repr(C)]
        pub struct ArchivedTestParentEv0
        where
            Test: ::rkyv::Archive,
            u8: ::rkyv::Archive,
        {
            pub test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
            pub parent_a: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                ),
            >,
        }
        impl ArchivedTestParentEv0 {
            pub fn new(
                test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                parent_a: ::rkyv::Archived<u8>,
            ) -> Self {
                Self {
                    test,
                    parent_a,
                    _pad_ev0: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestParentEv1
        where
            Test: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
            pub parent_a: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                ),
            >,
            pub parent_b: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                ),
            >,
        }
        impl ArchivedTestParentEv1 {
            pub fn new(
                test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                parent_a: ::rkyv::Archived<u8>,
                parent_b: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    test,
                    parent_a,
                    _pad_ev0: Default::default(),
                    parent_b,
                    _pad_ev1: Default::default(),
                }
            }
        }
        #[repr(C)]
        pub struct ArchivedTestParentEv2
        where
            Test: ::rkyv::Archive,
            u8: ::rkyv::Archive,
            u32: ::rkyv::Archive,
            Vec<u32>: ::rkyv::Archive,
            String: ::rkyv::Archive,
            u32: ::rkyv::Archive,
        {
            pub test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
            pub parent_a: ::rkyv::Archived<u8>,
            _pad_ev0: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                ),
            >,
            pub parent_b: ::rkyv::Archived<u32>,
            _pad_ev1: ::protoss::rkyv_impl::PadToAlign<
                (
                    ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                    ::rkyv::Archived<u8>,
                    ::rkyv::Archived<u32>,
                ),
            >,
            pub parent_c: ::rkyv::Archived<
                ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
            >,
            pub parent_d: ::rkyv::Archived<String>,
            pub parent_e: ::rkyv::Archived<u32>,
        }
        impl ArchivedTestParentEv2 {
            pub fn new(
                test: ::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>,
                parent_a: ::rkyv::Archived<u8>,
                parent_b: ::rkyv::Archived<u32>,
                parent_c: ::rkyv::Archived<
                    ::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>,
                >,
                parent_d: ::rkyv::Archived<String>,
                parent_e: ::rkyv::Archived<u32>,
            ) -> Self {
                Self {
                    test,
                    parent_a,
                    _pad_ev0: Default::default(),
                    parent_b,
                    _pad_ev1: Default::default(),
                    parent_c,
                    parent_d,
                    parent_e,
                }
            }
        }
        unsafe impl ::protoss::Evolution for TestParentEv0 {
            type Base = TestParent;
            const VERSION: ::protoss::Version = ::protoss::Version::new(0u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestParentEv1 {
            type Base = TestParent;
            const VERSION: ::protoss::Version = ::protoss::Version::new(1u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolution for TestParentEv2 {
            type Base = TestParent;
            const VERSION: ::protoss::Version = ::protoss::Version::new(2u16);
            const METADATA: ::protoss::ProbeMetadata = ::core::mem::size_of::<
                <Self as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
            >() as ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Evolving for TestParent {
            type Probe = TestParentProbe;
            type LatestEvolution = TestParentEv2;
            fn probe_metadata(
                version: ::protoss::Version,
            ) -> Result<::protoss::ProbeMetadata, ::protoss::Error> {
                match version {
                    <TestParentEv0 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestParentEv0 as ::protoss::Evolution>::METADATA)
                    }
                    <TestParentEv1 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestParentEv1 as ::protoss::Evolution>::METADATA)
                    }
                    <TestParentEv2 as ::protoss::Evolution>::VERSION => {
                        Ok(<TestParentEv2 as ::protoss::Evolution>::METADATA)
                    }
                    _ => {
                        Err(
                            ::protoss::Error::TriedToGetProbeMetadataForNonExistentVersion,
                        )
                    }
                }
            }
        }
        #[repr(transparent)]
        pub struct TestParentProbe {
            data: [u8],
        }
        impl ::protoss::Pointee for TestParentProbe {
            type Metadata = ::protoss::ProbeMetadata;
        }
        unsafe impl ::protoss::Probe for TestParentProbe {
            type Base = TestParent;
            #[inline(always)]
            unsafe fn as_version_unchecked<EV: ::protoss::Evolution<Base = TestParent>>(
                &self,
            ) -> &<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived {
                &*self
                    .data
                    .as_ptr()
                    .cast::<<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived>()
            }
            fn probe_as<EV: ::protoss::Evolution<Base = TestParent>>(
                &self,
            ) -> Option<&<EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived> {
                let data_size = ::core::mem::size_of_val(&self.data);
                let version_size = ::core::mem::size_of::<
                    <EV as ::protoss::rkyv_impl::rkyv::Archive>::Archived,
                >();
                if version_size <= data_size {
                    Some(unsafe { self.as_version_unchecked::<EV>() })
                } else {
                    None
                }
            }
            fn version(&self) -> Option<::protoss::Version> {
                match ::core::mem::size_of_val(&self.data) as ::protoss::ProbeMetadata {
                    <TestParentEv0 as ::protoss::Evolution>::METADATA => {
                        Some(<TestParentEv0 as ::protoss::Evolution>::VERSION)
                    }
                    <TestParentEv1 as ::protoss::Evolution>::METADATA => {
                        Some(<TestParentEv1 as ::protoss::Evolution>::VERSION)
                    }
                    <TestParentEv2 as ::protoss::Evolution>::METADATA => {
                        Some(<TestParentEv2 as ::protoss::Evolution>::VERSION)
                    }
                    _ => None,
                }
            }
        }
        impl TestParentProbe {
            pub fn test(
                &self,
            ) -> Option<&::rkyv::Archived<::rkyv::with::With<Test, protoss::Evolve>>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv0>(self) {
                    Some(&archived_ev.test)
                } else {
                    None
                }
            }
            pub fn parent_a(&self) -> Option<&::rkyv::Archived<u8>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv0>(self) {
                    Some(&archived_ev.parent_a)
                } else {
                    None
                }
            }
            pub fn parent_b(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv1>(self) {
                    Some(&archived_ev.parent_b)
                } else {
                    None
                }
            }
            pub fn parent_c(
                &self,
            ) -> Option<
                &::rkyv::Archived<::rkyv::with::With<Vec<u32>, rkyv::with::CopyOptimize>>,
            > {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv2>(self) {
                    Some(&archived_ev.parent_c)
                } else {
                    None
                }
            }
            pub fn parent_d(&self) -> Option<&::rkyv::Archived<String>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv2>(self) {
                    Some(&archived_ev.parent_d)
                } else {
                    None
                }
            }
            pub fn parent_e(&self) -> Option<&::rkyv::Archived<u32>> {
                if let Some(archived_ev)
                    = ::protoss::Probe::probe_as::<TestParentEv2>(self) {
                    Some(&archived_ev.parent_e)
                } else {
                    None
                }
            }
        }
    }
}
