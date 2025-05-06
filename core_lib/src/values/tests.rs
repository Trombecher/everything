#![cfg(test)]

use super::*;

#[test]
fn value_true() {
    assert_eq!(
        EncodedValue::new(&[ff::INTEGER, 15, 0, 0, 0]).map(|x| x.decode()),
        Some(Value::Integer(15)),
    )
}
