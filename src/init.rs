use crate::ipc_buffer::*;
use crate::types::*;

#[repr(C)]
pub struct GenericDescriptor {
    pub address: PhysicalAddress,
    pub size_radix: u8,
    pub is_device: bool,
}

#[repr(C)]
pub struct InterruptPortDescriptor {
    irq_number: Word,
}

#[repr(C)]
pub struct InitInfo {
    // kernel description
    pub kernel_version: Word,

    pub arch_info: [Word; 8],

    // ipc buffer
    pub ipc_buffer: *mut IpcBuffer,
    pub frame_ipc_buffer: CapabilityDescriptor,

    // initial generics
    pub generic_list: [GenericDescriptor; 128],
    pub generic_start: CapabilityDescriptor,
    pub generic_list_count: Word,

    // initial interrupt ports
    pub interrupt_port_list: [InterruptPortDescriptor; 128],
}

impl InitInfo {
    pub fn get_generic_descriptor_from_index(&self, index: Word) -> Option<CapabilityDescriptor> {
        if index >= self.generic_list_count {
            return None;
        }

        let common_offset: Word = WORD_BITS - BYTE_BITS;
        let base_descriptor: Word = InitSlotOffset::GenericNode.as_descriptor();

        // the current base has depth set for node
        // therefore, it is necessary to reset the depth
        let mask: Word = !(((1 << BYTE_BITS) - 1) << common_offset);

        Some(
            (base_descriptor & mask)
                | (common_offset << common_offset)
                | (index << common_offset.saturating_sub(8 + 7)), // root node + generic node
        )
        // 8 : initial node index
    }
}

pub enum InitSlotOffset {
    Reserved = 0,
    ProcessControlBlock = 1,
    ProcessAddressSpace = 2,
    ProcessRootNode = 3, // recursive !
    ProcessPageTableNode = 4,
    ProcessFrameNode = 5,
    ProcessIpcBufferFrame = 6,
    GenericNode = 7,
    InterruptRegion = 8,
    IoPort = 9,
}

impl AsCapabilityDescriptor for InitSlotOffset {
    #[inline(always)]
    fn as_descriptor(&self) -> CapabilityDescriptor {
        const PROCESS_ROOT_NODE_RADIX: Word = 8;
        const PROCESS_PAGE_TABLE_NODE_RADIX: Word = 7;
        const PROCESS_FRAME_NODE_RADIX: Word = 7;
        const GENERIC_NODE_RADIX: Word = 7;

        let common_offset_bit = WORD_BITS - BYTE_BITS;
        let base_descriptor = PROCESS_ROOT_NODE_RADIX << common_offset_bit; // with depth (msb 8 bits)
        let node_offset_bit = common_offset_bit.saturating_sub(PROCESS_ROOT_NODE_RADIX);

        match self {
            InitSlotOffset::Reserved => 0,
            InitSlotOffset::ProcessControlBlock => {
                base_descriptor | ((InitSlotOffset::ProcessControlBlock as Word) << node_offset_bit)
            }
            InitSlotOffset::ProcessAddressSpace => {
                base_descriptor | ((InitSlotOffset::ProcessAddressSpace as Word) << node_offset_bit)
            }
            InitSlotOffset::ProcessRootNode => {
                base_descriptor | ((InitSlotOffset::ProcessRootNode as Word) << node_offset_bit)
            }
            InitSlotOffset::ProcessPageTableNode => {
                base_descriptor
                    | ((InitSlotOffset::ProcessPageTableNode as Word) << node_offset_bit)
            }
            InitSlotOffset::ProcessFrameNode => {
                base_descriptor | ((InitSlotOffset::ProcessFrameNode as Word) << node_offset_bit)
            }
            InitSlotOffset::ProcessIpcBufferFrame => {
                base_descriptor
                    | ((InitSlotOffset::ProcessIpcBufferFrame as Word) << node_offset_bit)
            }
            InitSlotOffset::GenericNode => {
                base_descriptor | ((InitSlotOffset::GenericNode as Word) << node_offset_bit)
            }
            InitSlotOffset::InterruptRegion => {
                base_descriptor | ((InitSlotOffset::InterruptRegion as Word) << node_offset_bit)
            }
            InitSlotOffset::IoPort => {
                base_descriptor | ((InitSlotOffset::IoPort as Word) << node_offset_bit)
            }
        }
    }
}
