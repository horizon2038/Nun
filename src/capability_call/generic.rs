use crate::types::CapabilityResult;
use crate::types::*;

/*
enum operation_index : a9n::word
{
    RESERVED = 0,   // descriptor
    OPERATION_TYPE, // tag
    CAPABILITY_TYPE,
    CAPABILITY_SPECIFIC_BITS,
    CAPABILITY_COUNT,
    ROOT_DESCRIPTOR,
    SLOT_INDEX,
};
*/

#[repr(usize)]
pub enum OperationType {
    Convert, // TODO: implement "None"
}
