use x86_64::structures::paging::PhysFrame;

//vmx region
pub struct VmxRegion {
    frame: PhysFrame,
}


// impl VmxRegion {
//     pub const unsafe fn uninit() -> Self {
//         VmxRegion {
//             frame: PhysFrame::from_start_address(0x0).expect("could not create frame"),
//         }
//     }
//     pub fn new(revision_id: u32, shadow_indicator: bool) -> RvmResult<Self> {
//         let frame = 
//     }
// }