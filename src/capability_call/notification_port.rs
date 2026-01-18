use crate::types::*;

#[repr(usize)]
pub enum OperationType {
    None,
    Notify,
    Wait,
    Poll,
    Identify,
}
