#![no_std]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

/// 引导程序入口点
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // 初始化基础硬件环境
    init_serial();
    init_interrupts();
    
    // 调用内核入口
    unsafe { 
        let kernel_main: extern "C" fn() -> ! = core::mem::transmute(0x100000);
        kernel_main();
    }
}

/// 初始化串口输出
fn init_serial() {
    let mut port = Port::new(0x3F8);
    unsafe {
        port.write(0x03u8);  // 设置波特率
        port.write(0x80u8); // 启用FIFO
    }
}

/// 初始化中断控制器
fn init_interrupts() {
    unsafe {
        // 配置PIC主从片
        Port::new(0x20).write(0x11u8);
        Port::new(0xA0).write(0x11u8);
        // 设置中断向量偏移量
        Port::new(0x21).write(0x20u8);
        Port::new(0xA1).write(0x28u8);
        // 建立级联关系
        Port::new(0x21).write(0x04u8);
        Port::new(0xA1).write(0x02u8);
        // 设置工作模式
        Port::new(0x21).write(0x01u8);
        Port::new(0xA1).write(0x01u8);
    }
}

/// panic处理
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        let mut port = Port::new(0x3F8);
        let s = format_args!("Bootloader panic: {}\n", info);
        for b in s.as_str().unwrap().bytes() {
            port.write(b);
        }
    }
    loop {
        x86_64::instructions::hlt();
    }
}