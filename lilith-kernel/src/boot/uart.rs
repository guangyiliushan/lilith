//! 早期串口驱动实现
//! 
//! 本模块实现了用于早期调试输出的串口驱动
//! 在内存管理系统初始化之前提供基础的输出能力

use crate::error::BootError;
use core::fmt::{self, Arguments, Write};
use spin::Mutex;

/// UART寄存器基地址（需要根据具体硬件平台调整）
const UART_BASE: usize = 0x10000000;

/// UART寄存器偏移
const UART_THR: usize = 0x00;  // 发送保持寄存器
const UART_RBR: usize = 0x00;  // 接收缓冲寄存器
const UART_DLL: usize = 0x00;  // 除数锁存器低位
const UART_IER: usize = 0x01;  // 中断使能寄存器
const UART_DLH: usize = 0x01;  // 除数锁存器高位
const UART_FCR: usize = 0x02;  // FIFO控制寄存器
const UART_LCR: usize = 0x03;  // 线路控制寄存器
const UART_MCR: usize = 0x04;  // 调制解调器控制寄存器
const UART_LSR: usize = 0x05;  // 线路状态寄存器
const UART_MSR: usize = 0x06;  // 调制解调器状态寄存器

/// 线路状态寄存器位定义
const LSR_THRE: u8 = 1 << 5;  // 发送保持寄存器空
const LSR_TEMT: u8 = 1 << 6;  // 发送器空

/// 线路控制寄存器位定义
const LCR_DLAB: u8 = 1 << 7;  // 除数锁存器访问位
const LCR_8N1: u8 = 0x03;     // 8数据位，无奇偶校验，1停止位

/// UART配置结构
#[derive(Debug, Clone, Copy)]
pub struct UartConfig {
    /// 基地址
    pub base_addr: usize,
    /// 波特率
    pub baud_rate: u32,
    /// 时钟频率
    pub clock_freq: u32,
    /// 数据位数
    pub data_bits: u8,
    /// 停止位数
    pub stop_bits: u8,
    /// 奇偶校验
    pub parity: Parity,
}

/// 奇偶校验类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    None,
    Odd,
    Even,
}

/// UART驱动结构
pub struct Uart {
    base_addr: usize,
    config: UartConfig,
}

/// 全局早期UART实例
static EARLY_UART: Mutex<Option<Uart>> = Mutex::new(None);

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            base_addr: UART_BASE,
            baud_rate: 115200,
            clock_freq: 50_000_000, // 50MHz，需要根据实际硬件调整
            data_bits: 8,
            stop_bits: 1,
            parity: Parity::None,
        }
    }
}

impl Uart {
    /// 创建新的UART实例
    pub fn new(config: UartConfig) -> Self {
        Self {
            base_addr: config.base_addr,
            config,
        }
    }

    /// 初始化UART
    pub fn init(&self) -> Result<(), BootError> {
        unsafe {
            // 1. 禁用中断
            self.write_reg(UART_IER, 0x00);

            // 2. 启用DLAB以设置波特率
            self.write_reg(UART_LCR, LCR_DLAB);

            // 3. 计算并设置波特率除数
            let divisor = self.config.clock_freq / (16 * self.config.baud_rate);
            self.write_reg(UART_DLL, (divisor & 0xFF) as u8);
            self.write_reg(UART_DLH, ((divisor >> 8) & 0xFF) as u8);

            // 4. 设置数据格式（8N1）并禁用DLAB
            self.write_reg(UART_LCR, LCR_8N1);

            // 5. 启用FIFO，清空缓冲区
            self.write_reg(UART_FCR, 0xC7);

            // 6. 设置调制解调器控制
            self.write_reg(UART_MCR, 0x0B);

            // 7. 测试串口是否工作正常
            self.test_uart()?;
        }

        Ok(())
    }

    /// 测试UART是否正常工作
    fn test_uart(&self) -> Result<(), BootError> {
        // 发送测试字符
        self.write_byte(b'T');
        self.write_byte(b'e');
        self.write_byte(b's');
        self.write_byte(b't');
        self.write_byte(b'\n');

        Ok(())
    }

    /// 写入单个字节
    pub fn write_byte(&self, byte: u8) {
        unsafe {
            // 等待发送缓冲区空闲
            while (self.read_reg(UART_LSR) & LSR_THRE) == 0 {
                core::hint::spin_loop();
            }

            // 写入字节
            self.write_reg(UART_THR, byte);
        }
    }

    /// 读取单个字节
    pub fn read_byte(&self) -> Option<u8> {
        unsafe {
            // 检查是否有数据可读
            if (self.read_reg(UART_LSR) & 0x01) != 0 {
                Some(self.read_reg(UART_RBR))
            } else {
                None
            }
        }
    }

    /// 写入字符串
    pub fn write_str(&self, s: &str) {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
    }

    /// 等待发送完成
    pub fn flush(&self) {
        unsafe {
            while (self.read_reg(UART_LSR) & LSR_TEMT) == 0 {
                core::hint::spin_loop();
            }
        }
    }

    /// 读取寄存器
    unsafe fn read_reg(&self, offset: usize) -> u8 {
        core::ptr::read_volatile((self.base_addr + offset) as *const u8)
    }

    /// 写入寄存器
    unsafe fn write_reg(&self, offset: usize, value: u8) {
        core::ptr::write_volatile((self.base_addr + offset) as *mut u8, value);
    }
}

impl Write for Uart {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

/// 初始化早期UART
pub fn init_early_uart() -> Result<(), BootError> {
    let config = UartConfig::default();
    let uart = Uart::new(config);
    
    // 初始化UART硬件
    uart.init()?;
    
    // 保存到全局变量
    *EARLY_UART.lock() = Some(uart);
    
    // 输出初始化成功信息
    early_print("Lilith OS - 早期UART初始化完成\n");
    
    Ok(())
}

/// 早期打印函数
pub fn early_print(s: &str) {
    if let Some(uart) = EARLY_UART.lock().as_ref() {
        uart.write_str(s);
    }
}

/// 早期格式化打印函数
pub fn early_print_fmt(args: Arguments) {
    if let Some(uart) = EARLY_UART.lock().as_mut() {
        let _ = uart.write_fmt(args);
    }
}

/// 紧急写入函数（用于panic处理）
pub fn emergency_write_fmt(args: Arguments) {
    // 直接操作硬件，不使用锁
    let uart = Uart::new(UartConfig::default());
    
    // 尝试快速初始化（可能失败，但不影响输出）
    let _ = uart.init();
    
    // 格式化并输出
    struct EmergencyWriter<'a>(&'a Uart);
    
    impl<'a> Write for EmergencyWriter<'a> {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            self.0.write_str(s);
            Ok(())
        }
    }
    
    let mut writer = EmergencyWriter(&uart);
    let _ = writer.write_fmt(args);
    uart.flush();
}

/// 早期调试宏
#[macro_export]
macro_rules! early_println {
    () => {
        $crate::boot::uart::early_print("\n")
    };
    ($($arg:tt)*) => {
        $crate::boot::uart::early_print_fmt(format_args!($($arg)*));
        $crate::boot::uart::early_print("\n");
    };
}

#[macro_export]
macro_rules! early_print {
    ($($arg:tt)*) => {
        $crate::boot::uart::early_print_fmt(format_args!($($arg)*));
    };
}