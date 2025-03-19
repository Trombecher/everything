#![cfg(test)]

use super::*;

#[test]
fn value_true() {
    assert_eq!(
        EncodedValue::new(&[ff::value::TRUE]).map(|x| x.decode()),
        Some(Value::True),
    )
}