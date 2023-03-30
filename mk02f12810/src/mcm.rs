#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Crossbar Switch (AXBS) Slave Configuration"]
    pub plasc: PLASC,
    #[doc = "0x0a - Crossbar Switch (AXBS) Master Configuration"]
    pub plamc: PLAMC,
    #[doc = "0x0c - Crossbar Switch (AXBS) Control Register"]
    pub placr: PLACR,
    #[doc = "0x10 - Interrupt Status and Control Register"]
    pub iscr: ISCR,
    _reserved4: [u8; 0x2c],
    #[doc = "0x40 - Compute Operation Control Register"]
    pub cpo: CPO,
}
#[doc = "PLASC (r) register accessor: an alias for `Reg<PLASC_SPEC>`"]
pub type PLASC = crate::Reg<plasc::PLASC_SPEC>;
#[doc = "Crossbar Switch (AXBS) Slave Configuration"]
pub mod plasc;
#[doc = "PLAMC (r) register accessor: an alias for `Reg<PLAMC_SPEC>`"]
pub type PLAMC = crate::Reg<plamc::PLAMC_SPEC>;
#[doc = "Crossbar Switch (AXBS) Master Configuration"]
pub mod plamc;
#[doc = "PLACR (rw) register accessor: an alias for `Reg<PLACR_SPEC>`"]
pub type PLACR = crate::Reg<placr::PLACR_SPEC>;
#[doc = "Crossbar Switch (AXBS) Control Register"]
pub mod placr;
#[doc = "ISCR (rw) register accessor: an alias for `Reg<ISCR_SPEC>`"]
pub type ISCR = crate::Reg<iscr::ISCR_SPEC>;
#[doc = "Interrupt Status and Control Register"]
pub mod iscr;
#[doc = "CPO (rw) register accessor: an alias for `Reg<CPO_SPEC>`"]
pub type CPO = crate::Reg<cpo::CPO_SPEC>;
#[doc = "Compute Operation Control Register"]
pub mod cpo;
