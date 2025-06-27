#![cfg(test)]

use crate::InlineStr;

#[test]
#[should_panic]
fn panic_on_len_mismatch() {
    let _: InlineStr<10> = "Hello, World!".into();
}

#[test]
fn this_should_not_panic() {
    let _: InlineStr<13> = "Hello, World!".into();
}