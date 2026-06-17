#[allow(non_snake_case)]
mod Composite {
    use std::sync::Arc;

    use super::super::*;

    const ALICE: Object = Object::Abstract(Abstract(5486954));
    const BOB: Object = Object::Abstract(Abstract(5486955));

    #[test]
    fn union() {
        let a = Composite::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        let b = Composite::new(&mut [Property {
            tag: BOB,
            value: ALICE,
        }]);

        assert_eq!(
            a.union(&b),
            Composite::new(&mut [
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
                Composite::Integer(i),
                Composite::new(&mut [Property::new_integer(i)])
            );
        }
    }

    #[test]
    fn subset() {
        let Composite = Composite::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        assert!(Composite.is_subset_of(&Composite.add(&mut [Property {
            tag: ALICE,
            value: ALICE
        }])));

        assert!(!Composite.is_subset_of(&Composite::Empty));
    }

    #[test]
    fn no_values() {
        assert_eq!(Composite::Empty.values(ALICE).next(), None)
    }

    #[test]
    fn no_tags() {
        assert_eq!(Composite::Empty.tags(ALICE).next(), None);
    }

    #[test]
    fn one_tag() {
        let s = Composite::new(&mut [Property {
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
        let empty = Object::Composite(Composite::Empty);
        let byte = Object::Composite(Composite::Byte(Byte(0xA1)));
        let c = Object::Composite(Composite::Character('ß'));

        let Composite = Composite::new(&mut [
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
            format!("{Composite:?}"),
            format!("{{({abstr:?}, {empty:?}), ({byte:?}, {c:?})}}")
        );
        // TODO: more
    }

    #[test]
    fn remove_props() {
        let Composite = Composite::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        let should_be_empty = Composite.remove(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        assert_eq!(should_be_empty, []);
    }

    /// This test assures that structurally identical objects
    /// will have the same allocation.
    #[test]
    fn deduping() {
        let Composite_a = Composite::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);
        let Composite_b = Composite::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]);

        assert!(Arc::ptr_eq(
            &Composite_a.any().unwrap().properties,
            &Composite_b.any().unwrap().properties
        ))
    }

    #[test]
    fn one_value() {
        let s = Composite::new(&mut [Property {
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
        let s = Composite::new(&mut [
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
        let s = Composite::new(&mut [
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
