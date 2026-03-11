use crate::types::*;
use core::arch::asm;

pub fn write_char(c: char) {
    unsafe {
        asm!(
        "syscall",
        in("rax") KernelCallType::DebugCall as isize, // kernel call number 2 : debug::put_char
        in("rdi") c as u16, // debug_write_char (ascii)
        out("rcx") _,
        out("r11") _,
        clobber_abi("system"),
        // options(preserves_flags),
        );
    }
}
