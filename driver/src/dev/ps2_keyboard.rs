// PS/2 键盘驱动模块
use x86_64::instructions::port::Port;
use crate::interrupts;

const PS2_DATA_PORT: u16 = 0x60;
const PS2_STATUS_PORT: u16 = 0x64;

pub struct Ps2Keyboard {
    data_port: Port<u8>,
    status_port: Port<u8>,
}

impl Ps2Keyboard {
    pub fn new() -> Self {
        Self {
            data_port: Port::new(PS2_DATA_PORT),
            status_port: Port::new(PS2_STATUS_PORT),
        }
    }

    /// 初始化键盘控制器
    pub fn init(&mut self) {
        unsafe {
            // 启用第一个PS/2端口
            self.status_port.write(0xAE);
            interrupts::register_irq_handler(1, Self::irq_handler);
        }
    }

    /// 中断处理函数
    fn irq_handler() {
        let scancode = unsafe { Port::new(PS2_DATA_PORT).read() };
        // 将扫描码转换为ASCII字符
        let key = match scancode {
            0x02 => '1',
            0x03 => '2',
            // 其他键位映射...
            _ => '\0',
        };
        crate::tty::putc(key);
    }
}