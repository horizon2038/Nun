use core::arch::asm;

use crate::capability_call::ipc_port;
use crate::types::*;

use crate::arch::ipc_buffer;

const HARDWARE_MR_COUNT_MAX: usize = 10;
const RESERVED_MR_COUNT: usize = 4;
const USABLE_HARDWARE_MR_COUNT_MAX: usize = HARDWARE_MR_COUNT_MAX - RESERVED_MR_COUNT;

#[inline(always)]
fn execute_ipc(
    descriptor: CapabilityDescriptor,
    operation: ipc_port::OperationType,
    info: &mut ipc_port::MessageInfo,
    identifier: &mut Word,
) -> CapabilityResult {
    // for result
    let mut a0 = descriptor;
    let mut a1 = operation as Word;

    let mut ipc_buffer = ipc_buffer::get_ipc_buffer();

    let mut a2 = Word::try_from(info.data).unwrap_or(0);
    let mut a3 = 0; // identifier

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
        inout("rdx") a2 => a2, // info
        out("r8")    a3,       // identifier (receive)
        inout("r9") a4 => a4,
        inout("r10") a5 => a5,
        inout("r12") a6 => a6,
        inout("r13") a7 => a7,
        inout("r14") a8 => a8,
        inout("r15") a9 => a9,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    *info = ipc_port::MessageInfo::from(a2);
    *identifier = a3;

    // restore messages to ipc buffer
    ipc_buffer.configure_message(4, a4);
    ipc_buffer.configure_message(5, a5);
    ipc_buffer.configure_message(6, a6);
    ipc_buffer.configure_message(7, a7);
    ipc_buffer.configure_message(8, a8);
    ipc_buffer.configure_message(9, a9);

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn send(target: CapabilityDescriptor, info: ipc_port::MessageInfo) -> CapabilityResult {
    execute_ipc(
        target,
        ipc_port::OperationType::Send,
        &mut info.clone(), // only input
        &mut 0,            // only input
    )
}

#[inline(always)]
pub fn receive(
    target: CapabilityDescriptor,
    info: &mut ipc_port::MessageInfo,
    identifier: &mut Word,
) -> CapabilityResult {
    execute_ipc(target, ipc_port::OperationType::Receive, info, identifier)
}

#[inline(always)]
pub fn call(
    target: CapabilityDescriptor,
    info: &mut ipc_port::MessageInfo,
    identifier: &mut Word,
) -> CapabilityResult {
    execute_ipc(target, ipc_port::OperationType::Call, info, identifier)
}

#[inline(always)]
pub fn reply(target: CapabilityDescriptor, info: ipc_port::MessageInfo) -> CapabilityResult {
    execute_ipc(
        target,
        ipc_port::OperationType::Reply,
        &mut info.clone(), // only input
        &mut 0,
    )
}

#[inline(always)]
pub fn reply_receive(
    target: CapabilityDescriptor, // used for receive
    info: &mut ipc_port::MessageInfo,
    identifier: &mut Word,
) -> CapabilityResult {
    execute_ipc(
        target,
        ipc_port::OperationType::ReplyReceive,
        info,
        identifier,
    )
}

#[inline(always)]
pub fn identify(descriptor: CapabilityDescriptor, new_identifier: Word) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = ipc_port::OperationType::Identify as Word;

    let mut a2 = new_identifier;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        in("rdx")     a2,      // identifier
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
