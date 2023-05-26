///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Coprocessor access control register
    pub cpacr: CPACR,
}
///CPACR (rw) register accessor: an alias for `Reg<CPACR_SPEC>`
pub type CPACR = crate::Reg<cpacr::CPACR_SPEC>;
///Coprocessor access control register
pub mod cpacr;
