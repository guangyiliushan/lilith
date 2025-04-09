#![no_std]
use core::arch::asm;
use core::fmt::Write;
use multiboot2::BootInformation;

struct SerialPort {
    port: u16,
}

impl SerialPort {
    const fn new(port: u16) -> Self {
        SerialPort { port }
    }

    unsafe fn write_byte(&self, byte: u8) {
        unsafe {
            asm!(
                "outb %al, %dx",
                in("al") byte,
                in("dx") self.port,
                options(nostack, nomem, preserves_flags)
            );
        }
    }
}

impl Write for SerialPort {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            unsafe { self.write_byte(byte); }
        }
        Ok(())
    }
}

static mut SERIAL1: SerialPort = SerialPort::new(0x3F8);

pub extern "C" fn start(_boot_info: &'static BootInformation) {
    unsafe {
        let serial = &raw mut SERIAL1;
        serial.as_mut().unwrap().write_str("Hello, World! This is my kernel with multiboot2.\n").unwrap();
    }
}