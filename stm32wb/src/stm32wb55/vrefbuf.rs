///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - VREF control and status register
    pub csr: CSR,
    ///0x04 - calibration control register
    pub ccr: CCR,
}
///CSR (rw) register accessor: an alias for `Reg<CSR_SPEC>`
pub type CSR = crate::Reg<csr::CSR_SPEC>;
///VREF control and status register
pub mod csr;
///CCR (rw) register accessor: an alias for `Reg<CCR_SPEC>`
pub type CCR = crate::Reg<ccr::CCR_SPEC>;
///calibration control register
pub mod ccr;
