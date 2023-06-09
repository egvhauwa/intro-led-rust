#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - System Reset Status Register 0"]
    pub srs0: SRS0,
    #[doc = "0x01 - System Reset Status Register 1"]
    pub srs1: SRS1,
    _reserved2: [u8; 0x02],
    #[doc = "0x04 - Reset Pin Filter Control register"]
    pub rpfc: RPFC,
    #[doc = "0x05 - Reset Pin Filter Width register"]
    pub rpfw: RPFW,
    _reserved4: [u8; 0x02],
    #[doc = "0x08 - Sticky System Reset Status Register 0"]
    pub ssrs0: SSRS0,
    #[doc = "0x09 - Sticky System Reset Status Register 1"]
    pub ssrs1: SSRS1,
}
#[doc = "SRS0 (r) register accessor: an alias for `Reg<SRS0_SPEC>`"]
pub type SRS0 = crate::Reg<srs0::SRS0_SPEC>;
#[doc = "System Reset Status Register 0"]
pub mod srs0;
#[doc = "SRS1 (r) register accessor: an alias for `Reg<SRS1_SPEC>`"]
pub type SRS1 = crate::Reg<srs1::SRS1_SPEC>;
#[doc = "System Reset Status Register 1"]
pub mod srs1;
#[doc = "RPFC (rw) register accessor: an alias for `Reg<RPFC_SPEC>`"]
pub type RPFC = crate::Reg<rpfc::RPFC_SPEC>;
#[doc = "Reset Pin Filter Control register"]
pub mod rpfc;
#[doc = "RPFW (rw) register accessor: an alias for `Reg<RPFW_SPEC>`"]
pub type RPFW = crate::Reg<rpfw::RPFW_SPEC>;
#[doc = "Reset Pin Filter Width register"]
pub mod rpfw;
#[doc = "SSRS0 (rw) register accessor: an alias for `Reg<SSRS0_SPEC>`"]
pub type SSRS0 = crate::Reg<ssrs0::SSRS0_SPEC>;
#[doc = "Sticky System Reset Status Register 0"]
pub mod ssrs0;
#[doc = "SSRS1 (rw) register accessor: an alias for `Reg<SSRS1_SPEC>`"]
pub type SSRS1 = crate::Reg<ssrs1::SSRS1_SPEC>;
#[doc = "Sticky System Reset Status Register 1"]
pub mod ssrs1;
