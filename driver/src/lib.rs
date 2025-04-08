#![no_std]

pub mod dev;

// 导出公共接口
pub use dev::vga::Writer;