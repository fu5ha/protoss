
mod v1 {
    #[protoss::evolving]
    pub struct Test {
        #[field(id = 1, since_ev = 0)]
        pub b: u8,
        #[field(id = 2, since_ev = 1)]
        pub c: u32,
        #[field(id = 0, since_ev = 0)]
        pub a: u32,
    }
}

mod v2 {
    #[protoss::evolving]
    pub struct Test {
        #[field(id = 0, since_ev = 0)]
        pub a: u32,
        #[field(id = 1, since_ev = 0)]
        pub b: u8,
        #[field(id = 2, since_ev = 1)]
        pub c: u32,
        #[field(id = 3, since_ev = 2)]
        pub d: u8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use protoss::Probe;
    use protoss::Evolve;
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

        assert_eq!(probe.probe_as::<v1::TestEv0>(), Some(&v1::ArchivedTestEv0::new(1, 2)));
        assert_eq!(probe.probe_as::<v1::TestEv1>(), Some(&v1::ArchivedTestEv1::new(1, 2, 3)));
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

        assert_eq!(probe.probe_as::<v2::TestEv0>(), Some(&v2::ArchivedTestEv0::new(1, 2)));
        assert_eq!(probe.probe_as::<v2::TestEv1>(), Some(&v2::ArchivedTestEv1::new(1, 2, 3)));
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

        assert_eq!(probe.probe_as::<v1::TestEv0>(), Some(&v1::ArchivedTestEv0::new(5, 6)));
        assert_eq!(probe.probe_as::<v1::TestEv1>(), Some(&v1::ArchivedTestEv1::new(5, 6, 7)));
        // compile fails because v1 doesn't know about Ev2!
        // assert_eq!(probe.probe_as::<v1::TestEv2>(), Some(&v2::ArchivedTestEv2::new(5, 6, 7, 8)));
        assert_eq!(probe.a(), Some(&5));
        assert_eq!(probe.b(), Some(&6));
        assert_eq!(probe.c(), Some(&7));
        // compile fails because v1 doesn't know about field d on Ev2!
        // assert_eq!(probe.d(), Some(&8));
    }
}
