///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Data register
    pub dr: DR,
    ///0x04 - Independent data register
    pub idr: IDR,
    ///0x08 - Control register
    pub cr: CR,
    _reserved3: [u8; 0x04],
    ///0x10 - Initial CRC value
    pub init: INIT,
    ///0x14 - polynomial
    pub pol: POL,
}
///DR (rw) register accessor: an alias for `Reg<DR_SPEC>`
pub type DR = crate::Reg<dr::DR_SPEC>;
///Data register
pub mod dr;
///IDR (rw) register accessor: an alias for `Reg<IDR_SPEC>`
pub type IDR = crate::Reg<idr::IDR_SPEC>;
///Independent data register
pub mod idr;
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///Control register
pub mod cr;
///INIT (rw) register accessor: an alias for `Reg<INIT_SPEC>`
pub type INIT = crate::Reg<init::INIT_SPEC>;
///Initial CRC value
pub mod init;
///POL (rw) register accessor: an alias for `Reg<POL_SPEC>`
pub type POL = crate::Reg<pol::POL_SPEC>;
///polynomial
pub mod pol;
