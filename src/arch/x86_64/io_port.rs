use core::arch::asm;

use crate::capability_call::io_port;
use crate::types::*;

#[inline(always)]
pub fn read(
    target: CapabilityDescriptor,
    address: Word,
    byte_width: Word,
    data: &mut Word,
) -> CapabilityResult {
    let mut a0 = target;
    let mut a1 = io_port::OperationType::Read as Word;
    let mut a2 = address;
    let mut a3 = byte_width;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        inout("rdx")  a2 => a2, // address
        inout("r8")  a3 => a3, // byte_width
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    *data = a2;

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn write(
    target: CapabilityDescriptor,
    address: Word,
    byte_width: Word,
    data: Word,
) -> CapabilityResult {
    let mut a0 = target;
    let mut a1 = io_port::OperationType::Write as Word;
    let mut a2 = address;
    let mut a3 = byte_width;
    let mut a4 = data;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        in("rdx")  a2,          // address
        in("r8")  a3,        // byte_width
        in("r9")  a4,        // data
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
