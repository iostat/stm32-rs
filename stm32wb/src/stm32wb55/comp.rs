///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Comparator control and status register
    pub comp1_csr: COMP1_CSR,
    ///0x04 - Comparator 2 control and status register
    pub comp2_csr: COMP2_CSR,
}
///COMP1_CSR (rw) register accessor: an alias for `Reg<COMP1_CSR_SPEC>`
pub type COMP1_CSR = crate::Reg<comp1_csr::COMP1_CSR_SPEC>;
///Comparator control and status register
pub mod comp1_csr;
///COMP2_CSR (rw) register accessor: an alias for `Reg<COMP2_CSR_SPEC>`
pub type COMP2_CSR = crate::Reg<comp2_csr::COMP2_CSR_SPEC>;
///Comparator 2 control and status register
pub mod comp2_csr;
