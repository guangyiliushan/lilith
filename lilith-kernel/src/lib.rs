//! Lilith OS - 基于Rust的RISC-V RV23微内核操作系统
//! 
//! 本模块实现了内核的核心功能，包括：
//! - M-mode初始化和配置
//! - 硬件抽象层
//! - 内存管理
//! - 进程调度
//! - 设备驱动框架

#![no_std]
#![no_main]
#![feature(asm_const)]
#![feature(naked_functions)]
#![feature(panic_info_message)]

extern crate alloc;

use core::panic::PanicInfo;

// 核心模块导入
pub mod arch;
pub mod boot;
pub mod mm;
pub mod sched;
pub mod fs;
pub mod net;
pub mod drivers;
pub mod sync;
pub mod error;

// 重新导出核心类型
pub use arch::riscv::*;
pub use boot::*;
pub use error::*;

/// 内核版本信息
pub const KERNEL_VERSION: &str = "0.1.0";
pub const KERNEL_NAME: &str = "Lilith OS";
pub const KERNEL_ARCH: &str = "RISC-V RV23";

/// 内核初始化结果
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KernelInitResult {
    /// 初始化成功
    Success,
    /// 硬件不兼容
    HardwareIncompatible,
    /// 内存不足
    InsufficientMemory,
    /// 设备初始化失败
    DeviceInitFailed,
    /// 配置错误
    ConfigurationError,
}

/// 内核主初始化函数
/// 
/// 这是内核的主要入口点，负责完成所有必要的初始化工作
pub fn kernel_init() -> KernelInitResult {
    // 1. M-mode初始化（机器模式寄存器配置）
    match boot::machine_mode_init() {
        Ok(_) => {},
        Err(e) => {
            // 早期错误处理，此时可能无法使用正常的日志系统
            return match e {
                BootError::HardwareIncompatible => KernelInitResult::HardwareIncompatible,
                BootError::ConfigurationError => KernelInitResult::ConfigurationError,
                _ => KernelInitResult::DeviceInitFailed,
            };
        }
    }

    // 2. 早期串口初始化（用于调试输出）
    if let Err(_) = boot::early_uart_init() {
        return KernelInitResult::DeviceInitFailed;
    }

    // 3. 内存子系统初始化
    if let Err(_) = mm::memory_init() {
        return KernelInitResult::InsufficientMemory;
    }

    // 4. 中断系统初始化
    if let Err(_) = arch::interrupt_init() {
        return KernelInitResult::DeviceInitFailed;
    }

    // 5. 调度器初始化
    if let Err(_) = sched::scheduler_init() {
        return KernelInitResult::ConfigurationError;
    }

    KernelInitResult::Success
}

/// 内核主循环
/// 
/// 在完成初始化后，内核进入主循环等待事件处理
pub fn kernel_main() -> ! {
    match kernel_init() {
        KernelInitResult::Success => {
            // 初始化成功，进入正常运行模式
            loop {
                // 等待中断或调度事件
                arch::wait_for_interrupt();
            }
        },
        error => {
            // 初始化失败，进入错误处理模式
            panic!("内核初始化失败: {:?}", error);
        }
    }
}

/// 内核恐慌处理函数
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 尝试输出恐慌信息
    if let Some(message) = info.message() {
        // 如果串口可用，输出详细信息
        boot::emergency_print(format_args!("内核恐慌: {}\n", message));
    }
    
    if let Some(location) = info.location() {
        boot::emergency_print(format_args!(
            "位置: {}:{}:{}\n", 
            location.file(), 
            location.line(), 
            location.column()
        ));
    }

    // 停止所有CPU核心
    arch::halt_all_cores();
}

/// 全局内存分配器错误处理
#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("内存分配失败: {:?}", layout);
}