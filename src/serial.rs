use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;

const COM1: u16 = 0x3F8;

#[inline(always)]
unsafe fn outb(port: u16, value: u8) {
    core::arch::asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nomem, nostack, preserves_flags),
    );
}

#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    core::arch::asm!(
        "in al, dx",
        in("dx") port,
        out("al") value,
        options(nomem, nostack, preserves_flags),
    );
    value
}

pub struct SerialPort {
    base: u16,
}

impl SerialPort {
    pub const fn new(base: u16) -> Self {
        SerialPort { base }
    }

    pub unsafe fn init(&self) {
        outb(self.base + 1, 0x00); // disable interrupts
        outb(self.base + 3, 0x80); // enable DLAB
        outb(self.base + 0, 0x03); // divisor low
        outb(self.base + 1, 0x00); // divisor high
        outb(self.base + 3, 0x03); // 8 bits, no parity, one stop bit
        outb(self.base + 2, 0xC7); // enable FIFO
        outb(self.base + 4, 0x0B); // IRQs enabled, RTS/DSR set
    }

    #[inline(always)]
    fn is_transmit_empty(&self) -> bool {
        unsafe { inb(self.base + 5) & 0x20 != 0 }
    }

    pub fn write_byte(&self, byte: u8) {
        while !self.is_transmit_empty() {}
        unsafe {
            outb(self.base, byte);
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = {
        let port = SerialPort::new(COM1);
        unsafe {
            port.init();
        }
        Mutex::new(port)
    };
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("serial print failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (
        $crate::serial_print!(concat!($fmt, "\n"), $($arg)*)
    );
}
