#![no_std]
#![no_main]
use core::fmt::Write;

#[macro_use]
extern crate log;

mod lang_items;
mod uart;
mod uart16550;
#[macro_use]
mod logging;

const HELLO: &'static str = r"
    OOOOO   HH   HH YYY   YYY PPPPPP  EEEEEEE RRRRRR
   OO   OO  HH   HH  YYY YYY  PP   PP EE      RR   RR
   OO   OO  HHHHHHH   YYY     PPPPPP  EEEEE   RRRRRR
   OO   OO  HH   HH   YYY     PP      EE      RR  RR
    OOOOO   HH   HH   YYY     PP      EEEEEEE RR   RR
";
#[no_mangle]
pub extern "C" fn _start() -> ! {
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
    loop{}
}