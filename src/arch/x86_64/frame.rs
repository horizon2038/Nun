use core::arch::asm;

use crate::capability_call::frame;
use crate::types::*;

#[inline(always)]
pub fn get_address(descriptor: CapabilityDescriptor) -> Result<PhysicalAddress, CapabilityError> {
    let mut a0 = descriptor;
    let mut a1 = frame::OperationType::GetAddress as Word;
    let mut a2 = 0; // address

    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        out("rdx") a2,
        out("r8") _,
        out("r9") _,
        out("r10") _,
        out("r12") _,
        out("r13") _,
        out("r14") _,
        out("r15") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1).map(|_| a2)
}
