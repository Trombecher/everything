use crate::{base::BASE, ext::StructureExt};
use std::assert_matches;

#[test]
fn base_is_knowledge() {
    assert_matches!(BASE.is_knowledge(), Ok(_));
}
