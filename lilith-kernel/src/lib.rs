#![no_std]
#![no_main]

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
        asm!(
            "outb %al, %dx",
            in("al") byte,
            in("dx") self.port,
            options(nostack, nomem, preserves_flags)
        );
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

#[no_mangle]
pub extern "C" fn start(boot_info: &'static BootInformation) -> ! {
    unsafe {
        let serial = &mut SERIAL1;
        serial.write_str("Kernel is starting...\n").unwrap();
        serial.write_str("Initializing kernel...\n").unwrap();
    }

    kernel_init();

    unsafe {
        let serial = &mut SERIAL1;
        serial.write_str("Kernel initialized successfully!\n").unwrap();
    }

    loop {
        unsafe {
            asm!("hlt", options(nomem, nostack, preserves_flags));
        }
    }
}

fn kernel_init() {
    unsafe {
        let serial = &mut SERIAL1;
        serial.write_str("Performing basic setup...\n").unwrap();
    }
}