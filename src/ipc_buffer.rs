use crate::types::*;

pub const IPC_BUFFER_SIZE: usize = (PAGE_SIZE / core::mem::size_of::<Word>()) - (16 + 2);
pub const TLS_BASE_OFFSET: usize = IPC_BUFFER_SIZE - 4;
pub const PORT_BASE_OFFSET: usize = IPC_BUFFER_SIZE - 1;

#[repr(C, packed)]
pub struct IpcBuffer {
    pub messages: [Word; IPC_BUFFER_SIZE],
    pub transfer_destination_node: CapabilityDescriptor,
    pub transfer_destination_index: Word,
}

impl IpcBuffer {
    pub fn get_message(&self, index: Word) -> Word {
        self.messages[index]
    }

    pub fn configure_message(&mut self, index: Word, value: Word) {
        self.messages[index] = value
    }
}
