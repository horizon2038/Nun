use core::arch::asm;

use crate::types::*;

#[inline(always)]
fn cycle_counter() -> Word {
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
}
