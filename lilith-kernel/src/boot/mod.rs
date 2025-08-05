//! 引导加载和初始化模块
//! 
//! 本模块负责系统的早期初始化，包括：
//! - M-mode机器模式寄存器配置
//! - 硬件发现与初始化
//! - S-mode准备工作
//! - 早期调试支持

pub mod machine_mode;
pub mod uart;
pub mod memory_detect;

use crate::error::{BootError, KernelError};
use core::fmt::Arguments;

// 重新导出核心功能
pub use machine_mode::*;
pub use uart::*;
pub use memory_detect::*;

/// M-mode初始化主函数
/// 
/// 这是系统启动后的第一个初始化步骤，负责配置机器模式寄存器
/// 并为后续的S-mode切换做准备
pub fn machine_mode_init() -> Result<(), BootError> {
    // 1. 验证硬件兼容性
    machine_mode::verify_hardware_compatibility()?;
    
    // 2. 配置机器模式寄存器
    machine_mode::configure_machine_registers()?;
    
    // 3. 设置物理内存保护
    machine_mode::setup_physical_memory_protection()?;
    
    // 4. 初始化机器模式异常向量
    machine_mode::setup_machine_trap_vector()?;
    
    // 5. 准备S-mode环境
    machine_mode::prepare_supervisor_mode()?;
    
    Ok(())
}

/// 早期串口初始化
/// 
/// 在内存管理系统初始化之前提供基础的调试输出能力
pub fn early_uart_init() -> Result<(), BootError> {
    uart::init_early_uart()
}

/// 紧急打印函数
/// 
/// 用于在系统恐慌或严重错误时输出信息
pub fn emergency_print(args: Arguments) {
    uart::emergency_write_fmt(args);
}

/// 内存检测和初始化
/// 
/// 检测系统可用内存并建立基础的内存映射
pub fn detect_memory() -> Result<(), BootError> {
    memory_detect::detect_system_memory()
}