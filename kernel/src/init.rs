#![no_std]

use core::fmt;

#[derive(Debug)]
pub enum InitError {
    GdtError,
    InterruptError,
    MemoryError(&'static str),
}

impl fmt::Display for InitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InitError::GdtError => write!(f, "GDT初始化失败"),
            InitError::InterruptError => write!(f, "中断描述符表初始化失败"),
            InitError::MemoryError(msg) => write!(f, "内存管理错误: {}", msg),
        }
    }
}

pub fn early_init() -> Result<(), InitError> {
    init_gdt()?;
    init_idt()?;
    Ok(())
}

fn init_gdt() -> Result<(), InitError> {
    // 使用x86_64 crate的GDT结构
    x86_64::instructions::tables::load_tss(
        x86_64::structures::gdt::SegmentSelector::new(0)
    ).map_err(|_| InitError::GdtError)
}

fn init_idt() -> Result<(), InitError> {
    // 使用x86_64 crate的IDT结构
    let mut idt = x86_64::structures::idt::InterruptDescriptorTable::new();
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    unsafe { idt.load() };
    Ok(())
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    // 基本断点处理逻辑
}