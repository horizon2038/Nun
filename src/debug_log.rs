use crate::arch::debug_call;
use crate::types::*;
use core::fmt;

#[macro_export]
macro_rules! print {
    ($($args:tt)*) => ({
        use core::fmt::Write;
        let _ = write!($crate::DebugWriter::new(), $($args)*);
    });
}

#[macro_export]
macro_rules! println {
    () => {
        print!("\r\n")
    };

    ($fmt:expr) => {
        $crate::print!(concat!($fmt, "\r\n"))
    };

    ($fmt:expr, $($args:tt)*) => {
        $crate::print!(concat!($fmt, "\r\n"), $($args)*)
    };
}

pub struct DebugWriter;

impl DebugWriter {
    pub fn new() -> Self {
        DebugWriter {}
    }
}

impl fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.bytes() {
            if !c.is_ascii() {
                debug_call::write_char(b'?' as char);
                break;
            }

            debug_call::write_char(c as char);
        }

        Ok(())
    }
}
