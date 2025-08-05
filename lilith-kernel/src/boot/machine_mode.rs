//! M-mode机器模式初始化实现
//! 
//! 本模块实现了RISC-V机器模式的寄存器配置和初始化流程
//! 严格遵循RISC-V特权架构规范

use crate::error::BootError;
use riscv::register::*;
use bitflags::bitflags;

/// 支持的RISC-V扩展集合
bitflags! {
    pub struct RiscvExtensions: u64 {
        const INTEGER       = 1 << 8;   // I - 基础整数指令集
        const MULTIPLY      = 1 << 12;  // M - 乘法除法扩展
        const ATOMIC        = 1 << 0;   // A - 原子指令扩展
        const FLOAT_SINGLE  = 1 << 5;   // F - 单精度浮点扩展
        const FLOAT_DOUBLE  = 1 << 3;   // D - 双精度浮点扩展
        const COMPRESSED    = 1 << 2;   // C - 压缩指令扩展
        const VECTOR        = 1 << 21;  // V - 向量扩展
        const SUPERVISOR    = 1 << 18;  // S - 监管者模式
        const USER          = 1 << 20;  // U - 用户模式
    }
}

/// 机器模式配置结构
#[derive(Debug, Clone)]
pub struct MachineConfig {
    /// CPU核心数量
    pub core_count: usize,
    /// 支持的扩展
    pub extensions: RiscvExtensions,
    /// 物理内存大小
    pub memory_size: usize,
    /// 时钟频率
    pub clock_frequency: u64,
    /// 是否支持向量扩展
    pub vector_support: bool,
}

/// 全局机器配置
static mut MACHINE_CONFIG: Option<MachineConfig> = None;

/// 验证硬件兼容性
/// 
/// 检查当前硬件是否支持运行Lilith OS所需的最小特性集
pub fn verify_hardware_compatibility() -> Result<(), BootError> {
    // 读取机器ISA寄存器
    let misa = misa::read();
    
    // 检查必需的扩展
    let required_extensions = RiscvExtensions::INTEGER 
        | RiscvExtensions::MULTIPLY 
        | RiscvExtensions::ATOMIC
        | RiscvExtensions::SUPERVISOR;
    
    let mut supported_extensions = RiscvExtensions::empty();
    
    // 解析MISA寄存器中的扩展信息
    if misa & (1 << 8) != 0 { supported_extensions |= RiscvExtensions::INTEGER; }
    if misa & (1 << 12) != 0 { supported_extensions |= RiscvExtensions::MULTIPLY; }
    if misa & (1 << 0) != 0 { supported_extensions |= RiscvExtensions::ATOMIC; }
    if misa & (1 << 5) != 0 { supported_extensions |= RiscvExtensions::FLOAT_SINGLE; }
    if misa & (1 << 3) != 0 { supported_extensions |= RiscvExtensions::FLOAT_DOUBLE; }
    if misa & (1 << 2) != 0 { supported_extensions |= RiscvExtensions::COMPRESSED; }
    if misa & (1 << 21) != 0 { supported_extensions |= RiscvExtensions::VECTOR; }
    if misa & (1 << 18) != 0 { supported_extensions |= RiscvExtensions::SUPERVISOR; }
    if misa & (1 << 20) != 0 { supported_extensions |= RiscvExtensions::USER; }
    
    // 检查是否支持所有必需的扩展
    if !supported_extensions.contains(required_extensions) {
        return Err(BootError::HardwareIncompatible);
    }
    
    // 检查RISC-V架构位数（必须是64位）
    let xlen = (misa >> 62) & 0x3;
    if xlen != 2 { // 2表示64位
        return Err(BootError::HardwareIncompatible);
    }
    
    // 创建机器配置
    let config = MachineConfig {
        core_count: 1, // 暂时假设单核，后续会通过设备树检测
        extensions: supported_extensions,
        memory_size: 0, // 后续通过内存检测获取
        clock_frequency: 0, // 后续通过设备树获取
        vector_support: supported_extensions.contains(RiscvExtensions::VECTOR),
    };
    
    unsafe {
        MACHINE_CONFIG = Some(config);
    }
    
    Ok(())
}

/// 配置机器模式寄存器
/// 
/// 设置机器模式下的各种控制寄存器，确保系统安全和稳定运行
pub fn configure_machine_registers() -> Result<(), BootError> {
    unsafe {
        // 1. 配置机器状态寄存器 (mstatus)
        configure_mstatus()?;
        
        // 2. 配置机器中断使能寄存器 (mie)
        configure_mie()?;
        
        // 3. 配置机器计数器使能寄存器 (mcounteren)
        configure_mcounteren()?;
        
        // 4. 配置机器环境配置寄存器 (menvcfg) - 如果支持
        if let Some(config) = &MACHINE_CONFIG {
            if config.extensions.contains(RiscvExtensions::USER) {
                configure_menvcfg()?;
            }
        }
        
        // 5. 配置向量扩展寄存器 - 如果支持
        if let Some(config) = &MACHINE_CONFIG {
            if config.vector_support {
                configure_vector_registers()?;
            }
        }
    }
    
    Ok(())
}

/// 配置机器状态寄存器
unsafe fn configure_mstatus() -> Result<(), BootError> {
    let mut mstatus_val = mstatus::read();
    
    // 设置内存特权模式为Sv48（如果支持）
    mstatus_val.set_mpp(mstatus::MPP::Supervisor);
    
    // 启用浮点单元（如果支持）
    if let Some(config) = &MACHINE_CONFIG {
        if config.extensions.contains(RiscvExtensions::FLOAT_SINGLE) 
            || config.extensions.contains(RiscvExtensions::FLOAT_DOUBLE) {
            mstatus_val.set_fs(mstatus::FS::Initial);
        }
        
        // 启用向量扩展（如果支持）
        if config.vector_support {
            mstatus_val.set_vs(mstatus::VS::Initial);
        }
    }
    
    // 设置全局中断使能（在S-mode下）
    mstatus_val.set_sie();
    
    mstatus::write(mstatus_val);
    
    Ok(())
}

/// 配置机器中断使能寄存器
unsafe fn configure_mie() -> Result<(), BootError> {
    // 暂时禁用所有机器模式中断
    // 后续会在适当的时候启用特定中断
    mie::clear_mext(); // 外部中断
    mie::clear_mtimer(); // 定时器中断
    mie::clear_msoft(); // 软件中断
    
    Ok(())
}

/// 配置机器计数器使能寄存器
unsafe fn configure_mcounteren() -> Result<(), BootError> {
    // 允许S-mode和U-mode访问基础性能计数器
    mcounteren::set_cy(); // 周期计数器
    mcounteren::set_tm(); // 时间计数器
    mcounteren::set_ir(); // 指令退休计数器
    
    Ok(())
}

/// 配置机器环境配置寄存器
unsafe fn configure_menvcfg() -> Result<(), BootError> {
    // 这里可以配置各种环境特性
    // 例如：缓存管理、内存一致性等
    // 具体配置取决于硬件实现
    
    Ok(())
}

/// 配置向量扩展寄存器
unsafe fn configure_vector_registers() -> Result<(), BootError> {
    // 设置向量长度和元素宽度
    // 这需要根据具体的向量扩展实现来配置
    
    Ok(())
}

/// 设置物理内存保护
/// 
/// 配置PMP寄存器以保护关键内存区域
pub fn setup_physical_memory_protection() -> Result<(), BootError> {
    unsafe {
        // PMP配置将在后续实现
        // 这里需要根据内存布局来设置保护区域
        
        // 示例：保护内核代码段
        // pmpaddr0::write(kernel_start_addr >> 2);
        // pmpcfg0::write(pmpcfg0::read() | (0x1F << 0)); // RWX权限
    }
    
    Ok(())
}

/// 设置机器模式异常向量
/// 
/// 配置机器模式下的异常和中断处理入口
pub fn setup_machine_trap_vector() -> Result<(), BootError> {
    extern "C" {
        fn machine_trap_handler();
    }
    
    unsafe {
        // 设置机器模式异常向量基址
        mtvec::write(
            machine_trap_handler as usize,
            mtvec::TrapMode::Direct
        );
    }
    
    Ok(())
}

/// 准备监管者模式环境
/// 
/// 为切换到S-mode做准备工作
pub fn prepare_supervisor_mode() -> Result<(), BootError> {
    unsafe {
        // 设置监管者模式异常向量（暂时指向同一个处理程序）
        extern "C" {
            fn supervisor_trap_handler();
        }
        
        // 这里需要设置stvec寄存器，但在M-mode下可能无法直接访问
        // 需要在切换到S-mode后再设置
        
        // 设置监管者模式的返回地址
        // mepc::write(supervisor_main as usize);
    }
    
    Ok(())
}

/// 获取机器配置
pub fn get_machine_config() -> Option<&'static MachineConfig> {
    unsafe { MACHINE_CONFIG.as_ref() }
}

/// 机器模式异常处理程序（汇编实现）
#[naked]
#[no_mangle]
extern "C" fn machine_trap_handler() {
    unsafe {
        core::arch::asm!(
            // 保存寄存器上下文
            "addi sp, sp, -256",
            "sd x1, 0(sp)",
            "sd x2, 8(sp)",
            "sd x3, 16(sp)",
            "sd x4, 24(sp)",
            // ... 保存所有寄存器
            
            // 调用Rust异常处理函数
            "call machine_trap_handler_rust",
            
            // 恢复寄存器上下文
            "ld x1, 0(sp)",
            "ld x2, 8(sp)",
            "ld x3, 16(sp)",
            "ld x4, 24(sp)",
            // ... 恢复所有寄存器
            "addi sp, sp, 256",
            
            // 返回
            "mret",
            options(noreturn)
        );
    }
}

/// Rust实现的机器模式异常处理函数
#[no_mangle]
extern "C" fn machine_trap_handler_rust() {
    // 读取异常原因
    let mcause = mcause::read();
    let mtval = mtval::read();
    let mepc = mepc::read();
    
    // 根据异常类型进行处理
    match mcause.cause() {
        mcause::Trap::Exception(exception) => {
            handle_machine_exception(exception, mtval, mepc);
        },
        mcause::Trap::Interrupt(interrupt) => {
            handle_machine_interrupt(interrupt);
        }
    }
}

/// 处理机器模式异常
fn handle_machine_exception(exception: mcause::Exception, mtval: usize, mepc: usize) {
    match exception {
        mcause::Exception::InstructionMisaligned => {
            panic!("指令地址不对齐异常: mepc=0x{:x}, mtval=0x{:x}", mepc, mtval);
        },
        mcause::Exception::InstructionFault => {
            panic!("指令访问错误: mepc=0x{:x}, mtval=0x{:x}", mepc, mtval);
        },
        mcause::Exception::IllegalInstruction => {
            panic!("非法指令异常: mepc=0x{:x}, mtval=0x{:x}", mepc, mtval);
        },
        mcause::Exception::LoadMisaligned => {
            panic!("加载地址不对齐异常: mepc=0x{:x}, mtval=0x{:x}", mepc, mtval);
        },
        mcause::Exception::LoadFault => {
            panic!("加载访问错误: mepc=0x{:x}, mtval=0x{:x}", mepc, mtval);
        },
        mcause::Exception::StoreMisaligned => {
            panic!("存储地址不对齐异常: mepc=0x{:x}, mtval=0x{:x}", mepc, mtval);
        },
        mcause::Exception::StoreFault => {
            panic!("存储访问错误: mepc=0x{:x}, mtval=0x{:x}", mepc, mtval);
        },
        _ => {
            panic!("未知机器模式异常: {:?}, mepc=0x{:x}, mtval=0x{:x}", exception, mepc, mtval);
        }
    }
}

/// 处理机器模式中断
fn handle_machine_interrupt(interrupt: mcause::Interrupt) {
    match interrupt {
        mcause::Interrupt::MachineSoft => {
            // 处理机器软件中断
        },
        mcause::Interrupt::MachineTimer => {
            // 处理机器定时器中断
        },
        mcause::Interrupt::MachineExternal => {
            // 处理机器外部中断
        },
        _ => {
            // 未知中断类型
        }
    }
}