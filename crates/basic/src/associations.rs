use crate::PackedId;

pub struct Association {
    pub(crate) target: PackedId,
    pub(crate) tag: PackedId,
    pub(crate) value: PackedId,
}
