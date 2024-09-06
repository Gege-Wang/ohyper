#![no_std]
mod x86_64;
use x86_64::*;

pub fn has_vmx_support() -> bool {
    x86_64::has_vmx_support()
}