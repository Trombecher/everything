//! Extension traits and implementations [AbstractExt], [ObjectExt], [PropertyExt], and [StructureExt].

mod abstracts;
mod iter;
mod objects;
mod properties;
mod structures;

pub use abstracts::*;
pub use objects::*;
pub use properties::*;
pub use structures::*;

use everything_structures::Object;

#[derive(Clone, Debug, PartialEq)]
pub struct Statement {
    pub subject: Object,
    pub tag: Object,
    pub value: Object,
}
