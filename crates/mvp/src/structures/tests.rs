use crate::Structure;

#[test]
fn empty_structure() {
    assert_eq!(Structure::EMPTY.change(&mut []), Structure::EMPTY);
}
