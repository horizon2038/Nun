use core::arch::asm;

use crate::ipc_buffer::{IpcBuffer, IPC_BUFFER_SIZE};
use crate::{CapabilityDescriptor, CapabilityResult, BYTE_BITS, PAGE_SIZE};

use crate::arch;
use crate::capability_call::process_control_block;

#[inline(always)]
pub unsafe fn configure_to_tls(
    _pcb_descriptor: CapabilityDescriptor,
    ipc_buffer_ptr: *mut IpcBuffer,
) -> CapabilityResult {
    let configuration_info = process_control_block::ConfigurationInfo::new(
        false, false, false, false, false, false, false, true, false, false,
    );

    // user can access IPC buffer via gs:[0x00]
    const TLS_BASE_OFFSET: usize = IPC_BUFFER_SIZE - 2;
    let ipc_buffer_raw = ipc_buffer_ptr as usize;
    let tls_base = ipc_buffer_raw + (TLS_BASE_OFFSET * BYTE_BITS);

    ipc_buffer_ptr
        .as_mut()
        .expect("ipc_buffer_ptr is null")
        .configure_message(TLS_BASE_OFFSET, ipc_buffer_raw);

    crate::arch::process_control_block::configure(
        _pcb_descriptor,
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
pub unsafe fn get_ipc_buffer() -> *mut IpcBuffer {
    let ipc_buffer_ptr: *mut IpcBuffer;
    asm!(
        "mov {}, gs:[0x00]",
        out(reg) ipc_buffer_ptr,
        options(nostack, nomem, preserves_flags)
    );

    ipc_buffer_ptr
}
