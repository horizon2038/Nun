use core::arch::asm;

use crate::capability_call::interrupt_region;
use crate::types::*;

#[inline(always)]
pub fn make_port(
    descriptor: CapabilityDescriptor,
    irq_number: Word,
    target_node: CapabilityDescriptor,
    target_node_index: Word,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = interrupt_region::OperationType::MakePort as Word;
    let mut a2 = irq_number;
    let mut a3 = target_node as Word;
    let mut a4 = target_node_index;

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // operation  -> capability_error
        in("r8")     a2,       // irq_number
        in("r9")     a3,       // target_node
        in("r10")    a4,       // target_node_index
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
