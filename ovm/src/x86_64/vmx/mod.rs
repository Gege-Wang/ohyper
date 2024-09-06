use raw_cpuid::CpuId;
pub fn has_vmx_support() -> bool {
    match CpuId::new().get_feature_info() {
        Some(feature) => feature.has_vmx(),
        None => false,
    }
}