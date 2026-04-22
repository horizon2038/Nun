use core::arch::asm;

use crate::capability_call::address_space;
use crate::types::*;

#[inline(always)]
pub fn map(
    descriptor: CapabilityDescriptor,
    map_descriptor: CapabilityDescriptor,
    virtual_address: VirtualAddress,
    _attribute: Word,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = address_space::OperationType::Map as Word;
    let mut a2 = map_descriptor as Word;
    let mut a3 = virtual_address as Word;
    let mut a4 = _attribute; // currently unused, but we can use it to specify cacheability,
                             // access rights, etc.

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        in("rdx")    a2,       // map_descriptor
        in("r8")     a3,       // virtual_address
        in("r9")     a4,       // attribute
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn unmap(
    descriptor: CapabilityDescriptor,
    memory_descriptor: CapabilityDescriptor,
    virtual_address: VirtualAddress,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = address_space::OperationType::Unmap as Word;
    let mut a2 = memory_descriptor as Word;
    let mut a3 = virtual_address as Word;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        in("rdx")    a2,       // memory_descriptor
        in("r8")     a3,       // virtual_address
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

use crate::debug_log;

#[inline(always)]
pub fn get_unset_depth(
    descriptor: CapabilityDescriptor,
    address: VirtualAddress,
    size_bits: usize,
) -> Result<usize, CapabilityError> {
    let mut a0 = descriptor;
    let mut a1 = address_space::OperationType::GetUnsetDepth as Word;
    let mut a2 = address as Word; // address (r8)
    let mut a3 = size_bits as Word; // FIXME: 2^12 = 4KiB

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        inout("rdx") a2 => a2, // address -> depth
        in("r8")     a3,       // leaf size bits
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    match convert_capability_result(a0, a1) {
        Ok(()) => {
            let depth = a2;
            Ok(depth)
        }
        Err(e) => Err(e),
    }
}
