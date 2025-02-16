use crate::types::*;

#[repr(usize)]
pub enum OperationType {
    None,
    Read,
    Write,
    Mint,
}
