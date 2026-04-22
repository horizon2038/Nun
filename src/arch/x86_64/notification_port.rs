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
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // operation  -> capability_error
        out("rdx") a2, // identifier
        out("rcx") _,
        out("r11") _,
        options(nostack),
        options(nomem),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn wait(descriptor: CapabilityDescriptor) -> Result<Word, CapabilityError> {
    let mut a0 = descriptor;
    let mut a1 = notification_port::OperationType::Wait as Word;
    let mut a2 = 0usize; // flag word (return value)

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // operation  -> capability_error
        out("rdx") a2, // identifier
        out("rcx") _,
        out("r11") _,
        options(nostack),
        options(nomem),
        );
    }

    match convert_capability_result(a0, a1) {
        Ok(()) => Ok(a2 as Word),
        Err(e) => Err(e),
    }
}

#[inline(always)]
pub fn poll(descriptor: CapabilityDescriptor) -> Result<Word, CapabilityError> {
    let mut a0 = descriptor;
    let mut a1 = notification_port::OperationType::Poll as Word;
    let mut a2 = 0usize; // flag word (return value)

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // operation  -> capability_error
        out("rdx") a2, // identifier
        out("rcx") _,
        out("r11") _,
        options(nostack),
        options(nomem),
        );
    }

    match convert_capability_result(a0, a1) {
        Ok(()) => Ok(a2 as Word),
        Err(e) => Err(e),
    }
}

#[inline(always)]
pub fn identify(descriptor: CapabilityDescriptor, new_identifier: Word) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = notification_port::OperationType::Identify as Word;

    let mut a2 = new_identifier;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // operation  -> capability_error
        in("rdx")     a2,       // identifier
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
