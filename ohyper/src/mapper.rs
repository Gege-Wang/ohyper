// use lazy_static::lazy_static;
// use spin::Mutex;
// use x86_64::structures::paging::Mapper;
// use x86_64::structures::paging::Size4KiB;
// use x86_64::VirtAddr;
use crate::memory;
use alloc::boxed::Box;

use spin::Mutex;
use x86_64::structures::paging::{Mapper, Size4KiB};
use x86_64::VirtAddr;
use core::sync::atomic::{AtomicBool, Ordering};
use core::cell::RefCell;
use lazy_static::lazy_static;

// pub static mut MAPPER: Mutex<Option<Box<dyn Mapper<Size4KiB>>>> = Mutex::new(None);


lazy_static! {
    static ref MAPPER: Mutex<Option<Box<dyn Mapper<Size4KiB> + Send>>> = Mutex::new(None);
}

pub fn init_mapper(mapper: Box<dyn Mapper<Size4KiB> + Send>) {
    let mut mapper_lock = MAPPER.lock();
    *mapper_lock = Some(mapper);
}

pub fn get_mapper() -> Option<Box<dyn Mapper<Size4KiB> + Send>> {
    let mapper_lock = MAPPER.lock();
    mapper_lock.clone()
}

// lazy_static! {
//     static ref MAPPER: Mutex<Option<Box<dyn Mapper<Size4KiB>>>> = Mutex::new(None);
// }

// pub fn init_mapper(phys_mem_offset: VirtAddr) {
//     let mapper = unsafe {memory::init(phys_mem_offset)};
//     let mut mapper_lock = MAPPER.lock();
//     *mapper_lock = Some(mapper);
// }

// pub fn get_mapper() -> Option<impl Mapper<Size4KiB>> {
//     let mapper_locker = MAPPER.lock();
//     mapper_locker.clone()
// }