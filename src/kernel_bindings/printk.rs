
use core::fmt;
use core::cmp;
use super::bindings;

pub fn printk(str: &str) {
    unsafe {
        bindings::printk(str.as_bytes().as_ptr());
    }
}

const MAX_LINE_LEN: usize = 1024 - 32;

pub struct LogWriter {
    buf: [u8; MAX_LINE_LEN],
    pos: usize,
}

impl LogWriter {
    pub fn new() -> LogWriter {
        LogWriter {
            buf: [0; MAX_LINE_LEN],
            pos: 0,
        }
    }
    pub fn to_str(&self) -> &str {
        match core::str::from_utf8(&self.buf[..self.pos]) {
            Ok(v) => v,
            Err(e) => panic!("Invalid utf-8 sequence.")
        }
    }
}

impl fmt::Write for LogWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let copy_length = cmp::min(MAX_LINE_LEN - 1 - self.pos, s.len());
        self.buf[self.pos..self.pos + s.len()].copy_from_slice(&s.as_bytes()[..copy_length]);
        self.pos += copy_length;
        self.buf[self.pos] = 0;
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::printk::printk("\n\0");
        // $crate::printk("\n");
    };
    ($str: expr) => {
        println!("{}", $str);
    };
    ($format: expr, $($arg: expr)*) => {
        let mut writer = $crate::printk::LogWriter::new();
        let _ = core::fmt::write(&mut writer, format_args!(concat!($format, "\n"), $($arg)*)).unwrap();
        $crate::printk::printk(writer.to_str());
    }
}