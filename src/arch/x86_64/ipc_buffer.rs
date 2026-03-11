use core::arch::asm;

use crate::ipc_buffer::IpcBuffer;
use crate::{CapabilityDescriptor, CapabilityResult};

use crate::arch;
use crate::capability_call::process_control_block;

#[inline(always)]
pub unsafe fn configure_to_tls(
    _pcb_descriptor: CapabilityDescriptor,
    ipc_buffer_ptr: *mut IpcBuffer,
) -> CapabilityResult {
    let ipc_buffer_raw = ipc_buffer_ptr as usize;
    ipc_buffer_ptr
        .as_mut()
        .expect("ipc_buffer_ptr is null")
        .configure_message(10, ipc_buffer_raw);

    let configuration_info = process_control_block::ConfigurationInfo::new(
        false, false, false, false, false, false, false, true, false, false,
    );

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
        ipc_buffer_raw,
        0,
        0,
    )
}

#[inline(always)]
pub unsafe fn get_ipc_buffer() -> *mut IpcBuffer {
    let mut gs_base: usize;
    asm!("rdgsbase {}", out(reg) gs_base, options(nostack, nomem, preserves_flags));

    gs_base as *mut IpcBuffer
}
