/// A value encoded as a row. Every `[u64; 2]` is a valid value.
#[derive(Copy, Clone, Debug)]
pub struct ValueRow(pub [u64; 2]);