use crate::types::*;

#[repr(usize)]
pub enum OperationType {
    None,
    Map,
    Unmap,
    GetUnsetDepth,
}
