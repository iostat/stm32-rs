///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Control register 1
    pub cr1: CR1,
    ///0x04 - Control register 2
    pub cr2: CR2,
    ///0x08 - Own address register 1
    pub oar1: OAR1,
    ///0x0c - Own address register 2
    pub oar2: OAR2,
    ///0x10 - Timing register
    pub timingr: TIMINGR,
    ///0x14 - Status register 1
    pub timeoutr: TIMEOUTR,
    ///0x18 - Interrupt and Status register
    pub isr: ISR,
    ///0x1c - Interrupt clear register
    pub icr: ICR,
    ///0x20 - PEC register
    pub pecr: PECR,
    ///0x24 - Receive data register
    pub rxdr: RXDR,
    ///0x28 - Transmit data register
    pub txdr: TXDR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///Control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///Control register 2
pub mod cr2;
///OAR1 (rw) register accessor: an alias for `Reg<OAR1_SPEC>`
pub type OAR1 = crate::Reg<oar1::OAR1_SPEC>;
///Own address register 1
pub mod oar1;
///OAR2 (rw) register accessor: an alias for `Reg<OAR2_SPEC>`
pub type OAR2 = crate::Reg<oar2::OAR2_SPEC>;
///Own address register 2
pub mod oar2;
///TIMINGR (rw) register accessor: an alias for `Reg<TIMINGR_SPEC>`
pub type TIMINGR = crate::Reg<timingr::TIMINGR_SPEC>;
///Timing register
pub mod timingr;
///TIMEOUTR (rw) register accessor: an alias for `Reg<TIMEOUTR_SPEC>`
pub type TIMEOUTR = crate::Reg<timeoutr::TIMEOUTR_SPEC>;
///Status register 1
pub mod timeoutr;
///ISR (rw) register accessor: an alias for `Reg<ISR_SPEC>`
pub type ISR = crate::Reg<isr::ISR_SPEC>;
///Interrupt and Status register
pub mod isr;
///ICR (w) register accessor: an alias for `Reg<ICR_SPEC>`
pub type ICR = crate::Reg<icr::ICR_SPEC>;
///Interrupt clear register
pub mod icr;
///PECR (r) register accessor: an alias for `Reg<PECR_SPEC>`
pub type PECR = crate::Reg<pecr::PECR_SPEC>;
///PEC register
pub mod pecr;
///RXDR (r) register accessor: an alias for `Reg<RXDR_SPEC>`
pub type RXDR = crate::Reg<rxdr::RXDR_SPEC>;
///Receive data register
pub mod rxdr;
///TXDR (rw) register accessor: an alias for `Reg<TXDR_SPEC>`
pub type TXDR = crate::Reg<txdr::TXDR_SPEC>;
///Transmit data register
pub mod txdr;
