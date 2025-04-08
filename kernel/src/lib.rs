#![no_std]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

mod gdt;
mod interrupts;
mod memory;
mod vga_buffer;

/// 内核初始化入口
pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe { interrupts::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}

/// panic处理函数
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel panic: {}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

/// 内核主函数
#[no_mangle]
pub extern "C" fn _start() -> ! {
    match init::early_init() {
        Ok(()) => {
            if let Err(e) = memory::virtual::initialize_page_tables() {
                panic_handler!("内存初始化失败: {}", e);
            }
            main_loop();
        }
        Err(e) => panic_handler!("早期初始化失败: {}", e),
    }
}

fn main_loop() -> ! {
    x86_64::instructions::interrupts::enable();
    loop {
        x86_64::instructions::hlt();
    }
}