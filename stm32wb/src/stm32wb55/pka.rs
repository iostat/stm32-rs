///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register
    pub cr: CR,
    ///0x04 - PKA status register
    pub sr: SR,
    ///0x08 - PKA clear flag register
    pub clrfr: CLRFR,
    _reserved3: [u8; 0x1fe8],
    ///0x1ff4 - PKA version register
    pub verr: VERR,
    ///0x1ff8 - PKA identification register
    pub ipidr: IPIDR,
    ///0x1ffc - PKA size ID register
    pub sidr: SIDR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register
pub mod cr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///PKA status register
pub mod sr;
///CLRFR (rw) register accessor: an alias for `Reg<CLRFR_SPEC>`
pub type CLRFR = crate::Reg<clrfr::CLRFR_SPEC>;
///PKA clear flag register
pub mod clrfr;
///VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///PKA version register
pub mod verr;
///IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///PKA identification register
pub mod ipidr;
///SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///PKA size ID register
pub mod sidr;
