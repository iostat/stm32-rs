///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register CPU1
    pub c1cr: C1CR,
    ///0x04 - Mask register CPU1
    pub c1mr: C1MR,
    ///0x08 - Status Set or Clear register CPU1
    pub c1scr: C1SCR,
    ///0x0c - CPU1 to CPU2 status register
    pub c1toc2sr: C1TOC2SR,
    ///0x10 - Control register CPU2
    pub c2cr: C2CR,
    ///0x14 - Mask register CPU2
    pub c2mr: C2MR,
    ///0x18 - Status Set or Clear register CPU2
    pub c2scr: C2SCR,
    ///0x1c - CPU2 to CPU1 status register
    pub c2toc1sr: C2TOC1SR,
    _reserved8: [u8; 0x03d0],
    ///0x3f0 - IPCC Hardware configuration register
    pub hwcfgr: HWCFGR,
    ///0x3f4 - IPCC version register
    pub verr: VERR,
    ///0x3f8 - IPCC indentification register
    pub ipidr: IPIDR,
    ///0x3fc - IPCC size indentification register
    pub sidr: SIDR,
}
///C1CR (rw) register accessor: an alias for `Reg<C1CR_SPEC>`
pub type C1CR = crate::Reg<c1cr::C1CR_SPEC>;
///Control register CPU1
pub mod c1cr;
///C1MR (rw) register accessor: an alias for `Reg<C1MR_SPEC>`
pub type C1MR = crate::Reg<c1mr::C1MR_SPEC>;
///Mask register CPU1
pub mod c1mr;
///C1SCR (w) register accessor: an alias for `Reg<C1SCR_SPEC>`
pub type C1SCR = crate::Reg<c1scr::C1SCR_SPEC>;
///Status Set or Clear register CPU1
pub mod c1scr;
///C1TOC2SR (r) register accessor: an alias for `Reg<C1TOC2SR_SPEC>`
pub type C1TOC2SR = crate::Reg<c1toc2sr::C1TOC2SR_SPEC>;
///CPU1 to CPU2 status register
pub mod c1toc2sr;
///C2CR (rw) register accessor: an alias for `Reg<C2CR_SPEC>`
pub type C2CR = crate::Reg<c2cr::C2CR_SPEC>;
///Control register CPU2
pub mod c2cr;
///C2MR (rw) register accessor: an alias for `Reg<C2MR_SPEC>`
pub type C2MR = crate::Reg<c2mr::C2MR_SPEC>;
///Mask register CPU2
pub mod c2mr;
///C2SCR (w) register accessor: an alias for `Reg<C2SCR_SPEC>`
pub type C2SCR = crate::Reg<c2scr::C2SCR_SPEC>;
///Status Set or Clear register CPU2
pub mod c2scr;
///C2TOC1SR (r) register accessor: an alias for `Reg<C2TOC1SR_SPEC>`
pub type C2TOC1SR = crate::Reg<c2toc1sr::C2TOC1SR_SPEC>;
///CPU2 to CPU1 status register
pub mod c2toc1sr;
///HWCFGR (r) register accessor: an alias for `Reg<HWCFGR_SPEC>`
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGR_SPEC>;
///IPCC Hardware configuration register
pub mod hwcfgr;
///VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///IPCC version register
pub mod verr;
///IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///IPCC indentification register
pub mod ipidr;
///SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///IPCC size indentification register
pub mod sidr;
