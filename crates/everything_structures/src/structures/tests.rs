#[allow(non_snake_case)]
mod Structure {
    use std::sync::Arc;

    use super::super::*;

    const ALICE: Object = Object::Abstract(Abstract(5486954));
    const BOB: Object = Object::Abstract(Abstract(5486955));

    #[test]
    fn union() {
        let a = Structure::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        let b = Structure::new(&mut [Property {
            tag: BOB,
            value: ALICE,
        }]);

        assert_eq!(
            a.union(&b),
            Structure::new(&mut [
                Property {
                    tag: ALICE,
                    value: BOB
                },
                Property {
                    tag: BOB,
                    value: ALICE
                }
            ])
        );
    }

    #[test]
    fn integer_specialization() {
        for i in (-259..1000_i128).filter_map(|x| NonZeroI128::new(x * 7)) {
            assert_eq!(
                Structure::Integer(i),
                Structure::new(&mut [Property::new_integer(i)])
            );
        }
    }

    #[test]
    fn subset() {
        let structure = Structure::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        assert!(structure.is_subset_of(&structure.add(&mut [Property {
            tag: ALICE,
            value: ALICE
        }])));

        assert!(!structure.is_subset_of(&Structure::Empty));
    }

    #[test]
    fn no_values() {
        assert_eq!(Structure::Empty.values(ALICE).next(), None)
    }

    #[test]
    fn no_tags() {
        assert_eq!(Structure::Empty.tags(ALICE).next(), None);
    }

    #[test]
    fn one_tag() {
        let s = Structure::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        let mut tags_that_have_bob = s.tags(BOB);
        assert_eq!(tags_that_have_bob.next(), Some(ALICE));
        assert_eq!(tags_that_have_bob.next(), None);
        assert_eq!(tags_that_have_bob.next(), None);

        let mut tags_that_have_alice = s.tags(ALICE);

        assert_eq!(tags_that_have_alice.next(), None);
        assert_eq!(tags_that_have_alice.next(), None);
        assert_eq!(tags_that_have_alice.next(), None);
    }

    #[test]
    fn debug_fmt() {
        let abstr = Object::Abstract(Abstract(34985));
        let empty = Object::Structure(Structure::Empty);
        let byte = Object::Structure(Structure::Byte(Byte(0xA1)));
        let c = Object::Structure(Structure::Character('ß'));

        let structure = Structure::new(&mut [
            Property {
                tag: abstr.clone(),
                value: empty.clone(),
            },
            Property {
                tag: byte.clone(),
                value: c.clone(),
            },
        ]);

        assert_eq!(
            format!("{structure:?}"),
            format!("{{({abstr:?}, {empty:?}), ({byte:?}, {c:?})}}")
        );
        // TODO: more
    }

    #[test]
    fn remove_props() {
        let structure = Structure::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        let should_be_empty = structure.remove(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        assert_eq!(should_be_empty, []);
    }

    /// This test assures that structurally identical objects
    /// will have the same allocation.
    #[test]
    fn deduping() {
        let structure_a = Structure::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);
        let structure_b = Structure::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        assert!(Arc::ptr_eq(
            &structure_a.any().unwrap().properties,
            &structure_b.any().unwrap().properties
        ))
    }

    #[test]
    fn one_value() {
        let s = Structure::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        let mut alices = s.values(ALICE);
        assert_eq!(alices.next(), Some(BOB));
        assert_eq!(alices.next(), None);

        assert_eq!(s.values(BOB).next(), None);
    }

    #[test]
    fn multiple_values() {
        let s = Structure::new(&mut [
            Property {
                tag: ALICE,
                value: ALICE,
            },
            Property {
                tag: ALICE,
                value: BOB,
            },
        ]);

        let mut alices = s.values(ALICE);
        assert_eq!(alices.next(), Some(ALICE));
        assert_eq!(alices.next(), Some(BOB));
        assert_eq!(alices.next(), None);
    }

    #[test]
    fn multiple_tags() {
        let s = Structure::new(&mut [
            Property {
                tag: ALICE,
                value: ALICE,
            },
            Property {
                tag: BOB,
                value: ALICE,
            },
        ]);

        let mut tags_that_have_alice = s.tags(ALICE);
        assert_eq!(tags_that_have_alice.next(), Some(ALICE));
        assert_eq!(tags_that_have_alice.next(), Some(BOB));
        assert_eq!(tags_that_have_alice.next(), None);
    }
}
