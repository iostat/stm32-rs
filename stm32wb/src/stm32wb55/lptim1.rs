///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Interrupt and Status Register
    pub isr: ISR,
    ///0x04 - Interrupt Clear Register
    pub icr: ICR,
    ///0x08 - Interrupt Enable Register
    pub ier: IER,
    ///0x0c - Configuration Register
    pub cfgr: CFGR,
    ///0x10 - Control Register
    pub cr: CR,
    ///0x14 - Compare Register
    pub cmp: CMP,
    ///0x18 - Autoreload Register
    pub arr: ARR,
    ///0x1c - Counter Register
    pub cnt: CNT,
    ///0x20 - Option Register
    pub or: OR,
}
///ISR (r) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt and Status Register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///Interrupt Clear Register
pub mod icr;
///IER (rw) register accessor: an alias for `Reg<IER_SPEC>`
pub type IER = crate::Reg<ier::IER_SPEC>;
///Interrupt Enable Register
pub mod ier;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///Configuration Register
pub mod cfgr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control Register
pub mod cr;
///CMP (rw) register accessor: an alias for `Reg<CMP_SPEC>`
pub type CMP = crate::Reg<cmp::CMP_SPEC>;
///Compare Register
pub mod cmp;
///ARR (rw) register accessor: an alias for `Reg<ARR_SPEC>`
pub type ARR = crate::Reg<arr::ARR_SPEC>;
///Autoreload Register
pub mod arr;
///CNT (r) register accessor: an alias for `Reg<CNT_SPEC>`
pub type CNT = crate::Reg<cnt::CNT_SPEC>;
///Counter Register
pub mod cnt;
///OR (rw) register accessor: an alias for `Reg<OR_SPEC>`
pub type OR = crate::Reg<or::OR_SPEC>;
///Option Register
pub mod or;
