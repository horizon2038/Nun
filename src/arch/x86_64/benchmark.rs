use core::arch::asm;
use core::arch::x86_64::__rdtscp;

use crate::types::*;

#[inline(always)]
pub fn cycle_counter() -> Word {
    let mut _aux = 0u32;
    unsafe { __rdtscp(&mut _aux) as Word }
}
