///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - GPIO port mode register
    pub moder: MODER,
    ///0x04 - GPIO port output type register
    pub otyper: OTYPER,
    ///0x08 - GPIO port output speed register
    pub ospeedr: OSPEEDR,
    ///0x0c - GPIO port pull-up/pull-down register
    pub pupdr: PUPDR,
    ///0x10 - GPIO port input data register
    pub idr: IDR,
    ///0x14 - GPIO port output data register
    pub odr: ODR,
    ///0x18 - GPIO port bit set/reset register
    pub bsrr: BSRR,
    ///0x1c - GPIO port configuration lock register
    pub lckr: LCKR,
    ///0x20 - GPIO alternate function low register
    pub afrl: AFRL,
    ///0x24 - GPIO alternate function high register
    pub afrh: AFRH,
    ///0x28 - port bit reset register
    pub brr: BRR,
}
///MODER (rw) register accessor: an alias for `Reg<MODER_SPEC>`
pub type MODER = crate::Reg<moder::MODER_SPEC>;
///GPIO port mode register
pub mod moder;
///OTYPER (rw) register accessor: an alias for `Reg<OTYPER_SPEC>`
pub type OTYPER = crate::Reg<otyper::OTYPER_SPEC>;
///GPIO port output type register
pub mod otyper;
///OSPEEDR (rw) register accessor: an alias for `Reg<OSPEEDR_SPEC>`
pub type OSPEEDR = crate::Reg<ospeedr::OSPEEDR_SPEC>;
///GPIO port output speed register
pub mod ospeedr;
///PUPDR (rw) register accessor: an alias for `Reg<PUPDR_SPEC>`
pub type PUPDR = crate::Reg<pupdr::PUPDR_SPEC>;
///GPIO port pull-up/pull-down register
pub mod pupdr;
///IDR (r) register accessor: an alias for `Reg<IDR_SPEC>`
pub type IDR = crate::Reg<idr::IDR_SPEC>;
///GPIO port input data register
pub mod idr;
///ODR (rw) register accessor: an alias for `Reg<ODR_SPEC>`
pub type ODR = crate::Reg<odr::ODR_SPEC>;
///GPIO port output data register
pub mod odr;
///BSRR (w) register accessor: an alias for `Reg<BSRR_SPEC>`
pub type BSRR = crate::Reg<bsrr::BSRR_SPEC>;
///GPIO port bit set/reset register
pub mod bsrr;
///LCKR (rw) register accessor: an alias for `Reg<LCKR_SPEC>`
pub type LCKR = crate::Reg<lckr::LCKR_SPEC>;
///GPIO port configuration lock register
pub mod lckr;
///AFRL (rw) register accessor: an alias for `Reg<AFRL_SPEC>`
pub type AFRL = crate::Reg<afrl::AFRL_SPEC>;
///GPIO alternate function low register
pub mod afrl;
///AFRH (rw) register accessor: an alias for `Reg<AFRH_SPEC>`
pub type AFRH = crate::Reg<afrh::AFRH_SPEC>;
///GPIO alternate function high register
pub mod afrh;
///BRR (w) register accessor: an alias for `Reg<BRR_SPEC>`
pub type BRR = crate::Reg<brr::BRR_SPEC>;
///port bit reset register
pub mod brr;
