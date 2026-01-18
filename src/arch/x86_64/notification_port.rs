use crate::capability_call::notification_port;
use crate::types::*;
use core::arch::asm;

#[inline(always)]
pub fn notify(descriptor: CapabilityDescriptor) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = notification_port::OperationType::Notify as Word;
    let mut a2 = 0usize; // flag word (return value)

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // operation  -> capability_error
        out("r8") a2, // identifier
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        options(nomem),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn wait(descriptor: CapabilityDescriptor) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = notification_port::OperationType::Wait as Word;
    let mut a2 = 0usize; // flag word (return value)

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // operation  -> capability_error
        out("r8") a2, // identifier
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        options(nomem),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn poll(descriptor: CapabilityDescriptor) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = notification_port::OperationType::Poll as Word;
    let mut a2 = 0usize; // flag word (return value)

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // operation  -> capability_error
        out("r8") a2, // identifier
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        options(nomem),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn identify(descriptor: CapabilityDescriptor, new_identifier: Word) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = notification_port::OperationType::Identify as Word;

    let mut a2 = new_identifier;

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // operation  -> capability_error
        in("r8")     a2,       // identifier
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
