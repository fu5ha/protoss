
mod v1 {
    #[protoss::evolving]
    #[archived_evolution_attr(derive(Debug, PartialEq))]
    pub struct Test {
        #[field(id = 1, since_ev = 0)]
        pub b: u8,
        #[field(id = 2, since_ev = 1)]
        pub c: u32,
        #[field(id = 0, since_ev = 0)]
        pub a: u32,
    }

    #[protoss::evolving]
    pub struct TestParent {
        #[field(id = 0, since_ev = 0)]
        #[with(protoss::Evolve)]
        pub test: Test,
        #[field(id = 1, since_ev = 0)]
        pub parent_a: u8,
    }
}

mod v2 {
    #[protoss::evolving]
    #[archived_evolution_attr(derive(Debug, PartialEq))]
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

    #[protoss::evolving]
    pub struct TestParent {
        #[field(id = 0, since_ev = 0)]
        #[with(protoss::Evolve)]
        pub test: Test,
        #[field(id = 1, since_ev = 0)]
        pub parent_a: u8,
        #[field(id = 2, since_ev = 1)]
        pub parent_b: u32,
    }
}

mod v3 {
    #[protoss::evolving]
    #[archived_evolution_attr(derive(Debug, PartialEq))]
    pub struct Test {
        #[field(id = 0, since_ev = 0)]
        pub a: u32,
        #[field(id = 1, since_ev = 0)]
        pub b: u8,
        #[field(id = 2, since_ev = 1)]
        pub c: u32,
        #[field(id = 3, since_ev = 2)]
        pub d: u8,
        #[field(id = 4, since_ev = 3)]
        pub e: String,
        #[field(id = 5, since_ev = 3)]
        pub f: u32,
    }

    #[protoss::evolving]
    pub struct TestParent {
        #[field(id = 0, since_ev = 0)]
        #[with(protoss::Evolve)]
        pub test: Test,
        #[field(id = 1, since_ev = 0)]
        pub parent_a: u8,
        #[field(id = 2, since_ev = 1)]
        pub parent_b: u32,
        #[field(id = 3, since_ev = 2)]
        #[with(rkyv::with::CopyOptimize)]
        pub parent_c: Vec<u32>,
        #[field(id = 4, since_ev = 2)]
        pub parent_d: String,
        #[field(id = 5, since_ev = 2)]
        pub parent_e: u32,
    }
}

mod v4 {
    #[protoss::evolving]
    #[archived_evolution_attr(derive(Debug, PartialEq))]
    pub struct Test {
        #[field(id = 0, since_ev = 0)]
        pub a: u32,
        #[field(id = 1, since_ev = 0)]
        pub b: u8,
        #[field(id = 2, since_ev = 1)]
        pub c: u32,
        #[field(id = 3, since_ev = 2)]
        pub d: u8,
        #[field(id = 4, since_ev = 3)]
        pub e: String,
        #[field(id = 5, since_ev = 3)]
        pub f: u32,
        #[field(id = 6, since_ev = 4)]
        pub g: Vec<String>,
        #[field(id = 7, since_ev = 4)]
        pub h: u8,
    }

    #[protoss::evolving]
    pub struct TestParent {
        #[field(id = 0, since_ev = 0)]
        #[with(protoss::Evolve)]
        pub test: Test,
        #[field(id = 1, since_ev = 0)]
        pub parent_a: u8,
        #[field(id = 2, since_ev = 1)]
        pub parent_b: u32,
        #[field(id = 3, since_ev = 2)]
        #[with(rkyv::with::CopyOptimize)]
        pub parent_c: Vec<u32>,
        #[field(id = 4, since_ev = 2)]
        pub parent_d: String,
        #[field(id = 5, since_ev = 2)]
        pub parent_e: u32,
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
    fn nested_evolve_basic_archiving() {
        #[derive(Archive, Serialize)]
        struct Container {
            #[with(Evolve)]
            test_parent: v1::TestParent,
        }

        let container = Container {
            test_parent: v1::TestParent {
                test: v1::Test {
                    a: 1,
                    b: 2,
                    c: 3,
                },
                parent_a: 12,
            }
        };

        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();

        let archived_container: &ArchivedContainer = unsafe { archived_root::<Container>(&buf) };
        let archived_test_parent: &protoss::ArchivedEvolution<v1::TestParent> = &archived_container.test_parent;

        let test_parent_probe = archived_test_parent.as_probe();

        assert!(test_parent_probe.test().is_some());
        assert_eq!(test_parent_probe.parent_a(), Some(&12));

        let test_probe = test_parent_probe.test().unwrap().as_probe();

        assert_eq!(test_probe.probe_as::<v1::TestEv0>(), Some(&v1::ArchivedTestEv0::new(1, 2)));
        assert_eq!(test_probe.probe_as::<v1::TestEv1>(), Some(&v1::ArchivedTestEv1::new(1, 2, 3)));
        assert_eq!(test_probe.a(), Some(&1));
        assert_eq!(test_probe.b(), Some(&2));
        assert_eq!(test_probe.c(), Some(&3));
    }

    #[test]
    fn complex_archiving() {
        #[derive(Archive, Serialize)]
        struct Container {
            #[with(Evolve)]
            test: v3::Test,
        }

        let container = Container {
            test: v3::Test {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: String::from("test string"),
                f: 5
            }
        };

        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();

        let archived_container: &ArchivedContainer = unsafe { archived_root::<Container>(&buf) };
        let archived_test: &protoss::ArchivedEvolution<v3::Test> = &archived_container.test;

        let probe = archived_test.as_probe();

        assert_eq!(probe.a(), Some(&1));
        assert_eq!(probe.b(), Some(&2));
        assert_eq!(probe.c(), Some(&3));
        assert_eq!(probe.d(), Some(&4));
        assert!(probe.e().is_some());
        assert_eq!(probe.e().unwrap(), "test string");
        assert_eq!(probe.f(), Some(&5));
    }

    #[test]
    fn complex_archiving_2() {
        #[derive(Archive, Serialize)]
        struct Container {
            #[with(Evolve)]
            test: v4::Test,
        }

        let container = Container {
            test: v4::Test {
                a: 1,
                b: 2,
                c: 3,
                d: 4,
                e: String::from("test string"),
                f: 5,
                g: vec![String::from("test1"), String::from("test2"), String::from("test3")],
                h: 6,
            }
        };

        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();

        let archived_container: &ArchivedContainer = unsafe { archived_root::<Container>(&buf) };
        let archived_test: &protoss::ArchivedEvolution<v4::Test> = &archived_container.test;

        let probe = archived_test.as_probe();

        assert_eq!(probe.a(), Some(&1));
        assert_eq!(probe.b(), Some(&2));
        assert_eq!(probe.c(), Some(&3));
        assert_eq!(probe.d(), Some(&4));
        assert!(probe.e().is_some());
        assert_eq!(probe.e().unwrap(), "test string");
        assert_eq!(probe.f(), Some(&5));
        assert!(probe.g().is_some());
        assert_eq!(probe.g().unwrap(), &["test1", "test2", "test3"]);
        assert_eq!(probe.h(), Some(&6));
    }

    #[test]
    fn nested_complex_archiving() {
        #[derive(Archive, Serialize)]
        struct Container {
            #[with(Evolve)]
            test_parent: v3::TestParent,
        }

        let container = Container {
            test_parent: v3::TestParent {
                test: v3::Test {
                    a: 1,
                    b: 2,
                    c: 3,
                    d: 4,
                    e: String::from("test string"),
                    f: 5
                },
                parent_a: 12,
                parent_b: 13,
                parent_c: vec![0, 1, 2, 3, 10, 11, 12],
                parent_d: String::from("parent test string"),
                parent_e: 14,
            }
        };

        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();

        let archived_container: &ArchivedContainer = unsafe { archived_root::<Container>(&buf) };
        let archived_test_parent: &protoss::ArchivedEvolution<v3::TestParent> = &archived_container.test_parent;

        let test_parent_probe = archived_test_parent.as_probe();

        assert!(test_parent_probe.test().is_some());
        assert_eq!(test_parent_probe.parent_a(), Some(&12));
        assert_eq!(test_parent_probe.parent_b(), Some(&13));
        assert!(test_parent_probe.parent_c().is_some());
        assert_eq!(test_parent_probe.parent_c().unwrap(), &[0, 1, 2, 3, 10, 11, 12]);
        assert!(test_parent_probe.parent_d().is_some());
        assert_eq!(test_parent_probe.parent_d().unwrap(), "parent test string");
        assert_eq!(test_parent_probe.parent_e(), Some(&14));

        let test_probe = test_parent_probe.test().unwrap().as_probe();

        assert_eq!(test_probe.a(), Some(&1));
        assert_eq!(test_probe.b(), Some(&2));
        assert_eq!(test_probe.c(), Some(&3));
        assert_eq!(test_probe.d(), Some(&4));
        assert!(test_probe.e().is_some());
        assert_eq!(test_probe.e().unwrap(), "test string");
        assert_eq!(test_probe.f(), Some(&5));
    }

    #[test]
    fn nested_complex_archiving_2() {
        #[derive(Archive, Serialize)]
        struct Container {
            #[with(Evolve)]
            test_parent: v4::TestParent,
        }

        let container = Container {
            test_parent: v4::TestParent {
                test: v4::Test {
                    a: 1,
                    b: 2,
                    c: 3,
                    d: 4,
                    e: String::from("test string"),
                    f: 5,
                    g: vec![String::from("test1"), String::from("test2"), String::from("test3")],
                    h: 6,
                },
                parent_a: 12,
                parent_b: 13,
                parent_c: vec![0, 1, 2, 3, 10, 11, 12],
                parent_d: String::from("parent test string"),
                parent_e: 14,
            }
        };

        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();

        let archived_container: &ArchivedContainer = unsafe { archived_root::<Container>(&buf) };
        let archived_test_parent: &protoss::ArchivedEvolution<v4::TestParent> = &archived_container.test_parent;

        let test_parent_probe = archived_test_parent.as_probe();

        assert!(test_parent_probe.test().is_some());
        assert_eq!(test_parent_probe.parent_a(), Some(&12));
        assert_eq!(test_parent_probe.parent_b(), Some(&13));
        assert!(test_parent_probe.parent_c().is_some());
        assert_eq!(test_parent_probe.parent_c().unwrap(), &[0, 1, 2, 3, 10, 11, 12]);
        assert!(test_parent_probe.parent_d().is_some());
        assert_eq!(test_parent_probe.parent_d().unwrap(), "parent test string");
        assert_eq!(test_parent_probe.parent_e(), Some(&14));

        let test_probe = test_parent_probe.test().unwrap().as_probe();

        assert_eq!(test_probe.a(), Some(&1));
        assert_eq!(test_probe.b(), Some(&2));
        assert_eq!(test_probe.c(), Some(&3));
        assert_eq!(test_probe.d(), Some(&4));
        assert!(test_probe.e().is_some());
        assert_eq!(test_probe.e().unwrap(), "test string");
        assert_eq!(test_probe.f(), Some(&5));
        assert!(test_probe.g().is_some());
        assert_eq!(test_probe.g().unwrap(), &["test1", "test2", "test3"]);
        assert_eq!(test_probe.h(), Some(&6));
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

    #[test]
    fn nested_basic_evolution_backwards_compat() {
        #[derive(Archive, Serialize)]
        struct ContainerV1 {
            #[with(Evolve)]
            test_parent: v1::TestParent,
        }

        #[derive(Archive, Serialize)]
        struct ContainerV2 {
            #[with(Evolve)]
            test_parent: v2::TestParent,
        }

        let container_v1 = ContainerV1 {
            test_parent: v1::TestParent {
                test: v1::Test {
                    a: 1,
                    b: 2,
                    c: 3,
                },
                parent_a: 12,
            }
        };

        // producer is on v1, serializes a v1
        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container_v1).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();


        // consumer is on v2, accesses v1 archive as v2
        let archived_container: &Archived<ContainerV2> = unsafe { archived_root::<ContainerV2>(&buf) };
        let archived_test_parent: &protoss::ArchivedEvolution<v2::TestParent> = &archived_container.test_parent;

        // v2 probe from v1 archived data
        let parent_probe: &v2::TestParentProbe = archived_test_parent.as_probe();

        assert!(parent_probe.test().is_some());
        assert_eq!(parent_probe.parent_a(), Some(&12));
        assert_eq!(parent_probe.parent_b(), None);

        let test_probe = parent_probe.test().unwrap().as_probe();

        assert_eq!(test_probe.probe_as::<v2::TestEv0>(), Some(&v2::ArchivedTestEv0::new(1, 2)));
        assert_eq!(test_probe.probe_as::<v2::TestEv1>(), Some(&v2::ArchivedTestEv1::new(1, 2, 3)));
        assert_eq!(test_probe.probe_as::<v2::TestEv2>(), None);
        assert_eq!(test_probe.a(), Some(&1));
        assert_eq!(test_probe.b(), Some(&2));
        assert_eq!(test_probe.c(), Some(&3));
        assert_eq!(test_probe.d(), None);
    }

    #[test]
    fn nested_basic_evolution_forwards_compat() {
        #[derive(Archive, Serialize)]
        struct ContainerV1 {
            #[with(Evolve)]
            test_parent: v1::TestParent,
        }

        #[derive(Archive, Serialize)]
        struct ContainerV2 {
            #[with(Evolve)]
            test_parent: v2::TestParent,
        }

        let container_v2 = ContainerV2 {
            test_parent: v2::TestParent {
                test: v2::Test {
                    a: 5,
                    b: 6,
                    c: 7,
                    d: 8,
                },
                parent_a: 12,
                parent_b: 13,
            }
        };

        // producer is on v2, serializes v2
        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container_v2).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();


        // consumer is on v1, accesses v2-serialized archive as v1
        let archived_container: &Archived<ContainerV1> = unsafe { archived_root::<ContainerV1>(&buf) };
        let archived_test_parent: &protoss::ArchivedEvolution<v1::TestParent> = &archived_container.test_parent;

        // v1 probe from v2 archived data
        let parent_probe: &v1::TestParentProbe = archived_test_parent.as_probe();

        assert!(parent_probe.test().is_some());
        assert_eq!(parent_probe.parent_a(), Some(&12));
        // compile fails because v1 doesn't know about TestEv2!
        // assert_eq!(parent_probe.parent_b(), None);

        let test_probe = parent_probe.test().unwrap().as_probe();

        assert_eq!(test_probe.probe_as::<v1::TestEv0>(), Some(&v1::ArchivedTestEv0::new(5, 6)));
        assert_eq!(test_probe.probe_as::<v1::TestEv1>(), Some(&v1::ArchivedTestEv1::new(5, 6, 7)));
        // compile fails because v1 doesn't know about Ev2!
        // assert_eq!(probe.probe_as::<v1::TestEv2>(), Some(&v2::ArchivedTestEv2::new(5, 6, 7, 8)));
        assert_eq!(test_probe.a(), Some(&5));
        assert_eq!(test_probe.b(), Some(&6));
        assert_eq!(test_probe.c(), Some(&7));
        // compile fails because v1 doesn't know about field d on Ev2!
        // assert_eq!(probe.d(), Some(&8));
    }

    #[test]
    fn nested_advanced_evolution_backwards_compat() {
        #[derive(Archive, Serialize)]
        struct ContainerV1 {
            #[with(Evolve)]
            test_parent: v1::TestParent,
        }

        #[derive(Archive, Serialize)]
        struct ContainerV3 {
            #[with(Evolve)]
            test_parent: v3::TestParent,
        }

        let container_v1 = ContainerV1 {
            test_parent: v1::TestParent {
                test: v1::Test {
                    a: 1,
                    b: 2,
                    c: 3,
                },
                parent_a: 12,
            }
        };

        // producer is on v1, serializes a v1
        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container_v1).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();


        // consumer is on v3, accesses v1 archive as v3
        let archived_container: &Archived<ContainerV3> = unsafe { archived_root::<ContainerV3>(&buf) };
        let archived_test_parent: &protoss::ArchivedEvolution<v3::TestParent> = &archived_container.test_parent;

        // v3 probe from v1 archived data
        let parent_probe: &v3::TestParentProbe = archived_test_parent.as_probe();

        assert!(parent_probe.test().is_some());
        assert_eq!(parent_probe.parent_a(), Some(&12));
        assert_eq!(parent_probe.parent_b(), None);
        assert_eq!(parent_probe.parent_c(), None);
        assert_eq!(parent_probe.parent_d(), None);
        assert_eq!(parent_probe.parent_e(), None);

        let test_probe = parent_probe.test().unwrap().as_probe();

        assert_eq!(test_probe.probe_as::<v3::TestEv0>(), Some(&v3::ArchivedTestEv0::new(1, 2)));
        assert_eq!(test_probe.probe_as::<v3::TestEv1>(), Some(&v3::ArchivedTestEv1::new(1, 2, 3)));
        assert_eq!(test_probe.probe_as::<v3::TestEv2>(), None);
        assert_eq!(test_probe.probe_as::<v3::TestEv3>(), None);
        assert_eq!(test_probe.a(), Some(&1));
        assert_eq!(test_probe.b(), Some(&2));
        assert_eq!(test_probe.c(), Some(&3));
        assert_eq!(test_probe.d(), None);
        assert_eq!(test_probe.e(), None);
        assert_eq!(test_probe.f(), None);
    }

    #[test]
    fn nested_advanced_evolution_forwards_compat() {
        #[derive(Archive, Serialize)]
        struct ContainerV1 {
            #[with(Evolve)]
            test_parent: v1::TestParent,
        }

        #[derive(Archive, Serialize)]
        struct ContainerV3 {
            #[with(Evolve)]
            test_parent: v3::TestParent,
        }

        let container_v3 = ContainerV3 {
            test_parent: v3::TestParent {
                test: v3::Test {
                    a: 1,
                    b: 2,
                    c: 3,
                    d: 4,
                    e: String::from("test string"),
                    f: 5
                },
                parent_a: 12,
                parent_b: 13,
                parent_c: vec![0, 1, 2, 3, 10, 11, 12],
                parent_d: String::from("parent test string"),
                parent_e: 14,
            }
        };

        // producer is on v3, serializes v3
        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container_v3).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();


        // consumer is on v1, accesses v3-serialized archive as v1
        let archived_container: &Archived<ContainerV1> = unsafe { archived_root::<ContainerV1>(&buf) };
        let archived_test_parent: &protoss::ArchivedEvolution<v1::TestParent> = &archived_container.test_parent;

        // v1 probe from v3 archived data
        let parent_probe: &v1::TestParentProbe = archived_test_parent.as_probe();

        assert!(parent_probe.test().is_some());
        assert_eq!(parent_probe.parent_a(), Some(&12));

        let test_probe = parent_probe.test().unwrap().as_probe();

        assert_eq!(test_probe.probe_as::<v1::TestEv0>(), Some(&v1::ArchivedTestEv0::new(1, 2)));
        assert_eq!(test_probe.probe_as::<v1::TestEv1>(), Some(&v1::ArchivedTestEv1::new(1, 2, 3)));
        assert_eq!(test_probe.a(), Some(&1));
        assert_eq!(test_probe.b(), Some(&2));
        assert_eq!(test_probe.c(), Some(&3));
    }

    #[test]
    fn nested_advanced_evolution_backwards_compat_2() {
        #[derive(Archive, Serialize)]
        struct ContainerV3 {
            #[with(Evolve)]
            test_parent: v3::TestParent,
        }

        #[derive(Archive, Serialize)]
        struct ContainerV4 {
            #[with(Evolve)]
            test_parent: v4::TestParent,
        }

        let container_v3 = ContainerV3 {
            test_parent: v3::TestParent {
                test: v3::Test {
                    a: 1,
                    b: 2,
                    c: 3,
                    d: 4,
                    e: String::from("test string"),
                    f: 5
                },
                parent_a: 12,
                parent_b: 13,
                parent_c: vec![0, 1, 2, 3, 10, 11, 12],
                parent_d: String::from("parent test string"),
                parent_e: 14,
            }
        };

        // producer is on v3, serializes a v3
        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container_v3).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();


        // consumer is on v4, accesses v3 archive as v4
        let archived_container: &Archived<ContainerV4> = unsafe { archived_root::<ContainerV4>(&buf) };
        let archived_test_parent: &protoss::ArchivedEvolution<v4::TestParent> = &archived_container.test_parent;

        // v4 probe from v3 archived data
        let parent_probe: &v4::TestParentProbe = archived_test_parent.as_probe();

        assert!(parent_probe.test().is_some());
        assert_eq!(parent_probe.parent_a(), Some(&12));
        assert_eq!(parent_probe.parent_b(), Some(&13));
        assert!(parent_probe.parent_c().is_some());
        assert_eq!(parent_probe.parent_c().unwrap(), &[0, 1, 2, 3, 10, 11, 12]);
        assert!(parent_probe.parent_d().is_some());
        assert_eq!(parent_probe.parent_d().unwrap(), "parent test string");
        assert_eq!(parent_probe.parent_e(), Some(&14));

        let test_probe = parent_probe.test().unwrap().as_probe();

        assert_eq!(test_probe.a(), Some(&1));
        assert_eq!(test_probe.b(), Some(&2));
        assert_eq!(test_probe.c(), Some(&3));
        assert_eq!(test_probe.d(), Some(&4));
        assert!(test_probe.e().is_some());
        assert_eq!(test_probe.e().unwrap(), "test string");
        assert_eq!(test_probe.f(), Some(&5));
        assert_eq!(test_probe.g(), None);
        assert_eq!(test_probe.h(), None);
    }

    #[test]
    fn nested_advanced_evolution_forwards_compat_2() {
        #[derive(Archive, Serialize)]
        struct ContainerV3 {
            #[with(Evolve)]
            test_parent: v3::TestParent,
        }

        #[derive(Archive, Serialize)]
        struct ContainerV4 {
            #[with(Evolve)]
            test_parent: v4::TestParent,
        }

        let container_v4 = ContainerV4 {
            test_parent: v4::TestParent {
                test: v4::Test {
                    a: 1,
                    b: 2,
                    c: 3,
                    d: 4,
                    e: String::from("test string"),
                    f: 5,
                    g: vec![String::from("test1"), String::from("test2"), String::from("test3")],
                    h: 6,
                },
                parent_a: 12,
                parent_b: 13,
                parent_c: vec![0, 1, 2, 3, 10, 11, 12],
                parent_d: String::from("parent test string"),
                parent_e: 14,
            }
        };

        // producer is on v4, serializes v4
        let mut serializer = DefaultSerializer::default();
        serializer.serialize_value(&container_v4).unwrap();
        let buf: AlignedVec = serializer.into_serializer().into_inner();


        // consumer is on v3, accesses v4-serialized archive as v3
        let archived_container: &Archived<ContainerV3> = unsafe { archived_root::<ContainerV3>(&buf) };
        let archived_test_parent: &protoss::ArchivedEvolution<v3::TestParent> = &archived_container.test_parent;

        // v3 probe from v4 archived data
        let parent_probe: &v3::TestParentProbe = archived_test_parent.as_probe();

        assert!(parent_probe.test().is_some());
        assert_eq!(parent_probe.parent_a(), Some(&12));
        assert_eq!(parent_probe.parent_b(), Some(&13));
        assert!(parent_probe.parent_c().is_some());
        assert_eq!(parent_probe.parent_c().unwrap(), &[0, 1, 2, 3, 10, 11, 12]);
        assert!(parent_probe.parent_d().is_some());
        assert_eq!(parent_probe.parent_d().unwrap(), "parent test string");
        assert_eq!(parent_probe.parent_e(), Some(&14));

        let test_probe = parent_probe.test().unwrap().as_probe();

        assert_eq!(test_probe.a(), Some(&1));
        assert_eq!(test_probe.b(), Some(&2));
        assert_eq!(test_probe.c(), Some(&3));
        assert_eq!(test_probe.d(), Some(&4));
        assert!(test_probe.e().is_some());
        assert_eq!(test_probe.e().unwrap(), "test string");
        assert_eq!(test_probe.f(), Some(&5));
        // doesn't compile because v3 probe doesn't know about g and h!
        // assert_eq!(test_probe.g(), None);
        // assert_eq!(test_probe.h(), None);
    }

}
