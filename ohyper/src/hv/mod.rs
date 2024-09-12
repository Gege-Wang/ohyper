use crate::println;
use core::arch::asm;
use ovm;

use x86::bits32::paging::Page;
use x86::msr::{rdmsr, wrmsr};
use x86_64::registers::control::{Cr4, Cr0, Cr4Flags};

use x86_64::structures::paging::Translate;
use crate::MAPPER;
pub fn run() {
    println!("Starting run VMXON....");
    println!("Hardware VMX support: {:?}", ovm::has_vmx_support());
    if let Err(e) = enable_vmx_operation() {
        println!("Failed to enable VMX: {}", e);
    } else {
        println!("VMX enabled successfully!");
    }
}


// MSR 寄存器定义
const IA32_FEATURE_CONTROL: u32 = 0x3a;
const IA32_VMX_BASIC: u32 = 0x480;
const IA32_VMX_CR0_FIXED0: u32 = 0x486;
const IA32_VMX_CR0_FIXED1: u32 = 0x487;
const IA32_VMX_CR4_FIXED0: u32 = 0x488;
const IA32_VMX_CR4_FIXED1: u32 = 0x489;

// VMXON 区域的物理地址对齐要求
const VMXON_REGION_ALIGN: usize = 4096;  // 4 KB

// 检查 CPU 是否支持 VMX
fn check_vmx_support() -> bool {
    let cpuid = unsafe { core::arch::x86_64::__cpuid(1) };
    let vmx_supported = (cpuid.ecx & (1 << 5)) != 0;
    vmx_supported
}

// 启用 VMX 操作
fn enable_vmx_operation() -> Result<(), &'static str> {
    // Step 1: 检查 CPU 是否支持 VMX
    if !check_vmx_support() {
        return Err("CPU does not support VMX");
    }

    // Step 2: 检查和设置 IA32_FEATURE_CONTROL MSR
    let feature_control = unsafe { rdmsr(IA32_FEATURE_CONTROL) };
    if (feature_control & 0x1) == 0 {
        // 设置 IA32_FEATURE_CONTROL MSR 的锁位和 VMX 使能位
        unsafe {
            wrmsr(IA32_FEATURE_CONTROL, feature_control | 0x5);
        }
    }

    // // Step 3: 设置 CR4 寄存器中的 VMXE 位
    // let cr4 = unsafe { Cr4::read_raw() };
    // unsafe { Cr4::write_raw(cr4 | (1 << 13)) };  // 设置 VMXE 位

    // Step 4: 设置 CR0 和 CR4 的固定位
    let cr0_fixed0 = unsafe { rdmsr(IA32_VMX_CR0_FIXED0) };
    let cr0_fixed1 = unsafe { rdmsr(IA32_VMX_CR0_FIXED1) };
    let cr4_fixed0 = unsafe { rdmsr(IA32_VMX_CR4_FIXED0) };
    let cr4_fixed1 = unsafe { rdmsr(IA32_VMX_CR4_FIXED1) };

    // 确保 CR0 和 CR4 的固定位设置正确
    let mut cr0 = unsafe { Cr0::read_raw() };
    cr0 &= cr0_fixed1;
    cr0 |= cr0_fixed0;
    unsafe { Cr0::write_raw(cr0) };

    let mut cr4 = unsafe { Cr4::read_raw() };
    cr4 &= cr4_fixed1;
    cr4 |= cr4_fixed0;
    unsafe { Cr4::write_raw(cr4) };

    // Step 5: 分配并初始化 VMXON 区域
    let vmxon_region = allocate_vmxon_region()?;
    initialize_vmxon_region(vmxon_region)?;


    unsafe { 
        Cr4::write(Cr4::read() | Cr4Flags::VIRTUAL_MACHINE_EXTENSIONS);
        info!("cr4 set");
        //vmx::vmxon(vmxon_region).expect("Failed to start VMX operation"); 
    }  

    //Step 6: 启动 VMXON
    unsafe {
        let vmxon_result: u64;
        asm!(
            "vmxon [{}]",
            in(reg) &vmxon_region,
            out("rax") vmxon_result,
        );
        if vmxon_result != 0 {
            info!("VMXON failed with result: {:?}", vmxon_result);
        }
    }

    info!("first VMX enabled");
    Ok(())
}

// 分配 VMXON 区域 (4KB 对齐)
fn allocate_vmxon_region() -> Result<u64, &'static str> {
    // 在这里，你需要分配内存并确保其物理地址是 4KB 对齐的
    // 通常在内核模式下，可以使用相关的内存管理功能
    let region = allocate_aligned_memory(VMXON_REGION_ALIGN, VMXON_REGION_ALIGN)?;
    Ok(region)
}

// 初始化 VMXON 区域
fn initialize_vmxon_region(vmxon_region: u64) -> Result<(), &'static str> {
    // 根据 Intel SDM，VMXON 区域需要将第一个 u32 设置为 VMCS 修订 ID
    let vmx_basic = unsafe { rdmsr(IA32_VMX_BASIC) };
    let revision_id = vmx_basic as u32;

    unsafe {
        let vmxon_ptr = vmxon_region as *mut u32;
        *vmxon_ptr = revision_id;
    }

    Ok(())
}

use core::alloc::Layout;
use x86_64::VirtAddr;
use x86_64::structures::paging::Mapper;
use alloc::boxed::Box;
fn allocate_aligned_memory(align: usize, size: usize) -> Result<u64, &'static str> {
    // 确保 align 是 2 的幂次
    if !align.is_power_of_two() {
        return Err("Alignment must be a power of two");
    }

    // 创建一个内存布局（Layout），它要求大小和对齐方式
    let layout = Layout::from_size_align(size, align).map_err(|_| "Invalid layout")?;

    // 分配内存
    let memory: *mut u8 = unsafe { alloc::alloc::alloc(layout) };
    
    // 检查分配是否成功
    if memory.is_null() {
        return Err("Failed to allocate aligned memory");
    }
    info!("Allocated memory at 0x{:?}", memory);

        // 获取物理地址（假设有 `mapper` 和 `frame_allocator`）
        let virt_addr = VirtAddr::new(memory as u64);
        let mapper = unsafe {&MAPPER.as_ref().lock().unwrap()};
        let page = x86_64::structures::paging::Page::containing_address(virt_addr);
        let frame = mapper.translate_page(page).unwrap();
        let phys_addr = frame.start_address().as_u64();
    
        info!("Allocated memory at virtual address 0x{:x}, physical address 0x{:x}", virt_addr.as_u64(), phys_addr);
    

    // 返回分配的地址，并确保类型转换
    Ok(memory as u64)
}
