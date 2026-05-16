use crate::{Abstract, Object};

pub const ALICE: Object = Object::Abstract(Abstract(u128::from_be_bytes(*b"This is Alice!!!")));
pub const BOB: Object = Object::Abstract(Abstract(u128::from_be_bytes(*b"This is Bob!!!!!")));

#[allow(non_snake_case)]
mod AnyStructure {
    use crate::{
        Abstract, Structure,
        structures::any::tests::{ALICE, BOB},
    };

    use super::super::*;

    fn alice_bob_structure() -> AnyStructure {
        match Structure::new(&mut [Property {
            tag: ALICE,
            value: BOB,
        }]) {
            Structure::Any(any) => any,
            _ => unreachable!(),
        }
    }

    #[test]
    fn new() {
        const PROP: Property = Property {
            tag: Object::Abstract(Abstract(654987123984)),
            value: Object::Abstract(Abstract(543020512)),
        };

        let s = match Structure::new(&mut [PROP]) {
            Structure::Any(any) => any,
            _ => unreachable!("structure is not any"),
        };

        assert_eq!(s.properties.as_ref(), &[PROP]);
    }

    #[test]
    fn inner_structure() {
        let inner = alice_bob_structure();

        let outer = Structure::new(&mut [Property {
            tag: ALICE,
            value: Structure::Any(inner.clone()).into(),
        }]);

        assert_eq!(
            outer,
            [Property {
                tag: ALICE,
                value: Structure::Any(inner).into()
            }]
        )
    }

    #[test]
    fn has() {
        let structure = alice_bob_structure();

        assert!(structure.has(&ALICE, &BOB));
        assert!(!structure.has(&ALICE, &ALICE));
    }
}

#[allow(non_snake_case)]
mod AnyStructureValues {
    use crate::{Abstract, Structure};

    use super::super::*;

    #[test]
    fn next() {
        let structure = match Structure::new(&mut [Property {
            tag: Abstract::BIT_0.into(),
            value: Abstract(543539).into(),
        }]) {
            Structure::Any(any) => any,
            _ => unreachable!(),
        };

        let mut values = AnyStructureValues {
            done: true,
            tag: Abstract::BIT_0.into(),
            properties: AnyStructureProperties {
                subject: structure,
                index: 0,
            },
        };

        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
        assert_eq!(values.next(), None);
    }
}
