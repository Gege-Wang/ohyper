#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

#[macro_use]
extern crate log;

mod lang_items;
mod uart;
mod uart16550;
mod interrupts;
mod gdt;
mod lapic;
mod timer;
mod memory;
mod heap;
mod hv;
#[macro_use]
mod logging;

extern crate alloc;

use core::sync::atomic::{AtomicBool, Ordering};
use bootloader::{BootInfo, entry_point};
use x86_64::VirtAddr;

static INIT_OK: AtomicBool = AtomicBool::new(false);
pub fn init_ok() -> bool {
    INIT_OK.load(Ordering::SeqCst)
}

const HELLO: &'static str = r"
    OOOOO   HH   HH YYY   YYY PPPPPP  EEEEEEE RRRRRR
   OO   OO  HH   HH  YYY YYY  PP   PP EE      RR   RR
   OO   OO  HHHHHHH   YYY     PPPPPP  EEEEE   RRRRRR
   OO   OO  HH   HH   YYY     PP      EE      RR  RR
    OOOOO   HH   HH   YYY     PP      EEEEEEE RR   RR
";

entry_point!(kernel_main);
fn kernel_main(bootloader_info: &'static BootInfo) -> ! {
// #[no_mangle]
// extern "C" fn _start() -> ! {
    println!("{}", HELLO);
    println!(
        "\
        arch = {}\n\
        build_mode = {}\n\
        log_level = {}\n\
        ",
        option_env!("ARCH").unwrap_or(""),
        option_env!("MODE").unwrap_or(""),
        option_env!("LOG").unwrap_or(""),
    );
    logging::init();
    info!("Logging initialized");
    interrupts::init();
    gdt::init();
    lapic::init();
    timer::init();
    INIT_OK.store(true, Ordering::SeqCst);
    //x86_64::instructions::interrupts::int3();
    x86_64::instructions::interrupts::enable();
    info!("Interrupts enabled");

    let physical_mem_offset = VirtAddr::new(bootloader_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(physical_mem_offset) };
    let mut frame_allocator = unsafe{
        memory::BootInfoFrameAllocator::init(
            &bootloader_info.memory_map
        )
    };
    heap::init(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
    info!("Initializd heap");
    hv::run();
    hlt_loop();
}

fn hlt_loop() -> ! {
    loop{
        x86_64::instructions::hlt();
    }
}