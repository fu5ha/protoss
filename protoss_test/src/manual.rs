macro_rules! define_types {
    () => {
        use rkyv::{Archived, Archive, Serialize, Deserialize};
        use protoss::rkyv_impl::PadToAlign;

        #[derive(Debug, Archive, Serialize, Deserialize)]
        #[archive(as = "ArchivedTestEnumEv0")]
        pub enum TestEnumEv0 {
            A(TestEv0),
        }

        #[derive(Debug, PartialEq)]
        #[repr(C, u64)]
        pub enum ArchivedTestEnumEv0 {
            A(ArchivedTestEv0) = 0,
        }

        #[derive(Debug, Archive, Serialize, Deserialize)]
        #[archive(as = "ArchivedTestEnumEv1")]
        pub enum TestEnumEv1 {
            A(TestEv1),
            B(BarEv0),
        }

        #[derive(Debug, PartialEq)]
        #[repr(C, u64)]
        pub enum ArchivedTestEnumEv1 {
            A(ArchivedTestEv1) = 0,
            B(ArchivedBarEv0) = 1,
        }

        #[derive(Debug, Archive, Serialize, Deserialize)]
        #[archive(as = "ArchivedBarEv0")]
        pub struct BarEv0 {
            pub a: u64,
            pub b: u8,
        }

        #[derive(Debug, PartialEq)]
        #[repr(C)]
        pub struct ArchivedBarEv0 {
            pub a: u64,
            pub b: u8,
            pub _pad0: PadToAlign<(Archived<u64>, Archived<u8>)>,
        }

        #[derive(Debug, Archive, Serialize, Deserialize)]
        #[archive(as = "ArchivedTestEv0")]
        pub struct TestEv0 {
            pub a: u32,
            pub b: u8,
        }

        #[derive(Debug, PartialEq)]
        #[repr(C)]
        pub struct ArchivedTestEv0 {
            pub a: u32,
            pub b: u8,
            pub _pad0: PadToAlign<(Archived<u32>, Archived<u8>)>,
        }

        #[derive(Debug, Archive, Serialize, Deserialize)]
        #[archive(as = "ArchivedTestEv1")]
        pub struct TestEv1 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
        }

        #[derive(Debug, PartialEq)]
        #[repr(C)]
        pub struct ArchivedTestEv1 {
            pub a: Archived<u32>,
            pub b: Archived<u8>,
            pub _pad0: PadToAlign<(Archived<u32>, Archived<u8>)>,
            pub c: Archived<u32>,
            pub _pad1: PadToAlign<(Archived<u32>, Archived<u8>, Archived<u32>)>,
        }

        #[derive(Debug, Archive, Serialize, Deserialize)]
        #[archive(as = "ArchivedTestEv2")]
        pub struct TestEv2 {
            pub a: u32,
            pub b: u8,
            pub c: u32,
            pub d: u8
        }

        #[derive(Debug, PartialEq)]
        #[repr(C)]
        pub struct ArchivedTestEv2 {
            pub a: Archived<u32>,
            pub b: Archived<u8>,
            pub _pad0: PadToAlign<(Archived<u32>, Archived<u8>)>,
            pub c: Archived<u32>,
            pub _pad1: PadToAlign<(Archived<u32>, Archived<u8>, Archived<u32>)>,
            pub d: Archived<u8>,
            pub _pad2: PadToAlign<(Archived<u32>, Archived<u8>, Archived<u32>, Archived<u8>)>,
        }
    }
}

mod v1 {
    use protoss::{Evolution, Evolving, AnyProbe, Probe, Version, ProbeMetadata};
    use ptr_meta::Pointee;

    define_types!();

    // #[evolving]
    #[derive(rkyv::Archive, rkyv::Serialize)]
    #[archive(as = "<<Self as Evolving>::LatestEvolution as Archive>::Archived")]
    pub enum TestEnum {
        //#[variant(id = 0, since_ev = 0)]
        A(Test),
        //#[variant(id = 1, since_ev = 1)]
        B(Bar),
    }

    #[derive(Pointee)]
    #[repr(C)]
    pub struct TestEnumProbe {
        discriminant: u64,
        variant_data: [u8]
    }

    unsafe impl Evolving for TestEnum {
        type Probe = TestEnumProbe;
        type LatestEvolution = TestEnumEv1;
        fn probe_metadata(version: Version) -> Result<ProbeMetadata, protoss::Error> {
            match version {
                TestEnumEv0::VERSION => Ok(core::mem::size_of::<rkyv::Archived<TestEnumV0>>() - 8),
                TestEnumEv1::VERSION => Ok(TestEnumEv1::METADATA),
                _ => Err(protoss::Error::TriedToGetProbeMetadataForNonExistentVersion)
            }
        }
    }

    unsafe impl Evolution for TestEnumEv0 {
        type Base = TestEnum;
        type Meta = u64;
        const VERSION: Version = Version::new(0);
        const METADATA: ProbeMetadata = core::mem::size_of::<Self::Archived>() as ProbeMetadata;
    }

    unsafe impl Evolution for TestEnumEv1 {
        type Base = TestEnum;
        const VERSION: Version = Version::new(1);
        const METADATA: ProbeMetadata = core::mem::size_of::<Self::Archived>() as ProbeMetadata;
    }

    unsafe impl Probe for TestEnumProbe {
        type Base = TestEnum;

        #[inline(always)]
        unsafe fn as_version_unchecked<V: Evolution<Base = TestEnum>>(&self) -> &V::Archived {
            &*self.as_ptr().cast::<V::Archived>()
        }

        fn probe_as<V: Evolution<Base = Bar>>(&self) -> Option<&V::Archived> {
            let data_size = core::mem::size_of_val(&self.data);
            let version_size = core::mem::size_of::<V::Archived>();
            if version_size <= data_size {
                Some(unsafe { self.as_version_unchecked::<V>() })
            } else {
                None
            }
        }

        fn version(&self) -> Option<Version> {
            match core::mem::size_of_val(&self.data) as ProbeMetadata {
                BarEv0::METADATA => Some(BarEv0::VERSION),
                _ => None,
            }
        }
    }

    impl TestEnumProbe {
        pub fn a(&self) -> Option<&> {
            let v0 = unsafe { self.as_version_unchecked::<BarEv0>() };
            Some(&v0.a)
        }

        pub fn b(&self) -> Option<&u8> {
            let v0 = unsafe { self.as_version_unchecked::<BarEv0>() };
            Some(&v0.b)
        }
    }
    // #[evolving]
    #[derive(rkyv::Archive, rkyv::Serialize)]
    #[archive(as = "<<Self as Evolving>::LatestEvolution as Archive>::Archived")]
    pub struct Bar {
        //#[field(id = 0, since_ev = 0)]
        pub a: u64,
        //#[field(id = 1, since_ev = 0)]
        pub b: u8,
    }

    // imagine this as Serialize
    impl From<Bar> for ArchivedBarEv0 {
        fn from(Test { a, b, c}: Test) -> Self {
            ArchivedBarEv0 {
                a,
                b,
                _pad0: Default::default(),
            }
        }
    }

    #[derive(Pointee)]
    #[repr(transparent)]
    pub struct BarProbe {
        data: [u8]
    }

    unsafe impl Evolving for Bar {
        type Probe = BarProbe;
        type LatestEvolution = BarEv0;
        fn probe_metadata(version: Version) -> Result<ProbeMetadata, protoss::Error> {
            match version {
                BarEv0::VERSION => Ok(BarEv0::METADATA),
                _ => Err(protoss::Error::TriedToGetProbeMetadataForNonExistentVersion)
            }
        }
    }

    unsafe impl Evolution for BarEv0 {
        type Base = Test;
        const VERSION: Version = Version::new(0);
        const METADATA: ProbeMetadata = core::mem::size_of::<Self::Archived>() as ProbeMetadata;
    }

    unsafe impl Probe for BarProbe {
        type Base = Bar;

        #[inline(always)]
        unsafe fn as_version_unchecked<V: Evolution<Base = Bar>>(&self) -> &V::Archived {
            &*self.data.as_ptr().cast::<V::Archived>()
        }

        fn probe_as<V: Evolution<Base = Bar>>(&self) -> Option<&V::Archived> {
            let data_size = core::mem::size_of_val(&self.data);
            let version_size = core::mem::size_of::<V::Archived>();
            if version_size <= data_size {
                Some(unsafe { self.as_version_unchecked::<V>() })
            } else {
                None
            }
        }

        fn version(&self) -> Option<Version> {
            match core::mem::size_of_val(&self.data) as ProbeMetadata {
                BarEv0::METADATA => Some(BarEv0::VERSION),
                _ => None,
            }
        }
    }

    impl BarProbe {
        pub fn a(&self) -> Option<&u32> {
            let v0 = unsafe { self.as_version_unchecked::<BarEv0>() };
            Some(&v0.a)
        }

        pub fn b(&self) -> Option<&u8> {
            let v0 = unsafe { self.as_version_unchecked::<BarEv0>() };
            Some(&v0.b)
        }
    }

    // #[evolving]
    #[derive(rkyv::Archive, rkyv::Serialize)]
    #[archive(as = "<<Self as Evolving>::LatestEvolution as Archive>::Archived")]
    pub struct Test {
        //#[field(id = 0, since_ev = 0)]
        pub a: u32,
        //#[field(id = 1, since_ev = 0)]
        pub b: u8,
        //#[field(id = 2, since_ev = 1)]
        pub c: u32,
    }

    // imagine this as Serialize
    impl From<Test> for ArchivedTestEv1 {
        fn from(Test { a, b, c}: Test) -> Self {
            ArchivedTestEv1 {
                a,
                b,
                c,
                _pad0: Default::default(),
                _pad1: Default::default(),
            }
        }
    }

    #[derive(Pointee)]
    #[repr(transparent)]
    pub struct TestProbe {
        data: [u8]
    }

    unsafe impl Evolving for Test {
        type Probe = TestProbe;
        type LatestEvolution = TestEv1;
        fn probe_metadata(version: Version) -> Result<<AnyProbe<Test> as Pointee>::Metadata, protoss::Error> {
            match version {
                TestEv0::VERSION => Ok(TestEv0::METADATA),
                TestEv1::VERSION => Ok(TestEv1::METADATA),
                _ => Err(protoss::Error::TriedToGetProbeMetadataForNonExistentVersion)
            }
        }
    }

    unsafe impl Evolution for TestEv0 {
        type Base = Test;
        const VERSION: Version = Version::new(0);
        const METADATA: ProbeMetadata = core::mem::size_of::<Self::Archived>() as ProbeMetadata;
    }
    unsafe impl Evolution for TestEv1 {
        type Base = Test;
        const VERSION: Version = Version::new(1);
        const METADATA: ProbeMetadata = core::mem::size_of::<Self::Archived>() as ProbeMetadata;
    }

    unsafe impl Probe for TestProbe {
        type Base = Test;

        #[inline(always)]
        unsafe fn as_version_unchecked<V: Evolution<Base = Test>>(&self) -> &V::Archived {
            &*self.data.as_ptr().cast::<V::Archived>()
        }

        fn probe_as<V: Evolution<Base = Test>>(&self) -> Option<&V::Archived> {
            let data_size = core::mem::size_of_val(&self.data);
            let version_size = core::mem::size_of::<V::Archived>();
            if version_size <= data_size {
                Some(unsafe { self.as_version_unchecked::<V>() })
            } else {
                None
            }
        }

        fn version(&self) -> Option<Version> {
            match core::mem::size_of_val(&self.data) as ProbeMetadata {
                TestEv0::METADATA => Some(TestEv0::VERSION),
                TestEv1::METADATA => Some(TestEv1::VERSION),
                _ => None,
            }
        }
    }

    impl TestProbe {
        pub fn a(&self) -> Option<&u32> {
            let v0 = unsafe { self.as_version_unchecked::<TestEv0>() };
            Some(&v0.a)
        }

        pub fn b(&self) -> Option<&u8> {
            let v0 = unsafe { self.as_version_unchecked::<TestEv0>() };
            Some(&v0.b)
        }

        pub fn c(&self) -> Option<&u32> {
            if let Some(v1) = self.probe_as::<TestEv1>() {
                Some(&v1.c)
            } else {
                None
            }
        }
    }
}

mod v2 {
    use protoss::{Evolution, Evolving, Version, Probe, AnyProbe, ProbeMetadata};
    use ptr_meta::Pointee;

    define_types!();

    //#[evolving]
    #[derive(rkyv::Archive, rkyv::Serialize)]
    #[archive(as = "<<Self as Evolving>::LatestEvolution as Archive>::Archived")]
    pub struct Test {
        //#[field(id = 0, since_ev = 0)]
        pub a: u32,
        //#[field(id = 1, since_ev = 0)]
        pub b: u8,
        //#[field(id = 2, since_ev = 1)]
        pub c: u32,
        //#[field(id = 3, since_ev = 2)]
        pub d: u8,
    }

    // imagine this as Serialize
    impl From<Test> for ArchivedTestEv2 {
        fn from(Test { a, b, c, d }: Test) -> Self {
            ArchivedTestEv2 {
                a,
                b,
                c,
                d,
                _pad0: Default::default(),
                _pad1: Default::default(),
                _pad2: Default::default(),
            }
        }
    }

    #[derive(Pointee)]
    #[repr(transparent)]
    pub struct TestProbe {
        data: [u8]
    }

    unsafe impl Evolving for Test {
        type Probe = TestProbe;
        type LatestEvolution = TestEv2;
        fn probe_metadata(version: Version) -> Result<<AnyProbe<Test> as Pointee>::Metadata, protoss::Error> {
            match version {
                TestEv0::VERSION => Ok(TestEv0::METADATA),
                TestEv1::VERSION => Ok(TestEv1::METADATA),
                TestEv2::VERSION => Ok(TestEv2::METADATA),
                _ => Err(protoss::Error::TriedToGetProbeMetadataForNonExistentVersion)
            }
        }
    }

    unsafe impl Evolution for TestEv0 {
        type Base = Test;
        const VERSION: Version = Version::new(0);
        const METADATA: ProbeMetadata = core::mem::size_of::<Self::Archived>() as ProbeMetadata;
    }

    unsafe impl Evolution for TestEv1 {
        type Base = Test;
        const VERSION: Version = Version::new(1);
        const METADATA: ProbeMetadata = core::mem::size_of::<Self::Archived>() as ProbeMetadata;
    }

    unsafe impl Evolution for TestEv2 {
        type Base = Test;
        const VERSION: Version = Version::new(2);
        const METADATA: ProbeMetadata = core::mem::size_of::<Self::Archived>() as ProbeMetadata;
    }

    unsafe impl Probe for TestProbe {
        type Base = Test;

        #[inline(always)]
        unsafe fn as_version_unchecked<V: Evolution<Base = Test>>(&self) -> &V::Archived {
            &*self.data.as_ptr().cast::<V::Archived>()
        }

        fn probe_as<V: Evolution<Base = Test>>(&self) -> Option<&V::Archived> {
            let data_size = core::mem::size_of_val(&self.data);
            let version_size = core::mem::size_of::<V::Archived>();
            if version_size <= data_size {
                Some(unsafe { self.as_version_unchecked::<V>() })
            } else {
                None
            }
        }

        fn version(&self) -> Option<Version> {
            match core::mem::size_of_val(&self.data) as ProbeMetadata {
                TestEv0::METADATA => Some(TestEv0::VERSION),
                TestEv1::METADATA => Some(TestEv1::VERSION),
                TestEv2::METADATA => Some(TestEv2::VERSION),
                _ => None,
            }
        }
    }

    impl TestProbe {
        pub fn a(&self) -> Option<&u32> {
            let v0 = unsafe { self.as_version_unchecked::<TestEv0>() };
            Some(&v0.a)
        }

        pub fn b(&self) -> Option<&u8> {
            let v0 = unsafe { self.as_version_unchecked::<TestEv0>() };
            Some(&v0.b)
        }

        pub fn c(&self) -> Option<&u32> {
            if let Some(v1) = self.probe_as::<TestEv1>() {
                Some(&v1.c)
            } else {
                None
            }
        }

        pub fn d(&self) -> Option<&u8> {
            if let Some(v2) = self.probe_as::<TestEv2>() {
                Some(&v2.d)
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use protoss::Probe;
    use protoss::Evolve;
    use protoss::rkyv_impl::pad;
    use rkyv::AlignedVec;
    use rkyv::Archive;
    use rkyv::Archived;
    use rkyv::Serialize;
    use rkyv::archived_root;
    use rkyv::ser::Serializer;
    use rkyv::ser::serializers::AllocSerializer;

    type DefaultSerializer = AllocSerializer<256>;

    #[test]
    fn basic_archiving() {
        #[derive(Archive, Serialize)]
        struct Container {
            #[with(Evolve)]
            test: v1::Test,
        }

        let container = Container {
            test: v1::Test {
                a: 1,
                b: 2,
                c: 3,
            }
        };

        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();

        let archived_container: &ArchivedContainer = unsafe { archived_root::<Container>(&buf) };
        let archived_test: &protoss::ArchivedEvolution<v1::Test> = &archived_container.test;

        let probe = archived_test.as_probe();

        assert_eq!(probe.probe_as::<v1::TestEv0>(), Some(&v1::ArchivedTestEv0 { a: 1, b: 2, _pad0: pad() }));
        assert_eq!(probe.probe_as::<v1::TestEv1>(), Some(&v1::ArchivedTestEv1 { a: 1, b: 2, _pad0: pad(), c: 3, _pad1: pad() }));
        assert_eq!(probe.a(), Some(&1));
        assert_eq!(probe.b(), Some(&2));
        assert_eq!(probe.c(), Some(&3));
    }

    #[test]
    fn basic_evolution_backwards_compat() {
        #[derive(Archive, Serialize)]
        struct ContainerV1 {
            #[with(Evolve)]
            test: v1::Test,
        }

        #[derive(Archive, Serialize)]
        struct ContainerV2 {
            #[with(Evolve)]
            test: v2::Test,
        }

        let container_v1 = ContainerV1 {
            test: v1::Test {
                a: 1,
                b: 2,
                c: 3,
            }
        };

        // producer is on v1, serializes a v1
        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container_v1).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();


        // consumer is on v2, accesses v1 archive as v2
        let archived_container: &Archived<ContainerV2> = unsafe { archived_root::<ContainerV2>(&buf) };
        let archived_test: &protoss::ArchivedEvolution<v2::Test> = &archived_container.test;

        // v2 probe from v1 archived data
        let probe: &v2::TestProbe = archived_test.as_probe();

        assert_eq!(probe.probe_as::<v2::TestEv0>(), Some(&v2::ArchivedTestEv0 { a: 1, b: 2, _pad0: pad() }));
        assert_eq!(probe.probe_as::<v2::TestEv1>(), Some(&v2::ArchivedTestEv1 { a: 1, b: 2, _pad0: pad(), c: 3, _pad1: pad() }));
        assert_eq!(probe.probe_as::<v2::TestEv2>(), None);
        assert_eq!(probe.a(), Some(&1));
        assert_eq!(probe.b(), Some(&2));
        assert_eq!(probe.c(), Some(&3));
        assert_eq!(probe.d(), None);
    }

    #[test]
    fn basic_evolution_forwards_compat() {
        #[derive(Archive, Serialize)]
        struct ContainerV1 {
            #[with(Evolve)]
            test: v1::Test,
        }

        #[derive(Archive, Serialize)]
        struct ContainerV2 {
            #[with(Evolve)]
            test: v2::Test,
        }

        let container_v2 = ContainerV2 {
            test: v2::Test {
                a: 5,
                b: 6,
                c: 7,
                d: 8,
            }
        };

        // producer is on v2, serializes v2
        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container_v2).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();


        // consumer is on v1, accesses v2-serialized archive as v1
        let archived_container: &Archived<ContainerV1> = unsafe { archived_root::<ContainerV1>(&buf) };
        let archived_test: &protoss::ArchivedEvolution<v1::Test> = &archived_container.test;

        // v1 probe from v2 archived data
        let probe: &v1::TestProbe = archived_test.as_probe();

        assert_eq!(probe.probe_as::<v1::TestEv0>(), Some(&v1::ArchivedTestEv0 { a: 5, b: 6, _pad0: pad() }));
        assert_eq!(probe.probe_as::<v1::TestEv1>(), Some(&v1::ArchivedTestEv1 { a: 5, b: 6, _pad0: pad(), c: 7, _pad1: pad() }));
        // compile fails because v1 doesn't know about V0_2!
        // assert_eq!(probe.probe_as::<TestEv0_2>(), Some(&ArchivedTestEv0_2 { a: 5, b: 6, _pad0: pad(), c: 7, _pad1: pad(), d: 8, _pad2: pad() }));
        assert_eq!(probe.a(), Some(&5));
        assert_eq!(probe.b(), Some(&6));
        assert_eq!(probe.c(), Some(&7));
        // compile fails because v1 doesn't know about field d on V0_2!
        // assert_eq!(probe.d(), Some(&8));
    }
}
