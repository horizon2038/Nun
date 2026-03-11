use core::arch::asm;

use crate::capability_call::generic;
use crate::types::*;

#[inline(always)]
pub fn convert(
    target: CapabilityDescriptor,
    capability_type: CapabilityType,
    specific_bits: Word,
    count: Word,
    node: CapabilityDescriptor,
    node_index: Word,
) -> CapabilityResult {
    let mut a0 = target;
    let mut a1 = generic::OperationType::Convert as Word;

    let a2: Word = capability_type as Word;
    let a3: Word = specific_bits;
    let a4: Word = count;
    let a5: Word = node;
    let a6: Word = node_index;

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // operation  -> capability_error
        in("rdx") a2, // capability type
        in("r8")  a3, // specific bits
        in("r9")  a4, // count
        in("r10") a5, // node descriptor
        in("r12") a6, // node index
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
