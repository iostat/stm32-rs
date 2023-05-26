///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Clock control register
    pub cr: CR,
    ///0x04 - Internal clock sources calibration register
    pub icscr: ICSCR,
    ///0x08 - Clock configuration register
    pub cfgr: CFGR,
    ///0x0c - PLLSYS configuration register
    pub pllcfgr: PLLCFGR,
    ///0x10 - PLLSAI1 configuration register
    pub pllsai1cfgr: PLLSAI1CFGR,
    _reserved5: [u8; 0x04],
    ///0x18 - Clock interrupt enable register
    pub cier: CIER,
    ///0x1c - Clock interrupt flag register
    pub cifr: CIFR,
    ///0x20 - Clock interrupt clear register
    pub cicr: CICR,
    ///0x24 - Step Down converter control register
    pub smpscr: SMPSCR,
    ///0x28 - AHB1 peripheral reset register
    pub ahb1rstr: AHB1RSTR,
    ///0x2c - AHB2 peripheral reset register
    pub ahb2rstr: AHB2RSTR,
    ///0x30 - AHB3 peripheral reset register
    pub ahb3rstr: AHB3RSTR,
    _reserved12: [u8; 0x04],
    ///0x38 - APB1 peripheral reset register 1
    pub apb1rstr1: APB1RSTR1,
    ///0x3c - APB1 peripheral reset register 2
    pub apb1rstr2: APB1RSTR2,
    ///0x40 - APB2 peripheral reset register
    pub apb2rstr: APB2RSTR,
    ///0x44 - APB3 peripheral reset register
    pub apb3rstr: APB3RSTR,
    ///0x48 - AHB1 peripheral clock enable register
    pub ahb1enr: AHB1ENR,
    ///0x4c - AHB2 peripheral clock enable register
    pub ahb2enr: AHB2ENR,
    ///0x50 - AHB3 peripheral clock enable register
    pub ahb3enr: AHB3ENR,
    _reserved19: [u8; 0x04],
    ///0x58 - APB1ENR1
    pub apb1enr1: APB1ENR1,
    ///0x5c - APB1 peripheral clock enable register 2
    pub apb1enr2: APB1ENR2,
    ///0x60 - APB2ENR
    pub apb2enr: APB2ENR,
    _reserved22: [u8; 0x04],
    ///0x68 - AHB1 peripheral clocks enable in Sleep and Stop modes register
    pub ahb1smenr: AHB1SMENR,
    ///0x6c - AHB2 peripheral clocks enable in Sleep and Stop modes register
    pub ahb2smenr: AHB2SMENR,
    ///0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register
    pub ahb3smenr: AHB3SMENR,
    _reserved25: [u8; 0x04],
    ///0x78 - APB1SMENR1
    pub apb1smenr1: APB1SMENR1,
    ///0x7c - APB1 peripheral clocks enable in Sleep and Stop modes register 2
    pub apb1smenr2: APB1SMENR2,
    ///0x80 - APB2SMENR
    pub apb2smenr: APB2SMENR,
    _reserved28: [u8; 0x04],
    ///0x88 - CCIPR
    pub ccipr: CCIPR,
    _reserved29: [u8; 0x04],
    ///0x90 - BDCR
    pub bdcr: BDCR,
    ///0x94 - CSR
    pub csr: CSR,
    ///0x98 - Clock recovery RC register
    pub crrcr: CRRCR,
    ///0x9c - Clock HSE register
    pub hsecr: HSECR,
    _reserved33: [u8; 0x68],
    ///0x108 - Extended clock recovery register
    pub extcfgr: EXTCFGR,
    _reserved34: [u8; 0x3c],
    ///0x148 - CPU2 AHB1 peripheral clock enable register
    pub c2ahb1enr: C2AHB1ENR,
    ///0x14c - CPU2 AHB2 peripheral clock enable register
    pub c2ahb2enr: C2AHB2ENR,
    ///0x150 - CPU2 AHB3 peripheral clock enable register
    pub c2ahb3enr: C2AHB3ENR,
    _reserved37: [u8; 0x04],
    ///0x158 - CPU2 APB1ENR1
    pub c2apb1enr1: C2APB1ENR1,
    ///0x15c - CPU2 APB1 peripheral clock enable register 2
    pub c2apb1enr2: C2APB1ENR2,
    ///0x160 - CPU2 APB2ENR
    pub c2apb2enr: C2APB2ENR,
    ///0x164 - CPU2 APB3ENR
    pub c2apb3enr: C2APB3ENR,
    ///0x168 - CPU2 AHB1 peripheral clocks enable in Sleep and Stop modes register
    pub c2ahb1smenr: C2AHB1SMENR,
    ///0x16c - CPU2 AHB2 peripheral clocks enable in Sleep and Stop modes register
    pub c2ahb2smenr: C2AHB2SMENR,
    ///0x170 - CPU2 AHB3 peripheral clocks enable in Sleep and Stop modes register
    pub c2ahb3smenr: C2AHB3SMENR,
    _reserved44: [u8; 0x04],
    ///0x178 - CPU2 APB1SMENR1
    pub c2apb1smenr1: C2APB1SMENR1,
    ///0x17c - CPU2 APB1 peripheral clocks enable in Sleep and Stop modes register 2
    pub c2apb1smenr2: C2APB1SMENR2,
    ///0x180 - CPU2 APB2SMENR
    pub c2apb2smenr: C2APB2SMENR,
    ///0x184 - CPU2 APB3SMENR
    pub c2apb3smenr: C2APB3SMENR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Clock control register
pub mod cr;
///ICSCR (rw) register accessor: an alias for `Reg<ICSCR_SPEC>`
pub type ICSCR = crate::Reg<icscr::ICSCR_SPEC>;
///Internal clock sources calibration register
pub mod icscr;
///CFGR (rw) register accessor: an alias for `Reg<CFGR_SPEC>`
pub type CFGR = crate::Reg<cfgr::CFGR_SPEC>;
///Clock configuration register
pub mod cfgr;
///PLLCFGR (rw) register accessor: an alias for `Reg<PLLCFGR_SPEC>`
pub type PLLCFGR = crate::Reg<pllcfgr::PLLCFGR_SPEC>;
///PLLSYS configuration register
pub mod pllcfgr;
///PLLSAI1CFGR (rw) register accessor: an alias for `Reg<PLLSAI1CFGR_SPEC>`
pub type PLLSAI1CFGR = crate::Reg<pllsai1cfgr::PLLSAI1CFGR_SPEC>;
///PLLSAI1 configuration register
pub mod pllsai1cfgr;
///CIER (rw) register accessor: an alias for `Reg<CIER_SPEC>`
pub type CIER = crate::Reg<cier::CIER_SPEC>;
///Clock interrupt enable register
pub mod cier;
///CIFR (r) register accessor: an alias for `Reg<CIFR_SPEC>`
pub type CIFR = crate::Reg<cifr::CIFR_SPEC>;
///Clock interrupt flag register
pub mod cifr;
///CICR (w) register accessor: an alias for `Reg<CICR_SPEC>`
pub type CICR = crate::Reg<cicr::CICR_SPEC>;
///Clock interrupt clear register
pub mod cicr;
///SMPSCR (rw) register accessor: an alias for `Reg<SMPSCR_SPEC>`
pub type SMPSCR = crate::Reg<smpscr::SMPSCR_SPEC>;
///Step Down converter control register
pub mod smpscr;
///AHB1RSTR (rw) register accessor: an alias for `Reg<AHB1RSTR_SPEC>`
pub type AHB1RSTR = crate::Reg<ahb1rstr::AHB1RSTR_SPEC>;
///AHB1 peripheral reset register
pub mod ahb1rstr;
///AHB2RSTR (rw) register accessor: an alias for `Reg<AHB2RSTR_SPEC>`
pub type AHB2RSTR = crate::Reg<ahb2rstr::AHB2RSTR_SPEC>;
///AHB2 peripheral reset register
pub mod ahb2rstr;
///AHB3RSTR (rw) register accessor: an alias for `Reg<AHB3RSTR_SPEC>`
pub type AHB3RSTR = crate::Reg<ahb3rstr::AHB3RSTR_SPEC>;
///AHB3 peripheral reset register
pub mod ahb3rstr;
///APB1RSTR1 (rw) register accessor: an alias for `Reg<APB1RSTR1_SPEC>`
pub type APB1RSTR1 = crate::Reg<apb1rstr1::APB1RSTR1_SPEC>;
///APB1 peripheral reset register 1
pub mod apb1rstr1;
///APB1RSTR2 (rw) register accessor: an alias for `Reg<APB1RSTR2_SPEC>`
pub type APB1RSTR2 = crate::Reg<apb1rstr2::APB1RSTR2_SPEC>;
///APB1 peripheral reset register 2
pub mod apb1rstr2;
///APB2RSTR (rw) register accessor: an alias for `Reg<APB2RSTR_SPEC>`
pub type APB2RSTR = crate::Reg<apb2rstr::APB2RSTR_SPEC>;
///APB2 peripheral reset register
pub mod apb2rstr;
///APB3RSTR (rw) register accessor: an alias for `Reg<APB3RSTR_SPEC>`
pub type APB3RSTR = crate::Reg<apb3rstr::APB3RSTR_SPEC>;
///APB3 peripheral reset register
pub mod apb3rstr;
///AHB1ENR (rw) register accessor: an alias for `Reg<AHB1ENR_SPEC>`
pub type AHB1ENR = crate::Reg<ahb1enr::AHB1ENR_SPEC>;
///AHB1 peripheral clock enable register
pub mod ahb1enr;
///AHB2ENR (rw) register accessor: an alias for `Reg<AHB2ENR_SPEC>`
pub type AHB2ENR = crate::Reg<ahb2enr::AHB2ENR_SPEC>;
///AHB2 peripheral clock enable register
pub mod ahb2enr;
///AHB3ENR (rw) register accessor: an alias for `Reg<AHB3ENR_SPEC>`
pub type AHB3ENR = crate::Reg<ahb3enr::AHB3ENR_SPEC>;
///AHB3 peripheral clock enable register
pub mod ahb3enr;
///APB1ENR1 (rw) register accessor: an alias for `Reg<APB1ENR1_SPEC>`
pub type APB1ENR1 = crate::Reg<apb1enr1::APB1ENR1_SPEC>;
///APB1ENR1
pub mod apb1enr1;
///APB1ENR2 (rw) register accessor: an alias for `Reg<APB1ENR2_SPEC>`
pub type APB1ENR2 = crate::Reg<apb1enr2::APB1ENR2_SPEC>;
///APB1 peripheral clock enable register 2
pub mod apb1enr2;
///APB2ENR (rw) register accessor: an alias for `Reg<APB2ENR_SPEC>`
pub type APB2ENR = crate::Reg<apb2enr::APB2ENR_SPEC>;
///APB2ENR
pub mod apb2enr;
///AHB1SMENR (rw) register accessor: an alias for `Reg<AHB1SMENR_SPEC>`
pub type AHB1SMENR = crate::Reg<ahb1smenr::AHB1SMENR_SPEC>;
///AHB1 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb1smenr;
///AHB2SMENR (rw) register accessor: an alias for `Reg<AHB2SMENR_SPEC>`
pub type AHB2SMENR = crate::Reg<ahb2smenr::AHB2SMENR_SPEC>;
///AHB2 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb2smenr;
///AHB3SMENR (rw) register accessor: an alias for `Reg<AHB3SMENR_SPEC>`
pub type AHB3SMENR = crate::Reg<ahb3smenr::AHB3SMENR_SPEC>;
///AHB3 peripheral clocks enable in Sleep and Stop modes register
pub mod ahb3smenr;
///APB1SMENR1 (rw) register accessor: an alias for `Reg<APB1SMENR1_SPEC>`
pub type APB1SMENR1 = crate::Reg<apb1smenr1::APB1SMENR1_SPEC>;
///APB1SMENR1
pub mod apb1smenr1;
///APB1SMENR2 (rw) register accessor: an alias for `Reg<APB1SMENR2_SPEC>`
pub type APB1SMENR2 = crate::Reg<apb1smenr2::APB1SMENR2_SPEC>;
///APB1 peripheral clocks enable in Sleep and Stop modes register 2
pub mod apb1smenr2;
///APB2SMENR (rw) register accessor: an alias for `Reg<APB2SMENR_SPEC>`
pub type APB2SMENR = crate::Reg<apb2smenr::APB2SMENR_SPEC>;
///APB2SMENR
pub mod apb2smenr;
///CCIPR (rw) register accessor: an alias for `Reg<CCIPR_SPEC>`
pub type CCIPR = crate::Reg<ccipr::CCIPR_SPEC>;
///CCIPR
pub mod ccipr;
///BDCR (rw) register accessor: an alias for `Reg<BDCR_SPEC>`
pub type BDCR = crate::Reg<bdcr::BDCR_SPEC>;
///BDCR
pub mod bdcr;
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///CSR
pub mod csr;
///CRRCR (rw) register accessor: an alias for `Reg<CRRCR_SPEC>`
pub type CRRCR = crate::Reg<crrcr::CRRCR_SPEC>;
///Clock recovery RC register
pub mod crrcr;
///HSECR (rw) register accessor: an alias for `Reg<HSECR_SPEC>`
pub type HSECR = crate::Reg<hsecr::HSECR_SPEC>;
///Clock HSE register
pub mod hsecr;
///EXTCFGR (rw) register accessor: an alias for `Reg<EXTCFGR_SPEC>`
pub type EXTCFGR = crate::Reg<extcfgr::EXTCFGR_SPEC>;
///Extended clock recovery register
pub mod extcfgr;
///C2AHB1ENR (rw) register accessor: an alias for `Reg<C2AHB1ENR_SPEC>`
pub type C2AHB1ENR = crate::Reg<c2ahb1enr::C2AHB1ENR_SPEC>;
///CPU2 AHB1 peripheral clock enable register
pub mod c2ahb1enr;
///C2AHB2ENR (rw) register accessor: an alias for `Reg<C2AHB2ENR_SPEC>`
pub type C2AHB2ENR = crate::Reg<c2ahb2enr::C2AHB2ENR_SPEC>;
///CPU2 AHB2 peripheral clock enable register
pub mod c2ahb2enr;
///C2AHB3ENR (rw) register accessor: an alias for `Reg<C2AHB3ENR_SPEC>`
pub type C2AHB3ENR = crate::Reg<c2ahb3enr::C2AHB3ENR_SPEC>;
///CPU2 AHB3 peripheral clock enable register
pub mod c2ahb3enr;
///C2APB1ENR1 (rw) register accessor: an alias for `Reg<C2APB1ENR1_SPEC>`
pub type C2APB1ENR1 = crate::Reg<c2apb1enr1::C2APB1ENR1_SPEC>;
///CPU2 APB1ENR1
pub mod c2apb1enr1;
///C2APB1ENR2 (rw) register accessor: an alias for `Reg<C2APB1ENR2_SPEC>`
pub type C2APB1ENR2 = crate::Reg<c2apb1enr2::C2APB1ENR2_SPEC>;
///CPU2 APB1 peripheral clock enable register 2
pub mod c2apb1enr2;
///C2APB2ENR (rw) register accessor: an alias for `Reg<C2APB2ENR_SPEC>`
pub type C2APB2ENR = crate::Reg<c2apb2enr::C2APB2ENR_SPEC>;
///CPU2 APB2ENR
pub mod c2apb2enr;
///C2APB3ENR (rw) register accessor: an alias for `Reg<C2APB3ENR_SPEC>`
pub type C2APB3ENR = crate::Reg<c2apb3enr::C2APB3ENR_SPEC>;
///CPU2 APB3ENR
pub mod c2apb3enr;
///C2AHB1SMENR (rw) register accessor: an alias for `Reg<C2AHB1SMENR_SPEC>`
pub type C2AHB1SMENR = crate::Reg<c2ahb1smenr::C2AHB1SMENR_SPEC>;
///CPU2 AHB1 peripheral clocks enable in Sleep and Stop modes register
pub mod c2ahb1smenr;
///C2AHB2SMENR (rw) register accessor: an alias for `Reg<C2AHB2SMENR_SPEC>`
pub type C2AHB2SMENR = crate::Reg<c2ahb2smenr::C2AHB2SMENR_SPEC>;
///CPU2 AHB2 peripheral clocks enable in Sleep and Stop modes register
pub mod c2ahb2smenr;
///C2AHB3SMENR (rw) register accessor: an alias for `Reg<C2AHB3SMENR_SPEC>`
pub type C2AHB3SMENR = crate::Reg<c2ahb3smenr::C2AHB3SMENR_SPEC>;
///CPU2 AHB3 peripheral clocks enable in Sleep and Stop modes register
pub mod c2ahb3smenr;
///C2APB1SMENR1 (rw) register accessor: an alias for `Reg<C2APB1SMENR1_SPEC>`
pub type C2APB1SMENR1 = crate::Reg<c2apb1smenr1::C2APB1SMENR1_SPEC>;
///CPU2 APB1SMENR1
pub mod c2apb1smenr1;
///C2APB1SMENR2 (rw) register accessor: an alias for `Reg<C2APB1SMENR2_SPEC>`
pub type C2APB1SMENR2 = crate::Reg<c2apb1smenr2::C2APB1SMENR2_SPEC>;
///CPU2 APB1 peripheral clocks enable in Sleep and Stop modes register 2
pub mod c2apb1smenr2;
///C2APB2SMENR (rw) register accessor: an alias for `Reg<C2APB2SMENR_SPEC>`
pub type C2APB2SMENR = crate::Reg<c2apb2smenr::C2APB2SMENR_SPEC>;
///CPU2 APB2SMENR
pub mod c2apb2smenr;
///C2APB3SMENR (rw) register accessor: an alias for `Reg<C2APB3SMENR_SPEC>`
pub type C2APB3SMENR = crate::Reg<c2apb3smenr::C2APB3SMENR_SPEC>;
///CPU2 APB3SMENR
pub mod c2apb3smenr;
