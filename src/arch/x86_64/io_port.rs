use core::arch::asm;

use crate::capability_call::io_port;
use crate::types::*;

#[inline(always)]
pub fn read(target: CapabilityDescriptor, address: Word, data: &mut Word) -> CapabilityResult {
    let mut a0 = target;
    let mut a1 = io_port::OperationType::Read as Word;
    let mut a2 = address;

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // oepration  -> capablity_error
        inout("r8")  a2 => a2, // address
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    *data = a2;

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn write(target: CapabilityDescriptor, address: Word, data: Word) -> CapabilityResult {
    let mut a0 = target;
    let mut a1 = io_port::OperationType::Write as Word;
    let mut a2 = address;
    let mut a3 = data;

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // oepration  -> capablity_error
        in("r8")  a2,          // address
        in("r9")  a3,        // data
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
