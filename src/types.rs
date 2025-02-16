pub use core::result;

// common types
pub type Word = usize;
pub type Sword = isize;
pub type PhysicalAddress = Word;
pub type VirtualAddress = Word;

#[repr(isize)]
pub enum KernelCallType {
    CapabilityCall = -1,
    Yield = -2,
    DebugCall = -3,
}

// capability types

#[repr(usize)]
pub enum CapabilityType {
    // reserved
    None,
    Debug,

    // composite
    Node,

    // memory
    Generic,
    AddressSpace, // alias of root page table
    PageTable,
    Frame,

    // process
    ProcessControlBlock,

    // communication
    IpcPort,
    NotificationPort,

    // driver
    InterruptRegion,
    InterruptPort,
    IoPort,

    // virtualization
    VirtualCpu,
    VirtualAddressSpace, // alias of root virtual page table
    VirtualPageTable,
}

#[derive(Debug)]
#[repr(usize)]
pub enum CapabilityError {
    IllegalOperation,
    PermissionDenied,
    InvalidDescriptor,
    InvalidDepth,
    InvalidArgument,
    Fatal,
    DebugUnimplemented,
}

pub type CapabilityDescriptor = Word;
pub type CapabilityResult = result::Result<(), CapabilityError>;

pub trait AsCapabilityDescriptor {
    fn as_descriptor(&self) -> CapabilityDescriptor;
}

#[inline(always)]
pub fn convert_capability_result(a0: Word, a1: Word) -> CapabilityResult {
    if a0 == 0 {
        Err(match a1 {
            0 => CapabilityError::IllegalOperation,
            1 => CapabilityError::PermissionDenied,
            2 => CapabilityError::InvalidDescriptor,
            3 => CapabilityError::InvalidDepth,
            4 => CapabilityError::InvalidArgument,
            5 => CapabilityError::Fatal,
            6 => CapabilityError::DebugUnimplemented,
            _ => CapabilityError::DebugUnimplemented,
        })
    } else {
        Ok(())
    }
}

#[repr(usize)]
pub enum CapabilityRights {
    None = 0,
    Read = 1 << 0,
    Write = 1 << 1,
    Copy = 1 << 2,
    Modify = 1 << 3,
    All = CapabilityRights::Read as usize
        | CapabilityRights::Write as usize
        | CapabilityRights::Copy as usize
        | CapabilityRights::Modify as usize,
}

type CapabilityIdentifier = Word;

// constants
pub const BYTE_SIZE: usize = 1;
pub const WORD_SIZE: usize = core::mem::size_of::<Word>();
pub const PAGE_SIZE: usize = 4096;
pub const BYTE_BITS: usize = 8;
pub const WORD_BITS: usize = WORD_SIZE * BYTE_BITS;
