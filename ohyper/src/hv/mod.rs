use crate::println;
use ovm;

pub fn run() {
    println!("Starting run VMXON....");
    println!("Hardware VMX support: {:?}", ovm::has_vmx_support());
}
