//! 内存检测模块
//! 
//! 本模块负责检测系统可用内存并建立基础的内存映射

use crate::error::BootError;
use core::mem;

/// 内存区域类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryType {
    /// 可用内存
    Available,
    /// 保留内存
    Reserved,
    /// ACPI可回收内存
    AcpiReclaimable,
    /// ACPI NVS内存
    AcpiNvs,
    /// 坏内存
    BadMemory,
    /// 内核代码段
    KernelCode,
    /// 内核数据段
    KernelData,
    /// 设备内存映射
    DeviceMemory,
}

/// 内存区域描述符
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    /// 起始物理地址
    pub start_addr: usize,
    /// 大小（字节）
    pub size: usize,
    /// 内存类型
    pub memory_type: MemoryType,
    /// 属性标志
    pub attributes: MemoryAttributes,
}

/// 内存属性标志
#[derive(Debug, Clone, Copy)]
pub struct MemoryAttributes {
    /// 可读
    pub readable: bool,
    /// 可写
    pub writable: bool,
    /// 可执行
    pub executable: bool,
    /// 可缓存
    pub cacheable: bool,
    /// 写穿透
    pub write_through: bool,
}

/// 内存映射信息
#[derive(Debug)]
pub struct MemoryMap {
    /// 内存区域列表
    pub regions: [MemoryRegion; MAX_MEMORY_REGIONS],
    /// 有效区域数量
    pub region_count: usize,
    /// 总可用内存大小
    pub total_memory: usize,
    /// 可用内存大小
    pub available_memory: usize,
}

/// 最大内存区域数量
const MAX_MEMORY_REGIONS: usize = 64;

/// 全局内存映射
static mut MEMORY_MAP: Option<MemoryMap> = None;

impl Default for MemoryAttributes {
    fn default() -> Self {
        Self {
            readable: true,
            writable: true,
            executable: false,
            cacheable: true,
            write_through: false,
        }
    }
}

impl MemoryRegion {
    /// 创建新的内存区域
    pub fn new(start_addr: usize, size: usize, memory_type: MemoryType) -> Self {
        Self {
            start_addr,
            size,
            memory_type,
            attributes: MemoryAttributes::default(),
        }
    }

    /// 获取结束地址
    pub fn end_addr(&self) -> usize {
        self.start_addr + self.size
    }

    /// 检查地址是否在此区域内
    pub fn contains(&self, addr: usize) -> bool {
        addr >= self.start_addr && addr < self.end_addr()
    }

    /// 检查是否与另一个区域重叠
    pub fn overlaps(&self, other: &MemoryRegion) -> bool {
        self.start_addr < other.end_addr() && other.start_addr < self.end_addr()
    }
}

impl MemoryMap {
    /// 创建新的内存映射
    pub fn new() -> Self {
        Self {
            regions: [MemoryRegion::new(0, 0, MemoryType::Reserved); MAX_MEMORY_REGIONS],
            region_count: 0,
            total_memory: 0,
            available_memory: 0,
        }
    }

    /// 添加内存区域
    pub fn add_region(&mut self, region: MemoryRegion) -> Result<(), BootError> {
        if self.region_count >= MAX_MEMORY_REGIONS {
            return Err(BootError::MemoryDetectionFailed);
        }

        // 检查是否与现有区域重叠
        for i in 0..self.region_count {
            if self.regions[i].overlaps(&region) {
                crate::early_println!("警告: 内存区域重叠 0x{:x}-0x{:x} 与 0x{:x}-0x{:x}",
                    region.start_addr, region.end_addr(),
                    self.regions[i].start_addr, self.regions[i].end_addr());
            }
        }

        self.regions[self.region_count] = region;
        self.region_count += 1;

        // 更新统计信息
        self.total_memory += region.size;
        if region.memory_type == MemoryType::Available {
            self.available_memory += region.size;
        }

        Ok(())
    }

    /// 查找包含指定地址的内存区域
    pub fn find_region(&self, addr: usize) -> Option<&MemoryRegion> {
        for i in 0..self.region_count {
            if self.regions[i].contains(addr) {
                return Some(&self.regions[i]);
            }
        }
        None
    }

    /// 获取可用内存区域迭代器
    pub fn available_regions(&self) -> impl Iterator<Item = &MemoryRegion> {
        self.regions[..self.region_count]
            .iter()
            .filter(|r| r.memory_type == MemoryType::Available)
    }

    /// 排序内存区域（按起始地址）
    pub fn sort_regions(&mut self) {
        // 简单的冒泡排序
        for i in 0..self.region_count {
            for j in 0..self.region_count - 1 - i {
                if self.regions[j].start_addr > self.regions[j + 1].start_addr {
                    self.regions.swap(j, j + 1);
                }
            }
        }
    }
}

/// 检测系统内存
pub fn detect_system_memory() -> Result<(), BootError> {
    crate::early_println!("开始检测系统内存...");

    let mut memory_map = MemoryMap::new();

    // 从设备树或其他源获取内存信息
    // 这里使用硬编码的示例值，实际实现需要解析设备树
    detect_memory_from_device_tree(&mut memory_map)?;

    // 添加内核占用的内存区域
    add_kernel_regions(&mut memory_map)?;

    // 排序内存区域
    memory_map.sort_regions();

    // 打印内存映射信息
    print_memory_map(&memory_map);

    // 保存到全局变量
    unsafe {
        MEMORY_MAP = Some(memory_map);
    }

    crate::early_println!("内存检测完成");
    Ok(())
}

/// 从设备树检测内存
fn detect_memory_from_device_tree(memory_map: &mut MemoryMap) -> Result<(), BootError> {
    // 这里应该解析设备树中的memory节点
    // 暂时使用硬编码的值作为示例
    
    // 示例：添加主内存区域（128MB）
    let main_memory = MemoryRegion::new(
        0x80000000,  // RISC-V典型的内存起始地址
        128 * 1024 * 1024,  // 128MB
        MemoryType::Available,
    );
    memory_map.add_region(main_memory)?;

    // 示例：添加设备内存映射区域
    let device_memory = MemoryRegion {
        start_addr: 0x10000000,
        size: 0x10000000,  // 256MB设备空间
        memory_type: MemoryType::DeviceMemory,
        attributes: MemoryAttributes {
            readable: true,
            writable: true,
            executable: false,
            cacheable: false,  // 设备内存通常不可缓存
            write_through: true,
        },
    };
    memory_map.add_region(device_memory)?;

    Ok(())
}

/// 添加内核占用的内存区域
fn add_kernel_regions(memory_map: &mut MemoryMap) -> Result<(), BootError> {
    // 获取内核代码段和数据段的地址
    extern "C" {
        static __kernel_start: u8;
        static __kernel_end: u8;
        static __text_start: u8;
        static __text_end: u8;
        static __data_start: u8;
        static __data_end: u8;
        static __bss_start: u8;
        static __bss_end: u8;
    }

    unsafe {
        let kernel_start = &__kernel_start as *const u8 as usize;
        let kernel_end = &__kernel_end as *const u8 as usize;
        let text_start = &__text_start as *const u8 as usize;
        let text_end = &__text_end as *const u8 as usize;
        let data_start = &__data_start as *const u8 as usize;
        let data_end = &__data_end as *const u8 as usize;
        let bss_start = &__bss_start as *const u8 as usize;
        let bss_end = &__bss_end as *const u8 as usize;

        // 添加内核代码段
        if text_end > text_start {
            let code_region = MemoryRegion {
                start_addr: text_start,
                size: text_end - text_start,
                memory_type: MemoryType::KernelCode,
                attributes: MemoryAttributes {
                    readable: true,
                    writable: false,
                    executable: true,
                    cacheable: true,
                    write_through: false,
                },
            };
            memory_map.add_region(code_region)?;
        }

        // 添加内核数据段
        if data_end > data_start {
            let data_region = MemoryRegion {
                start_addr: data_start,
                size: data_end - data_start,
                memory_type: MemoryType::KernelData,
                attributes: MemoryAttributes {
                    readable: true,
                    writable: true,
                    executable: false,
                    cacheable: true,
                    write_through: false,
                },
            };
            memory_map.add_region(data_region)?;
        }

        // 添加BSS段
        if bss_end > bss_start {
            let bss_region = MemoryRegion {
                start_addr: bss_start,
                size: bss_end - bss_start,
                memory_type: MemoryType::KernelData,
                attributes: MemoryAttributes {
                    readable: true,
                    writable: true,
                    executable: false,
                    cacheable: true,
                    write_through: false,
                },
            };
            memory_map.add_region(bss_region)?;
        }
    }

    Ok(())
}

/// 打印内存映射信息
fn print_memory_map(memory_map: &MemoryMap) {
    crate::early_println!("内存映射信息:");
    crate::early_println!("总内存: {} MB", memory_map.total_memory / (1024 * 1024));
    crate::early_println!("可用内存: {} MB", memory_map.available_memory / (1024 * 1024));
    crate::early_println!("内存区域数量: {}", memory_map.region_count);
    crate::early_println!("");

    for i in 0..memory_map.region_count {
        let region = &memory_map.regions[i];
        let type_str = match region.memory_type {
            MemoryType::Available => "可用",
            MemoryType::Reserved => "保留",
            MemoryType::AcpiReclaimable => "ACPI可回收",
            MemoryType::AcpiNvs => "ACPI NVS",
            MemoryType::BadMemory => "坏内存",
            MemoryType::KernelCode => "内核代码",
            MemoryType::KernelData => "内核数据",
            MemoryType::DeviceMemory => "设备内存",
        };

        crate::early_println!(
            "  区域{}: 0x{:08x}-0x{:08x} ({} KB) - {}",
            i,
            region.start_addr,
            region.end_addr(),
            region.size / 1024,
            type_str
        );
    }
    crate::early_println!("");
}

/// 获取内存映射
pub fn get_memory_map() -> Option<&'static MemoryMap> {
    unsafe { MEMORY_MAP.as_ref() }
}

/// 获取可用内存的起始地址和大小
pub fn get_available_memory() -> Option<(usize, usize)> {
    unsafe {
        if let Some(memory_map) = &MEMORY_MAP {
            // 找到第一个可用内存区域
            for region in memory_map.available_regions() {
                return Some((region.start_addr, region.size));
            }
        }
    }
    None
}

/// 检查地址是否在可用内存范围内
pub fn is_address_available(addr: usize) -> bool {
    unsafe {
        if let Some(memory_map) = &MEMORY_MAP {
            if let Some(region) = memory_map.find_region(addr) {
                return region.memory_type == MemoryType::Available;
            }
        }
    }
    false
}