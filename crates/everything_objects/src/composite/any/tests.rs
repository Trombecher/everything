use crate::{Abstract, Object};

pub const ALICE: Object = Object::Abstract(Abstract(u128::from_be_bytes(*b"This is Alice!!!")));
pub const BOB: Object = Object::Abstract(Abstract(u128::from_be_bytes(*b"This is Bob!!!!!")));

#[allow(non_snake_case)]
mod AnyComposite {
    use crate::{Abstract, Composite};

    use super::super::*;
    use super::{ALICE, BOB};

    fn alice_bob_composite() -> AnyComposite {
        match Composite::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]) {
            Composite::Any(any) => any,
            _ => unreachable!(),
        }
    }

    #[test]
    fn new() {
        const PROP: Property = Property {
            tag: Object::Abstract(Abstract(654987123984)),
            value: Object::Abstract(Abstract(543020512)),
        };

        let s = match Composite::new(&mut [PROP]) {
            Composite::Any(any) => any,
            _ => unreachable!("composite is not any"),
        };

        assert_eq!(s.properties.as_ref(), &[PROP]);
    }

    #[test]
    fn inner_composite() {
        let inner = alice_bob_composite();

        let outer = Composite::new(&mut [Property {
            tag: ALICE,
            value: Composite::Any(inner.clone()).into(),
        }]);

        assert_eq!(
            outer,
            [Property {
                tag: ALICE,
                value: Composite::Any(inner).into()
            }]
        )
    }

    #[test]
    fn has() {
        let composite = alice_bob_composite();

        assert!(composite.has(&ALICE, &BOB));
        assert!(!composite.has(&ALICE, &ALICE));
    }
}

#[allow(non_snake_case)]
mod AnyCompositeValues {
    use crate::{Abstract, Composite};

    use super::super::*;

    #[test]
    fn next() {
        let composite = match Composite::new(&mut [Property {
            tag: Abstract::BIT_0.into(),
            value: Abstract(543539).into(),
        }]) {
            Composite::Any(any) => any,
            _ => unreachable!(),
        };

        let mut values = AnyCompositeValues {
            tag: Abstract::BIT_0.into(),
            properties: AnyCompositeProperties {
                subject: composite,
                index: 0,
            },
        };

        assert_eq!(values.next(), Some(Abstract(543539).into()));
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
    }
}
