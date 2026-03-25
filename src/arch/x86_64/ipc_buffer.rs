use core::arch::asm;

use crate::ipc_buffer::{IpcBuffer, IPC_BUFFER_SIZE, TLS_BASE_OFFSET};
use crate::{CapabilityDescriptor, CapabilityResult, BYTE_BITS, PAGE_SIZE};

use crate::arch;
use crate::capability_call::process_control_block;
use crate::types::*;

// In the x86_64 ABI, process_control_block::configure() requires an IPC Buffer to set the Thread Local Base.
// However, in the initial state, the IPC Buffer is not yet set as the TLS Base (which is a bit confusing).
// Therefore, it is necessary to manually set up the IPC Buffer and call the syscall directly.
pub fn early_configure_to_tls(
    pcb_descriptor: CapabilityDescriptor,
    ipc_buffer: &mut IpcBuffer,
) -> CapabilityResult {
    println!("Early configuring IPC buffer to TLS base...");

    let mut a0 = pcb_descriptor;
    let mut a1 = process_control_block::OperationType::Configure as Word;
    let mut a2 = process_control_block::ConfigurationInfo::new(
        false, false, false, false, false, false, false, true, false, false,
    )
    .data;

    // user can access IPC buffer via gs:[0x00]
    let ipc_buffer_ptr = ipc_buffer as *mut IpcBuffer;
    let ipc_buffer_raw = ipc_buffer_ptr as usize;
    let tls_base = ipc_buffer_raw + (TLS_BASE_OFFSET * BYTE_BITS);

    println!(
        "Configuring IPC buffer to TLS: ipc_buffer_ptr={:#x}, ipc_buffer_raw={:#x}, tls_base={:#x}",
        ipc_buffer_ptr as usize, ipc_buffer_raw, tls_base
    );

    ipc_buffer.configure_message(TLS_BASE_OFFSET, ipc_buffer_raw);
    ipc_buffer.configure_message(10, tls_base);

    // UNSAFE: use raw pointer to configure IPC buffer and thread local storage base
    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::CapabilityCall as Sword,
        inout("rdi") a0 => a0, // descriptor -> is_success
        inout("rsi") a1 => a1, // oepration  -> capablity_error
        in("rdx")    a2,       // info
        out("rcx") _,
        out("r11") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn configure_to_tls(
    pcb_descriptor: CapabilityDescriptor,
    ipc_buffer: &mut IpcBuffer,
) -> CapabilityResult {
    let configuration_info = process_control_block::ConfigurationInfo::new(
        false, false, false, false, false, false, false, true, false, false,
    );

    // user can access IPC buffer via gs:[0x00]
    let ipc_buffer_ptr = ipc_buffer as *mut IpcBuffer;
    let ipc_buffer_raw = ipc_buffer_ptr as usize;
    let tls_base = ipc_buffer_raw + (TLS_BASE_OFFSET * BYTE_BITS);

    println!(
        "Configuring IPC buffer to TLS: ipc_buffer_ptr={:#x}, ipc_buffer_raw={:#x}, tls_base={:#x}",
        ipc_buffer_ptr as usize, ipc_buffer_raw, tls_base
    );

    ipc_buffer.configure_message(TLS_BASE_OFFSET, ipc_buffer_raw);

    crate::arch::process_control_block::configure(
        pcb_descriptor,
        configuration_info,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        tls_base,
        0,
        0,
    )
}

#[inline(always)]
pub unsafe fn unsafe_get_ipc_buffer() -> *mut IpcBuffer {
    let ipc_buffer_ptr: *mut IpcBuffer;
    asm!(
        "mov {}, gs:[0x00]",
        lateout(reg) ipc_buffer_ptr,
        options(nostack, readonly, preserves_flags)
    );

    ipc_buffer_ptr
}

#[inline(always)]
pub fn get_ipc_buffer() -> &'static mut IpcBuffer {
    unsafe { &mut *unsafe_get_ipc_buffer() }
}
