use core::arch::asm;

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
    let mut a0 = descriptor;
    let mut a1 = process_control_block::OperationType::Configure as Word;

    let mut a2 = configuration_info.data;
    let mut a3 = address_space_descriptor as Word;
    let mut a4 = root_node_descriptor as Word;
    let mut a5 = frame_ipc_buffer_descriptor as Word;
    let mut a6 = notification_port as Word;
    let mut a7 = ipc_port_resolver as Word;
    let mut a8 = instruction_pointer as Word;

    // TODO: implement this:
    // configure_message_register(9, stack_pointer);
    // configure_message_register(10, thread_local_base);
    // configure_message_register(11, priority);
    // configure_message_register(12, affinity);

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // oepration  -> capablity_error
        in("r8")     a2,       // info
        in("r9")     a3,       // address_space_descriptor
        in("r10")    a4,       // root_node_descriptor
        in("r12")    a5,       // frame_ipc_buffer_descriptor
        in("r13")    a6,       // notification_port
        in("r14")    a7,       // ipc_port_resolver
        in("r15")    a8,       // instruction_pointer
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn read_register() -> CapabilityResult {
    // TODO: implement this
    Ok(())
}

#[inline(always)]
pub fn write_register() -> CapabilityResult {
    // TODO: implement this
    Ok(())
}

#[inline(always)]
pub fn resume(descriptor: CapabilityDescriptor) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = process_control_block::OperationType::Resume as Word;

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // oepration  -> capablity_error
        out("rax") _,
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
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // oepration  -> capablity_error
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
