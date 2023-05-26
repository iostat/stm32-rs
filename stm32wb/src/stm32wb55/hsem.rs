///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00..0x80 - Semaphore %s register
    pub r: [R; 32],
    ///0x80..0x100 - Semaphore %s read lock register
    pub rlr: [RLR; 32],
    ///0x100 - HSEM Interrupt enable register
    pub c1ier: C1IER,
    ///0x104 - HSEM Interrupt clear register
    pub c1icr: C1ICR,
    ///0x108 - HSEM Interrupt status register
    pub c1isr: C1ISR,
    ///0x10c - HSEM Masked interrupt status register
    pub c1misr: C1MISR,
    ///0x110 - HSEM Interrupt enable register
    pub c2ier: C2IER,
    ///0x114 - HSEM Interrupt clear register
    pub c2icr: C2ICR,
    ///0x118 - HSEM Interrupt status register
    pub c2isr: C2ISR,
    ///0x11c - HSEM Masked interrupt status register
    pub c2misr: C2MISR,
    _reserved10: [u8; 0x20],
    ///0x140 - Semaphore Clear register
    pub cr: CR,
    ///0x144 - Interrupt clear register
    pub keyr: KEYR,
    _reserved12: [u8; 0x02a4],
    ///0x3ec - Semaphore hardware configuration register 2
    pub hwcfgr2: HWCFGR2,
    ///0x3f0 - Semaphore hardware configuration register 1
    pub hwcfgr1: HWCFGR1,
    ///0x3f4 - HSEM version register
    pub verr: VERR,
    ///0x3f8 - HSEM indentification register
    pub ipidr: IPIDR,
    ///0x3fc - HSEM size indentification register
    pub sidr: SIDR,
}
///R (rw) register accessor: an alias for `Reg<R_SPEC>`
pub type R = crate::Reg<r::R_SPEC>;
///Semaphore %s register
pub mod r;
///RLR (r) register accessor: an alias for `Reg<RLR_SPEC>`
pub type RLR = crate::Reg<rlr::RLR_SPEC>;
///Semaphore %s read lock register
pub mod rlr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Semaphore Clear register
pub mod cr;
///KEYR (rw) register accessor: an alias for `Reg<KEYR_SPEC>`
pub type KEYR = crate::Reg<keyr::KEYR_SPEC>;
///Interrupt clear register
pub mod keyr;
///HWCFGR2 (r) register accessor: an alias for `Reg<HWCFGR2_SPEC>`
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2_SPEC>;
///Semaphore hardware configuration register 2
pub mod hwcfgr2;
///HWCFGR1 (r) register accessor: an alias for `Reg<HWCFGR1_SPEC>`
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1_SPEC>;
///Semaphore hardware configuration register 1
pub mod hwcfgr1;
///VERR (r) register accessor: an alias for `Reg<VERR_SPEC>`
pub type VERR = crate::Reg<verr::VERR_SPEC>;
///HSEM version register
pub mod verr;
///IPIDR (r) register accessor: an alias for `Reg<IPIDR_SPEC>`
pub type IPIDR = crate::Reg<ipidr::IPIDR_SPEC>;
///HSEM indentification register
pub mod ipidr;
///SIDR (r) register accessor: an alias for `Reg<SIDR_SPEC>`
pub type SIDR = crate::Reg<sidr::SIDR_SPEC>;
///HSEM size indentification register
pub mod sidr;
///C1IER (rw) register accessor: an alias for `Reg<C1IER_SPEC>`
pub type C1IER = crate::Reg<c1ier::C1IER_SPEC>;
///HSEM Interrupt enable register
pub mod c1ier;
///C1ICR (rw) register accessor: an alias for `Reg<C1ICR_SPEC>`
pub type C1ICR = crate::Reg<c1icr::C1ICR_SPEC>;
///HSEM Interrupt clear register
pub mod c1icr;
///C1ISR (r) register accessor: an alias for `Reg<C1ISR_SPEC>`
pub type C1ISR = crate::Reg<c1isr::C1ISR_SPEC>;
///HSEM Interrupt status register
pub mod c1isr;
///C1MISR (r) register accessor: an alias for `Reg<C1MISR_SPEC>`
pub type C1MISR = crate::Reg<c1misr::C1MISR_SPEC>;
///HSEM Masked interrupt status register
pub mod c1misr;
///C2IER (rw) register accessor: an alias for `Reg<C2IER_SPEC>`
pub type C2IER = crate::Reg<c2ier::C2IER_SPEC>;
///HSEM Interrupt enable register
pub mod c2ier;
///C2ICR (rw) register accessor: an alias for `Reg<C2ICR_SPEC>`
pub type C2ICR = crate::Reg<c2icr::C2ICR_SPEC>;
///HSEM Interrupt clear register
pub mod c2icr;
///C2ISR (r) register accessor: an alias for `Reg<C2ISR_SPEC>`
pub type C2ISR = crate::Reg<c2isr::C2ISR_SPEC>;
///HSEM Interrupt status register
pub mod c2isr;
///C2MISR (r) register accessor: an alias for `Reg<C2MISR_SPEC>`
pub type C2MISR = crate::Reg<c2misr::C2MISR_SPEC>;
///HSEM Masked interrupt status register
pub mod c2misr;
