use crate::types::*;

#[repr(usize)]
pub enum OperationType {
    None,
    Send,
    Receive,
    Call,
    Reply,
    ReplyReceive,
    Identify,
}

#[derive(Clone)]
pub struct MessageInfo {
    pub data: Word,
}

impl MessageInfo {
    pub fn new(
        is_block: bool,
        message_length: u8,
        transfer_capabiltiy: bool,
        transfer_count: u8,
    ) -> Self {
        Self {
            data: (is_block as Word) << 0
                | ((message_length as Word) & 0xFF) << 1
                | (transfer_capabiltiy as Word) << 9
                | ((transfer_count as Word) & 0b11111) << 10,
        }
    }

    pub fn is_block(&self) -> bool {
        self.data & (1 << 0) != 0
    }

    pub fn message_length(&self) -> u8 {
        ((self.data >> 1) & 0xFF) as u8
    }

    pub fn is_transfer_capability(&self) -> bool {
        self.data & (1 << 9) != 0
    }

    pub fn transfer_count(&self) -> u8 {
        ((self.data >> 10) & 0b11111) as u8
    }

    pub fn is_kernel_message(&self) -> bool {
        self.data & (1 << 15) != 0
    }
}

impl From<Word> for MessageInfo {
    fn from(data: Word) -> MessageInfo {
        MessageInfo { data }
    }
}
