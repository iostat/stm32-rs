///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - ADC common status register
    pub csr: CSR,
    _reserved1: [u8; 0x04],
    ///0x08 - ADC common control register
    pub ccr: CCR,
}
///CSR (r) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///ADC common status register
pub mod csr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///ADC common control register
pub mod ccr;
