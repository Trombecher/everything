#![cfg(test)]

use super::*;

#[test]
fn has() {
    let features = Features::none() + Feature::Bin;
    assert!(features.has(Feature::Bin));
    assert!(!features.has(Feature::FileTypes));
    assert!(!features.has(Feature::NodeCount));
    assert!(!features.has(Feature::Naming));
    assert!(!features.has(Feature::Favourites));
    assert!(!features.has(Feature::FileSystem));
    assert!(!features.has(Feature::Images));
    assert!(!features.has(Feature::Internationalization));
    assert!(!features.has(Feature::Creation));
    assert!(!features.has(Feature::TemporaryObjects));
}

#[test]
fn has_implication() {
    let features = Features::none() + Feature::Images;
    assert!(!features.has(Feature::Bin));
    assert!(!features.has(Feature::FileTypes));
    assert!(!features.has(Feature::NodeCount));
    assert!(features.has(Feature::Naming));
    assert!(!features.has(Feature::Favourites));
    assert!(features.has(Feature::FileSystem));
    assert!(features.has(Feature::Images));
    assert!(!features.has(Feature::Internationalization));
    assert!(!features.has(Feature::Creation));
    assert!(!features.has(Feature::TemporaryObjects));
}

#[test]
fn iter() {
    let features = Features::none()
        + Feature::Bin
        + Feature::FileTypes;
    
    let mut iter = features.into_iter();

    assert_eq!(iter.next(), Some(Feature::Bin));
    assert_eq!(iter.next(), Some(Feature::Naming));
    assert_eq!(iter.next(), Some(Feature::FileSystem));
    assert_eq!(iter.next(), Some(Feature::FileTypes));
    assert_eq!(iter.next(), None);
}