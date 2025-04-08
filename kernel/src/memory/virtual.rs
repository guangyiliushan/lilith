#![no_std]

use x86_64::{
    structures::paging::{
        Mapper, Page, PageTable, PageTableFlags,
        FrameAllocator, Size4KiB
    },
    VirtAddr,
};
use crate::memory::physical::PhysicalMemoryManager;

#[derive(Debug)]
pub struct VirtualAddress(VirtAddr);

pub struct PageTableManager {
    level4_table: &'static mut PageTable,
    frame_allocator: &'static mut PhysicalMemoryManager,
}

impl PageTableManager {
    pub unsafe fn new(
        level4_table: &'static mut PageTable,
        frame_allocator: &'static mut PhysicalMemoryManager
    ) -> Self {
        Self {
            level4_table,
            frame_allocator,
        }
    }

    pub fn map_to<A: FrameAllocator<Size4KiB>>(
        &mut self,
        page: Page,
        flags: PageTableFlags,
        allocator: &mut A
    ) -> Result<(), &'static str> {
        let frame = self.frame_allocator
            .allocate_frame()
            .ok_or("无法分配物理帧")?;

        unsafe {
            self.level4_table
                .map_to(page, frame, flags, allocator)
                .flush();
        }
        Ok(())
    }
}

impl Mapper<Size4KiB> for PageTableManager {
    fn map_to(
        &mut self,
        page: Page<Size4KiB>,
        frame: PhysFrame<Size4KiB>,
        flags: PageTableFlags,
        allocator: &mut impl FrameAllocator<Size4KiB>
    ) -> Result<MapperFlush<Size4KiB>, MapToError<Size4KiB>> {
        let flush = unsafe {
            self.level4_table
                .map_to(page, frame, flags, allocator)?
        };
        Ok(flush)
    }
}

// 添加地址映射方法
impl VirtualAddressSpace {
    pub unsafe fn map_page(&mut self, virtual_addr: VirtAddr, physical_addr: PhysAddr) {
        let page = Page::containing_address(virtual_addr);
        let frame = PhysFrame::containing_address(physical_addr);
        self.level4_table[page.p4_index()].set_frame(...);
        // ... 完善四级页表设置逻辑
    }
}