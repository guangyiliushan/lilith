//! RISC-V架构特定实现
//! 
//! 本模块实现了RISC-V RV23架构的特定功能

pub mod registers;
pub mod interrupt;
pub mod memory;
pub mod smp;

use crate::error::KernelError;

// 重新导出核心功能
pub use registers::*;
pub use interrupt::*;
pub use memory::*;
pub use smp::*;

/// 等待中断
pub fn wait_for_interrupt() {
    unsafe {
        core::arch::asm!("wfi");
    }
}

/// 停止所有CPU核心
pub fn halt_all_cores() -> ! {
    // 发送停止信号给其他核心
    smp::halt_other_cores();
    
    // 停止当前核心
    loop {
        unsafe {
            core::arch::asm!("wfi");
        }
    }
}

/// 初始化中断系统
pub fn interrupt_init() -> Result<(), KernelError> {
    interrupt::init_interrupt_system()
}