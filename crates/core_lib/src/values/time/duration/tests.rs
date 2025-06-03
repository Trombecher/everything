#![cfg(test)]

use super::*;

#[test]
pub fn conversions() {
    assert_eq!(
        Duration::from_hours(1).as_secs(),
        60 * 60
    );
}