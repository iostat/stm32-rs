///Register block
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    ///0x04..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    pub ch: [CH; 2],
    ///0x44 - PDM control register
    pub pdmcr: PDMCR,
    ///0x48 - PDM delay register
    pub pdmdly: PDMDLY,
}
impl RegisterBlock {
    ///0x04..0x24 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    #[inline(always)]
    pub fn cha(&self) -> &CH {
        &self.ch[0]
    }
    ///0x24..0x44 - Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
    #[inline(always)]
    pub fn chb(&self) -> &CH {
        &self.ch[1]
    }
}
///Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
pub use self::ch::CH;
///Cluster
///Cluster CH%s, containing ?CR1, ?CR2, ?FRCR, ?SLOTR, ?IM, ?SR, ?CLRFR, ?DR
pub mod ch;
///PDMCR (rw) register accessor: an alias for `Reg<PDMCR_SPEC>`
pub type PDMCR = crate::Reg<pdmcr::PDMCR_SPEC>;
///PDM control register
pub mod pdmcr;
///PDMDLY (rw) register accessor: an alias for `Reg<PDMDLY_SPEC>`
pub type PDMDLY = crate::Reg<pdmdly::PDMDLY_SPEC>;
///PDM delay register
pub mod pdmdly;
