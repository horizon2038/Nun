use crate::types::CapabilityResult;
use crate::types::*;

#[repr(usize)]
pub enum OperationType {
    None,
    Copy,
    Move,
    Mint,
    Demote,
    Revoke,
    Remove,
}
