use core::arch::asm;
use core::arch::x86_64::__rdtscp;

use crate::types::*;

#[inline(always)]
pub fn cycle_counter() -> Word {
    /*
    let eax: Word;
    let edx: Word;
    unsafe {
        asm!(
            "rdtscp",
            out("eax") eax,
            out("edx") edx,
            lateout("ecx") _,
            options(nomem, nostack, preserves_flags),
        );
    }
    (((edx as Word) << 32) | (eax as Word)) as Word
    */
    let mut _aux = 0u32;
    unsafe { __rdtscp(&mut _aux) as Word }
}
