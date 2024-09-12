mod structs;
use raw_cpuid::CpuId;
use structs::VmxRegion;
pub fn has_vmx_support() -> bool {
    match CpuId::new().get_feature_info() {
        Some(feature) => feature.has_vmx(),
        None => false,
    }
}


//vmx region
pub struct VmxPerState {
    vmcs_revision_id: u32,
    vmx_region: VmxRegion,    
}
