use core::fmt::{self, Write};
use crate::sbi::console_putchar;

struct StdOut;

impl Write for StdOut {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_putchar(c as usize);
        }
        Ok(())
    }
}

pub fn _print(s: fmt::Arguments) {
    StdOut.write_fmt(s).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal$(, $($arg: tt)+)?) => {
        $crate::stdio::_print(format_args!($fmt$(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal$(, $($arg: tt)+)?) => {
        $crate::stdio::_print(format_args!(concat!($fmt, "\n")$(, $($arg)+)?));
    };
}
