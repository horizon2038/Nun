use crate::types::*;

#[repr(usize)]
pub enum OperationType {
    None,
    Configure,
    ReadRegister,
    WriteRegister,
    Resume,
    Suspend,
}

#[derive(Clone)]
pub struct ConfigurationInfo {
    pub data: Word,
}

impl ConfigurationInfo {
    pub fn new(
        address_space: bool,
        root_node: bool,
        frame_ipc_buffer: bool,
        notification_port: bool,
        ipc_port_resolver: bool,
        instruction_pointer: bool,
        stack_pointer: bool,
        thread_local_base: bool,
        priority: bool,
        affinity: bool,
    ) -> Self {
        Self {
            data: (address_space as Word) << 0
                | (root_node as Word) << 1
                | (frame_ipc_buffer as Word) << 2
                | (notification_port as Word) << 3
                | (ipc_port_resolver as Word) << 4
                | (instruction_pointer as Word) << 5
                | (stack_pointer as Word) << 6
                | (thread_local_base as Word) << 7
                | (priority as Word) << 8
                | (affinity as Word) << 9,
        }
    }

    pub fn is_address_space(&self) -> bool {
        self.data & (1 << 0) != 0
    }

    pub fn is_root_node(&self) -> bool {
        self.data & (1 << 1) != 0
    }

    pub fn is_frame_ipc_buffer(&self) -> bool {
        self.data & (1 << 2) != 0
    }

    pub fn is_notification_port(&self) -> bool {
        self.data & (1 << 3) != 0
    }

    pub fn is_ipc_port_resolver(&self) -> bool {
        self.data & (1 << 4) != 0
    }

    pub fn is_instruction_pointer(&self) -> bool {
        self.data & (1 << 5) != 0
    }

    pub fn is_stack_pointer(&self) -> bool {
        self.data & (1 << 6) != 0
    }

    pub fn is_thread_local_base(&self) -> bool {
        self.data & (1 << 7) != 0
    }

    pub fn is_priority(&self) -> bool {
        self.data & (1 << 8) != 0
    }

    pub fn is_affinity(&self) -> bool {
        self.data & (1 << 9) != 0
    }
}
