use core::arch::asm;

use crate::capability_call::interrupt_port;
use crate::types::*;

#[inline(always)]
pub fn bind(
    descriptor: CapabilityDescriptor,
    target_notification_port: CapabilityDescriptor,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = interrupt_port::OperationType::Bind as Word;
    let mut a2 = target_notification_port as Word;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // operation  -> capability_error
        in("r8")     a2,       // target_notification_port
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn unbind(descriptor: CapabilityDescriptor) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = interrupt_port::OperationType::Unbind as Word;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // operation  -> capability_error
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn ack(descriptor: CapabilityDescriptor) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = interrupt_port::OperationType::Ack as Word;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // operation  -> capability_error
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

pub fn get_irq_number(descriptor: CapabilityDescriptor) -> Result<Word, CapabilityError> {
    let mut a0 = descriptor;
    let mut a1 = interrupt_port::OperationType::GetIrqNumber as Word;
    let mut a2 = 0usize; // irq_number (return value)

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // operation  -> capability_error
        out("rdx")     a2,      // irq_number
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    match convert_capability_result(a0, a1) {
        Ok(()) => Ok(a2),
        Err(e) => Err(e),
    }
}
