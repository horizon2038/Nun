use core::arch::asm;

use crate::capability_call::ipc_port;
use crate::types::*;

#[inline(always)]
fn execute_ipc(
    descriptor: CapabilityDescriptor,
    operation: ipc_port::OperationType,
    info: &mut ipc_port::MessageInfo,
    identifier: &mut Word,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = operation as Word;

    let mut a2 = Word::try_from(info.data).unwrap_or(0);
    let mut a3 = 0; // identifier

    // get messages from ipc buffer
    // let mut a4 = *arg4;
    // let mut a5 = *arg5;
    // let mut a6 = *arg6;
    // let mut a7 = *arg7;
    // let mut a8 = *arg8;

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // oepration  -> capablity_error
        inout("r8")  a2 => a2, // info
        out("r9")    a3, // identifier (receive)
        // inout("r10") a4 => a4,
        // inout("r12") a5 => a5,
        // inout("r13") a6 => a6,
        // inout("r14") a7 => a7,
        // inout("r15") a8 => a8,
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    *info = ipc_port::MessageInfo::from(a2);
    *identifier = a3;

    // restore messages to ipc buffer
    // *arg4 = a4;
    // *arg5 = a5;
    // *arg6 = a6;
    // *arg7 = a7;
    // *arg8 = a8;

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
pub fn reply(info: ipc_port::MessageInfo) -> CapabilityResult {
    execute_ipc(
        0, // descriptor is not used (reply)
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
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // oepration  -> capablity_error
        in("r8")     a2,       // identifier
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
