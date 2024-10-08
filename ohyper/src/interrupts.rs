use crate::lapic::local_apic;
use crate::lapic::vectors::APIC_TIMER_VECTOR;
use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

// static mut IDT:InterruptDescriptorTable = InterruptDescriptorTable::new();
// pub fn init_idt() {
//     unsafe{
//         IDT.breakpoint.set_handler_fn(breakpoint_handler);
//         IDT.load();
//     }
// }

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler).set_stack_index(0);
            // APIC Timer interrupt handler
            idt[APIC_TIMER_VECTOR].set_handler_fn(apic_timer_handler);
        }
        idt
    };
}

pub fn init() {
    IDT.load();
}
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    crate::hlt_loop();
}

extern "x86-interrupt" fn apic_timer_handler(_stack_frame: InterruptStackFrame) {
    // Handle the APIC timer interrupt
    //print!(".");

    // End of interrupt for Local APIC
    unsafe { local_apic().end_of_interrupt() };
}
