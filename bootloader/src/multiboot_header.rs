#![no_std]

use x86_64::structures::MultibootHeader;

#[repr(C)]
pub struct Multiboot2Header {
    /// 内存布局描述符
    pub memory_layout: MemoryLayoutTag,
    /// 帧缓冲设置
    pub framebuffer: FramebufferTag,
    /// 入口地址标签
    pub entry_address: EntryAddressTag,
    magic: u32,
    architecture: u32,
    header_length: u32,
    checksum: u32,
    tags: u64,
}

impl Multiboot2Header {
    /// 构建符合规范的头部结构
    pub fn new() -> Self {
        Multiboot2Header {
            magic: MULTIBOOT2_HEADER_MAGIC,
            architecture: ArchitectureTag::new(),
            memory_layout: MemoryLayoutTag::new(),
            framebuffer: FramebufferTag::default(),
            entry_address: EntryAddressTag::kernel_entry(),
            end_tag: EndTag::new()
        }
    }
    pub fn new() -> Option<Self> {
        let magic = 0x1BADB002;
        let architecture = 0;
        let header_length = core::mem::size_of::<Self>() as u32;
        
        let checksum = magic
            .wrapping_add(architecture)
            .wrapping_add(header_length)
            .wrapping_neg();

        Some(Self {
            magic,
            architecture,
            header_length,
            checksum,
            tags: 0,
        })
    }
}

#[link_section = ".multiboot"]
static HEADER: BootloaderHeader = BootloaderHeader::new().unwrap();