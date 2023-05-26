///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register 1
    pub cr1: CR1,
    ///0x04 - control register 2
    pub cr2: CR2,
    ///0x08 - status register
    pub sr: SR,
    ///0x0c - data register
    pub dr: DR,
    ///0x10 - CRC polynomial register
    pub crcpr: CRCPR,
    ///0x14 - RX CRC register
    pub rxcrcr: RXCRCR,
    ///0x18 - TX CRC register
    pub txcrcr: TXCRCR,
}
///CR1 (rw) register accessor: an alias for `Reg<CR1_SPEC>`
pub type CR1 = crate::Reg<cr1::CR1_SPEC>;
///control register 1
pub mod cr1;
///CR2 (rw) register accessor: an alias for `Reg<CR2_SPEC>`
pub type CR2 = crate::Reg<cr2::CR2_SPEC>;
///control register 2
pub mod cr2;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///data register
pub mod dr;
///CRCPR (rw) register accessor: an alias for `Reg<CRCPR_SPEC>`
pub type CRCPR = crate::Reg<crcpr::CRCPR_SPEC>;
///CRC polynomial register
pub mod crcpr;
///RXCRCR (r) register accessor: an alias for `Reg<RXCRCR_SPEC>`
pub type RXCRCR = crate::Reg<rxcrcr::RXCRCR_SPEC>;
///RX CRC register
pub mod rxcrcr;
///TXCRCR (r) register accessor: an alias for `Reg<TXCRCR_SPEC>`
pub type TXCRCR = crate::Reg<txcrcr::TXCRCR_SPEC>;
///TX CRC register
pub mod txcrcr;
