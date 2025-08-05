//! 内存管理模块
//! 
//! 本模块实现了内核的内存管理功能，包括：
//! - 物理内存管理
//! - 虚拟内存管理
//! - 页面分配器
//! - 内存映射

pub mod physical;
pub mod virtual_mem;
pub mod allocator;

use crate::error::{KernelError, MemoryError};

// 重新导出核心功能
pub use physical::*;
pub use virtual_mem::*;
pub use allocator::*;

/// 内存管理初始化
pub fn memory_init() -> Result<(), KernelError> {
    crate::early_println!("初始化内存管理系统...");

    // 1. 初始化物理内存管理器
    physical::init_physical_memory()?;

    // 2. 初始化虚拟内存管理器
    virtual_mem::init_virtual_memory()?;

    // 3. 初始化内核堆分配器
    allocator::init_kernel_allocator()?;

    crate::early_println!("内存管理系统初始化完成");
    Ok(())
}