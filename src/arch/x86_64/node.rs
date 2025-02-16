use core::arch::asm;

use crate::capability_call::node;
use crate::types::*;

#[inline(always)]
fn execute_destination_and_source(
    target: CapabilityDescriptor,
    operation: node::OperationType,
    destination_index: Word,
    source: CapabilityDescriptor,
    new_rights: Word,
) -> CapabilityResult {
    let mut a0 = target;
    let mut a1 = operation as Word;

    let a2: Word = destination_index;
    let a3: Word = source as Word;
    let a4: Word = new_rights;

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // oepration  -> capablity_error
        in("r8")  a2, // destination index
        in("r9")  a3, // source node
        in("r10") a4, // rights
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
fn execute_destination(
    target: CapabilityDescriptor,
    operation: node::OperationType,
    destination_index: Word,
) -> CapabilityResult {
    let mut a0 = target;
    let mut a1 = operation as Word;

    let a2: Word = destination_index;

    unsafe {
        asm!(
        "syscall",
        in("rdi") KernelCallType::CapabilityCall as Sword,
        inout("rsi") a0 => a0, // descriptor -> is_success
        inout("rdx") a1 => a1, // oepration  -> capablity_error
        in("r8")  a2, // destination index
        out("rax") _,
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn copy(
    target: CapabilityDescriptor,
    destination_index: Word,
    source: CapabilityDescriptor,
) -> CapabilityResult {
    execute_destination_and_source(
        target,
        node::OperationType::Copy,
        destination_index,
        source,
        0, // ignored
    )
}

#[inline(always)]
pub fn movec(
    target: CapabilityDescriptor,
    destination_index: Word,
    source: CapabilityDescriptor,
) -> CapabilityResult {
    execute_destination_and_source(
        target,
        node::OperationType::Move,
        destination_index,
        source,
        0, // ignored
    )
}

#[inline(always)]
pub fn mint(
    target: CapabilityDescriptor,
    destination_index: Word,
    source: CapabilityDescriptor,
    new_rights: Word,
) -> CapabilityResult {
    execute_destination_and_source(
        target,
        node::OperationType::Mint,
        destination_index,
        source,
        new_rights,
    )
}

#[inline(always)]
pub fn demote(
    target: CapabilityDescriptor,
    destination_index: Word,
    new_rights: Word,
) -> CapabilityResult {
    execute_destination_and_source(
        target,
        node::OperationType::Demote,
        destination_index,
        0 as CapabilityDescriptor, // ignored
        new_rights,
    )
}

#[inline(always)]
pub fn remove(target: CapabilityDescriptor, destination_index: Word) -> CapabilityResult {
    execute_destination(target, node::OperationType::Remove, destination_index)
}

#[inline(always)]
pub fn revoke(target: CapabilityDescriptor, destination_index: Word) -> CapabilityResult {
    execute_destination(target, node::OperationType::Revoke, destination_index)
}
