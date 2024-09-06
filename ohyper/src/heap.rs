use core::alloc::GlobalAlloc;
use buddy_system_allocator::LockedHeap;
use x86_64::{
    structures::paging::{mapper::MapToError, PageTableFlags, Mapper, Page, FrameAllocator, Size4KiB},
    PhysAddr, VirtAddr,
};

const HEAP_START: usize = 0x_4444_4444_0000;
const HEAP_SIZE: usize = 100 * 1024;

#[global_allocator]
static  ALLOCATOR: LockedHeap<32> = LockedHeap::empty();

pub fn init(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE as u64 - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };

    for page in page_range {
        let frame = frame_allocator
            .allocate_frame()
            .ok_or(MapToError::FrameAllocationFailed)?;
        let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        unsafe {
            mapper.map_to(page, frame, flags, frame_allocator)?.flush()
        };
    }
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }

    Ok(())
}