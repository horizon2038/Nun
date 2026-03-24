use core::arch::asm;

use crate::ipc_buffer::{IpcBuffer, IPC_BUFFER_SIZE, TLS_BASE_OFFSET};
use crate::{CapabilityDescriptor, CapabilityResult, BYTE_BITS, PAGE_SIZE};

use crate::arch;
use crate::capability_call::process_control_block;

#[inline(always)]
pub fn configure_to_tls(
    _pcb_descriptor: CapabilityDescriptor,
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

    // UNSAFE: use raw pointer to configure IPC buffer and thread local storage base
    unsafe {
        ipc_buffer_ptr
            .as_mut()
            .expect("ipc_buffer_ptr is null")
            .configure_message(TLS_BASE_OFFSET, ipc_buffer_raw);

        // configure thread_local_base
        ipc_buffer_ptr
            .as_mut()
            .expect("ipc_buffer_ptr is null")
            .configure_message(10, tls_base);
    }

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
