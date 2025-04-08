#![no_std]

use x86_64::{
    structures::paging::{
        FrameAllocator, PhysFrame, Size4KiB
    },
    PhysAddr,
};
use core::marker::PhantomData;

#[derive(Debug)]
// 物理内存管理
pub struct PhysicalAddress(PhysAddr);

// 虚拟内存管理（virtual.rs）
pub struct VirtualAddress(VirtAddr);

#[derive(Debug)]
pub struct MemoryRegion {
    start: PhysicalAddress,
    size: u64,
    region_type: MemoryRegionType,
}

#[derive(Debug, Clone, Copy)]
pub enum MemoryRegionType {
    Usable,
    Reserved,
    ACPIReclaimable,
}

pub struct PhysicalMemoryManager {
    next_frame: u64,
    _marker: PhantomData<*mut ()>,
}

impl PhysicalMemoryManager {
    pub unsafe fn init(regions: &[MemoryRegion]) -> Result<Self, &'static str> {
        let usable = regions
            .iter()
            .find(|r| matches!(r.region_type, MemoryRegionType::Usable))
            .ok_or("No usable memory region")?;

        Ok(Self {
            next_frame: usable.start.0.as_u64(),
            _marker: PhantomData,
        })
    }

    pub fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = PhysFrame::containing_address(PhysAddr::new(self.next_frame));
        self.next_frame += Size4KiB::SIZE;
        Some(frame)
    }
}

unsafe impl Send for PhysicalMemoryManager {}
unsafe impl Sync for PhysicalMemoryManager {}