#![allow(dead_code)]

use crate::vfs;
use x86_64::registers::control::Cr2;

#[repr(u32)]
#[derive(Debug)]
pub enum SyscallNumber {
    Read = 0,
    Write = 1,
    Open = 2,
    Close = 3,
}

#[derive(Debug, Copy, Clone)]
pub struct SyscallContext {
    pub rax: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub r10: u64,
    pub r8: u64,
    pub r9: u64,
}

#[no_mangle]
extern "C" fn syscall_handler(ctx: &mut SyscallContext) -> u64 {
    let syscall_num = ctx.rax as u32;
    
    match syscall_num {
        num if num == SyscallNumber::Read as u32 => {
            let fd = ctx.rdi as usize;
            let buf_ptr = ctx.rsi as *mut u8;
            let count = ctx.rdx as usize;
            vfs::read(fd, buf_ptr, count)
        }
        num if num == SyscallNumber::Write as u32 => {
            let fd = ctx.rdi as usize;
            let buf_ptr = ctx.rsi as *const u8;
            let count = ctx.rdx as usize;
            vfs::write(fd, buf_ptr, count)
        }
        num if num == SyscallNumber::Open as u32 => {
            let path_ptr = ctx.rdi as *const u8;
            let flags = ctx.rsi as u32;
            vfs::open(path_ptr, flags)
        }
        num if num == SyscallNumber::Close as u32 => {
            let fd = ctx.rdi as usize;
            vfs::close(fd)
        }
        _ => {
            println!("未知系统调用: {}", syscall_num);
            0
        }
    }
}

pub fn init() {
    unsafe {
        x86_64::instructions::interrupts::disable();
        // 设置系统调用门（INT 0x80）
        x86_64::instructions::interrupts::set_system_handler(0x80, syscall_entry);
        x86_64::instructions::interrupts::enable();
    }
}

extern "x86-interrupt" fn syscall_entry(stack_frame: x86_64::structures::idt::InterruptStackFrame) {
    let mut ctx = SyscallContext {
        rax: x86_64::registers::model_specific::Rax::read(),
        rdi: x86_64::registers::model_specific::Rdi::read(),
        rsi: x86_64::registers::model_specific::Rsi::read(),
        rdx: x86_64::registers::model_specific::Rdx::read(),
        r10: x86_64::registers::model_specific::R10::read(),
        r8: x86_64::registers::model_specific::R8::read(),
        r9: x86_64::registers::model_specific::R9::read(),
    };
    
    let result = syscall_handler(&mut ctx);
    
    x86_64::registers::model_specific::Rax::write(result);
}