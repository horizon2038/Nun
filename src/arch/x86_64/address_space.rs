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
