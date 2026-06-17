//! Extension traits and implementations [AbstractExt], [ObjectExt], [PropertyExt], and [StructureExt].

mod abstracts;
mod composites;
mod iter;
mod objects;
mod properties;

pub use abstracts::*;
pub use composites::*;
pub use objects::*;
pub use properties::*;

use everything_objects::Object;

#[derive(Clone, Debug, PartialEq)]
pub struct Statement {
    pub subject: Object,
    pub tag: Object,
    pub value: Object,
}
