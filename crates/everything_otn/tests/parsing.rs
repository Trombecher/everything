use std::num::NonZeroI128;

use everything_structures::{
    Abstract, Byte, BytesStructure, Object, Property, Structure, TextStructure,
};
use everything_structures_ff::Parsable;

#[test]
fn main() {
    assert_eq!(
        Structure::parse("{(@1, @2)}"),
        Ok(Structure::new(&mut [Property {
            tag: Object::Abstract(Abstract(1)),
            value: Object::Abstract(Abstract(2)),
        }]))
    );

    assert_eq!(
        Structure::parse(
            "{
                (@1, @245654),
                ('a', {(\"abccä10543ß'ä@20{}\", 64381094849683401)})
                (-20, {(X1069AAEF, x6F)})
            }"
        ),
        Ok(Structure::new(&mut [
            Property {
                tag: Abstract(1).into(),
                value: Abstract(245654).into(),
            },
            Property {
                tag: Structure::Character('a').into(),
                value: Structure::new(&mut [Property {
                    tag: Structure::Text(TextStructure::new("abccä10543ß'ä@20{}").unwrap()).into(),
                    value: Structure::Integer(NonZeroI128::new(64381094849683401).unwrap()).into()
                }])
                .into()
            },
            Property {
                tag: Structure::Integer(NonZeroI128::new(-20).unwrap()).into(),
                value: Structure::new(&mut [Property {
                    tag: Structure::Bytes(BytesStructure::new(&[0x10, 0x69, 0xAA, 0xEF]).unwrap())
                        .into(),
                    value: Structure::Byte(Byte(0x6F_u8)).into()
                }])
                .into()
            }
        ]))
    );
}
