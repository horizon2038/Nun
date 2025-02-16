use crate::debug_log;
use core::panic::PanicInfo;

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    println!("[ \x1b[31mPANIC\x1b[0m ] {}", _info);
    loop {}
}
