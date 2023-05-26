///Register block
#[repr(C)]
pub struct RegisterBlock {
    ///0x00 - Auxiliary control register
    pub actrl: ACTRL,
}
///ACTRL (rw) register accessor: an alias for `Reg<ACTRL_SPEC>`
pub type ACTRL = crate::Reg<actrl::ACTRL_SPEC>;
///Auxiliary control register
pub mod actrl;
