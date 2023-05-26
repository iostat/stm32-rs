///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - device configuration register
    pub dcr: DCR,
    ///0x08 - status register
    pub sr: SR,
    ///0x0c - flag clear register
    pub fcr: FCR,
    ///0x10 - data length register
    pub dlr: DLR,
    ///0x14 - communication configuration register
    pub ccr: CCR,
    ///0x18 - address register
    pub ar: AR,
    ///0x1c - ABR
    pub abr: ABR,
    ///0x20 - data register
    pub dr: DR,
    ///0x24 - polling status mask register
    pub psmkr: PSMKR,
    ///0x28 - polling status match register
    pub psmar: PSMAR,
    ///0x2c - polling interval register
    pub pir: PIR,
    ///0x30 - low-power timeout register
    pub lptr: LPTR,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///DCR (rw) register accessor: an alias for `Reg<DCR_SPEC>`
pub type DCR = crate::Reg<dcr::DCR_SPEC>;
///device configuration register
pub mod dcr;
///SR (r) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///flag clear register
pub mod fcr;
///DLR (rw) register accessor: an alias for `Reg<DLR_SPEC>`
pub type DLR = crate::Reg<dlr::DLR_SPEC>;
///data length register
pub mod dlr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///communication configuration register
pub mod ccr;
///AR (rw) register accessor: an alias for `Reg<AR_SPEC>`
pub type AR = crate::Reg<ar::AR_SPEC>;
///address register
pub mod ar;
///ABR (rw) register accessor: an alias for `Reg<ABR_SPEC>`
pub type ABR = crate::Reg<abr::ABR_SPEC>;
///ABR
pub mod abr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///data register
pub mod dr;
///PSMKR (rw) register accessor: an alias for `Reg<PSMKR_SPEC>`
pub type PSMKR = crate::Reg<psmkr::PSMKR_SPEC>;
///polling status mask register
pub mod psmkr;
///PSMAR (rw) register accessor: an alias for `Reg<PSMAR_SPEC>`
pub type PSMAR = crate::Reg<psmar::PSMAR_SPEC>;
///polling status match register
pub mod psmar;
///PIR (rw) register accessor: an alias for `Reg<PIR_SPEC>`
pub type PIR = crate::Reg<pir::PIR_SPEC>;
///polling interval register
pub mod pir;
///LPTR (rw) register accessor: an alias for `Reg<LPTR_SPEC>`
pub type LPTR = crate::Reg<lptr::LPTR_SPEC>;
///low-power timeout register
pub mod lptr;
