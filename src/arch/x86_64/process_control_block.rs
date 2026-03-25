use core::arch::asm;

use crate::arch::ipc_buffer;
use crate::capability_call::{ipc_port, process_control_block};
use crate::types::*;

#[inline(always)]
pub fn configure(
    descriptor: CapabilityDescriptor,
    configuration_info: process_control_block::ConfigurationInfo,
    address_space_descriptor: CapabilityDescriptor,
    root_node_descriptor: CapabilityDescriptor,
    frame_ipc_buffer_descriptor: CapabilityDescriptor,
    notification_port: CapabilityDescriptor,
    ipc_port_resolver: CapabilityDescriptor,
    instruction_pointer: VirtualAddress,
    stack_pointer: VirtualAddress,
    thread_local_base: VirtualAddress,
    priority: Word,
    affinity: Word,
) -> CapabilityResult {
    let mut ipc_buffer = ipc_buffer::get_ipc_buffer();

    let mut a0 = descriptor;
    let mut a1 = process_control_block::OperationType::Configure as Word;

    let mut a2 = configuration_info.data;
    let mut a3 = address_space_descriptor as Word;
    let mut a4 = root_node_descriptor as Word;
    let mut a5 = frame_ipc_buffer_descriptor as Word;
    let mut a6 = notification_port as Word;
    let mut a7 = ipc_port_resolver as Word;
    let mut a8 = instruction_pointer as Word;
    let mut a9 = stack_pointer as Word;

    ipc_buffer.configure_message(10, thread_local_base as Word);
    ipc_buffer.configure_message(11, priority);
    ipc_buffer.configure_message(12, affinity);

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        in("rdx")    a2,       // info
        in("r8")     a3,       // address_space_descriptor
        in("r9")     a4,       // root_node_descriptor
        in("r10")    a5,       // frame_ipc_buffer_descriptor
        in("r12")    a6,       // notification_port
        in("r13")    a7,       // ipc_port_resolver
        in("r14")    a8,       // instruction_pointer
        in("r15")    a9,       // stack_pointer
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn read_register(descriptor: CapabilityDescriptor, count: Word) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = process_control_block::OperationType::ReadRegister as Word;

    let mut a2 = count;
    let mut a3 = 0;
    let mut a4 = 0;
    let mut a5 = 0;
    let mut a6 = 0;
    let mut a7 = 0;
    let mut a8 = 0;
    let mut a9 = 0;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        in("rdx")    a2,       // count
        out("r8")     a3,       // register 3
        out("r9")     a4,       // register 4
        out("r10")    a5,       // register 5
        out("r12")    a6,       // register 6
        out("r13")    a7,       // register 7
        out("r14")    a8,       // register 8
        out("r15")    a9,       // register 9
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    let mut ipc_buffer = ipc_buffer::get_ipc_buffer();
    ipc_buffer.configure_message(3, a3);
    ipc_buffer.configure_message(4, a4);
    ipc_buffer.configure_message(5, a5);
    ipc_buffer.configure_message(6, a6);
    ipc_buffer.configure_message(7, a7);
    ipc_buffer.configure_message(8, a8);
    ipc_buffer.configure_message(9, a9);

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn write_register(descriptor: CapabilityDescriptor, count: Word) -> CapabilityResult {
    let mut ipc_buffer = ipc_buffer::get_ipc_buffer();

    let mut a0 = descriptor;
    let mut a1 = process_control_block::OperationType::WriteRegister as Word;

    let mut a2 = count;
    let mut a3 = ipc_buffer.get_message(3);
    let mut a4 = ipc_buffer.get_message(4);
    let mut a5 = ipc_buffer.get_message(5);
    let mut a6 = ipc_buffer.get_message(6);
    let mut a7 = ipc_buffer.get_message(7);
    let mut a8 = ipc_buffer.get_message(8);
    let mut a9 = ipc_buffer.get_message(9);

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        in("rdx")    a2,       // count
        in("r8")     a3,       // register 3
        in("r9")     a4,       // register 4
        in("r10")    a5,       // register 5
        in("r12")    a6,       // register 6
        in("r13")    a7,       // register 7
        in("r14")    a8,       // register 8
        in("r15")    a9,       // register 9
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn resume(descriptor: CapabilityDescriptor) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = process_control_block::OperationType::Resume as Word;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn suspend(descriptor: CapabilityDescriptor) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = process_control_block::OperationType::Suspend as Word;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
