///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - control register
    pub cr: CR,
    ///0x04 - frame control register
    pub fcr: FCR,
    ///0x08 - status register
    pub sr: SR,
    ///0x0c - clear register
    pub clr: CLR,
    _reserved4: [u8; 0x04],
    ///0x14 - display memory
    pub ram_com0: RAM_COM0,
    _reserved5: [u8; 0x04],
    ///0x1c - display memory
    pub ram_com1: RAM_COM1,
    _reserved6: [u8; 0x04],
    ///0x24 - display memory
    pub ram_com2: RAM_COM2,
    _reserved7: [u8; 0x04],
    ///0x2c - display memory
    pub ram_com3: RAM_COM3,
    _reserved8: [u8; 0x04],
    ///0x34 - display memory
    pub ram_com4: RAM_COM4,
    _reserved9: [u8; 0x04],
    ///0x3c - display memory
    pub ram_com5: RAM_COM5,
    _reserved10: [u8; 0x04],
    ///0x44 - display memory
    pub ram_com6: RAM_COM6,
    _reserved11: [u8; 0x04],
    ///0x4c - display memory
    pub ram_com7: RAM_COM7,
}
///CR (rw) register accessor: an alias for `Reg<CR_SPEC>`
pub type CR = crate::Reg<cr::CR_SPEC>;
///control register
pub mod cr;
///FCR (rw) register accessor: an alias for `Reg<FCR_SPEC>`
pub type FCR = crate::Reg<fcr::FCR_SPEC>;
///frame control register
pub mod fcr;
///SR (rw) register accessor: an alias for `Reg<SR_SPEC>`
pub type SR = crate::Reg<sr::SR_SPEC>;
///status register
pub mod sr;
///CLR (w) register accessor: an alias for `Reg<CLR_SPEC>`
pub type CLR = crate::Reg<clr::CLR_SPEC>;
///clear register
pub mod clr;
///RAM_COM0 (rw) register accessor: an alias for `Reg<RAM_COM0_SPEC>`
pub type RAM_COM0 = crate::Reg<ram_com0::RAM_COM0_SPEC>;
///display memory
pub mod ram_com0;
///RAM_COM1 (rw) register accessor: an alias for `Reg<RAM_COM1_SPEC>`
pub type RAM_COM1 = crate::Reg<ram_com1::RAM_COM1_SPEC>;
///display memory
pub mod ram_com1;
///RAM_COM2 (rw) register accessor: an alias for `Reg<RAM_COM2_SPEC>`
pub type RAM_COM2 = crate::Reg<ram_com2::RAM_COM2_SPEC>;
///display memory
pub mod ram_com2;
///RAM_COM3 (rw) register accessor: an alias for `Reg<RAM_COM3_SPEC>`
pub type RAM_COM3 = crate::Reg<ram_com3::RAM_COM3_SPEC>;
///display memory
pub mod ram_com3;
///RAM_COM4 (rw) register accessor: an alias for `Reg<RAM_COM4_SPEC>`
pub type RAM_COM4 = crate::Reg<ram_com4::RAM_COM4_SPEC>;
///display memory
pub mod ram_com4;
///RAM_COM5 (rw) register accessor: an alias for `Reg<RAM_COM5_SPEC>`
pub type RAM_COM5 = crate::Reg<ram_com5::RAM_COM5_SPEC>;
///display memory
pub mod ram_com5;
///RAM_COM6 (rw) register accessor: an alias for `Reg<RAM_COM6_SPEC>`
pub type RAM_COM6 = crate::Reg<ram_com6::RAM_COM6_SPEC>;
///display memory
pub mod ram_com6;
///RAM_COM7 (rw) register accessor: an alias for `Reg<RAM_COM7_SPEC>`
pub type RAM_COM7 = crate::Reg<ram_com7::RAM_COM7_SPEC>;
///display memory
pub mod ram_com7;
