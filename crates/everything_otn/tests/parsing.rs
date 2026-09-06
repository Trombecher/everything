use std::num::NonZeroI128;

use everything_objects::{
    Abstract, Byte, BytesComposite, Composite, Object, Property, TextComposite,
};
use everything_otn::Parsable;

#[test]
fn main() {
    assert_eq!(
        Composite::parse("{(@1, @2)}"),
        Ok(Composite::new(&mut [Property {
            tag: Object::Abstract(Abstract(1)),
            value: Object::Abstract(Abstract(2)),
        }]))
    );

    assert_eq!(
        Composite::parse(
            "{
                (@1, @245654),
                ('a', {(\"abccä10543ß'ä@20{}\", 64381094849683401)})
                (-20, {(X1069AAEF, x6F)})
            }"
        ),
        Ok(Composite::new(&mut [
            Property {
                tag: Abstract(1).into(),
                value: Abstract(245654).into(),
            },
            Property {
                tag: Composite::Character('a').into(),
                value: Composite::new(&mut [Property {
                    tag: Composite::Text(TextComposite::new("abccä10543ß'ä@20{}").unwrap()).into(),
                    value: Composite::Integer(NonZeroI128::new(64381094849683401).unwrap()).into()
                }])
                .into()
            },
            Property {
                tag: Composite::Integer(NonZeroI128::new(-20).unwrap()).into(),
                value: Composite::new(&mut [Property {
                    tag: Composite::Bytes(BytesComposite::new(&[0x10, 0x69, 0xAA, 0xEF]).unwrap())
                        .into(),
                    value: Composite::Byte(Byte(0x6F_u8)).into()
                }])
                .into()
            }
        ]))
    );
}
