use bootloader::bootinfo::MemoryMap;
use x86_64::{
    structures::paging::{OffsetPageTable, PageTable, FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr
};
use bootloader::bootinfo::MemoryRegionType;

unsafe fn active_level_4_table(physical_memory_offset: VirtAddr) -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;
    let (active_level_4_table_frame, _) = Cr3::read();
    let active_level_4_table_ptr =
        (physical_memory_offset + active_level_4_table_frame.start_address().as_u64()).as_mut_ptr();
    &mut *active_level_4_table_ptr
}

pub unsafe fn init(physical_memory_offset: VirtAddr) -> OffsetPageTable<'static> {
    let level_4_table = active_level_4_table(physical_memory_offset);
    OffsetPageTable::new(level_4_table, physical_memory_offset)
}
pub struct BootInfoFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootInfoFrameAllocator {
    pub unsafe fn init(memory_map: &'static MemoryMap) -> Self {
        BootInfoFrameAllocator {
            memory_map,
            next: 0,
        }
    }

    fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_rangs = usable_regions.map(|r| r.range.start_addr()..r.range.end_addr());
        let frame_addresses = addr_rangs.flat_map(|r| r.step_by(4096));
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))

    }
}

unsafe impl FrameAllocator<Size4KiB> for BootInfoFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}