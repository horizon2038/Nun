pub fn nun_logo() {
    println!(
        r#"
     _   ____  ___   __
    / | / / / / / | / /
   /  |/ / / / /  |/ / 
  / /|  / /_/ / /|  /  
 /_/ |_/\____/_/ |_/

 Nun - an operating system framework based on the A9N Microkernel
    "#
    );
}

#[macro_export]
macro_rules! entry {
    ($path:path) => {
        $crate::arch_entry!(_entry);

        fn _entry(init_info: *const nun::InitInfo) {
            $crate::entry_point::nun_logo();
            let user_entry: fn(&nun::InitInfo) = $path;

            // architecture-independent initialization
            $crate::entry_point::configure_init(unsafe { &*init_info });

            unsafe {
                user_entry(init_info.as_ref().unwrap());
            }
        }
    };
}

use crate::types::AsCapabilityDescriptor;

pub fn configure_init(init_info: &crate::InitInfo) {
    println!("Configuring <init> ...");
    let pcb_descriptor = crate::InitSlotOffset::ProcessControlBlock.as_descriptor();

    println!("Configuring Initial IPC buffer to thread local storage...");
    let result = configure_initial_ipc_buffer_to_tls(pcb_descriptor, init_info);
    if result.is_err() {
        panic!(
            "Nun initialization failed: failed to configure IPC buffer to TLS: {:?}",
            result.err()
        );
    }
}

fn configure_initial_ipc_buffer_to_tls(
    pcb_descriptor: crate::types::CapabilityDescriptor,
    init_info: &crate::InitInfo,
) -> crate::types::CapabilityResult {
    let ipc_buffer = unsafe {
        init_info
            .ipc_buffer
            .as_mut()
            .expect("Nun initialization failed: ipc_buffer is null")
    };

    crate::arch::ipc_buffer::configure_to_tls(pcb_descriptor, ipc_buffer)
}
