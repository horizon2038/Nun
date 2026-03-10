use crate::capability_call::virtual_cpu;
use crate::types::*;
use core::arch::asm;

#[inline(always)]
pub fn configure(
    descriptor: CapabilityDescriptor,
    address_space_descriptor: CapabilityDescriptor,
    vcpu_configuration: virtual_cpu::VcpuConfiguration,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = virtual_cpu::OperationType::Configure as Word;

    let mut a2 = address_space_descriptor as Word;
    let mut a3 = vcpu_configuration.data;

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // oepration  -> capablity_error
        in("r8")     a2,       // address_space_descriptor
        in("r9")     a3,       // vcpu_configuration
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
