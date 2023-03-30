#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00..0x80 - Pin Control Register n"]
    pub pcr: [PCR; 32],
    #[doc = "0x80 - Global Pin Control Low Register"]
    pub gpclr: GPCLR,
    #[doc = "0x84 - Global Pin Control High Register"]
    pub gpchr: GPCHR,
    _reserved3: [u8; 0x18],
    #[doc = "0xa0 - Interrupt Status Flag Register"]
    pub isfr: ISFR,
    _reserved4: [u8; 0x1c],
    #[doc = "0xc0 - Digital Filter Enable Register"]
    pub dfer: DFER,
    #[doc = "0xc4 - Digital Filter Clock Register"]
    pub dfcr: DFCR,
    #[doc = "0xc8 - Digital Filter Width Register"]
    pub dfwr: DFWR,
}
#[doc = "PCR (rw) register accessor: an alias for `Reg<PCR_SPEC>`"]
pub type PCR = crate::Reg<pcr::PCR_SPEC>;
#[doc = "Pin Control Register n"]
pub mod pcr;
#[doc = "GPCLR (w) register accessor: an alias for `Reg<GPCLR_SPEC>`"]
pub type GPCLR = crate::Reg<gpclr::GPCLR_SPEC>;
#[doc = "Global Pin Control Low Register"]
pub mod gpclr;
#[doc = "GPCHR (w) register accessor: an alias for `Reg<GPCHR_SPEC>`"]
pub type GPCHR = crate::Reg<gpchr::GPCHR_SPEC>;
#[doc = "Global Pin Control High Register"]
pub mod gpchr;
#[doc = "ISFR (rw) register accessor: an alias for `Reg<ISFR_SPEC>`"]
pub type ISFR = crate::Reg<isfr::ISFR_SPEC>;
#[doc = "Interrupt Status Flag Register"]
pub mod isfr;
#[doc = "DFER (rw) register accessor: an alias for `Reg<DFER_SPEC>`"]
pub type DFER = crate::Reg<dfer::DFER_SPEC>;
#[doc = "Digital Filter Enable Register"]
pub mod dfer;
#[doc = "DFCR (rw) register accessor: an alias for `Reg<DFCR_SPEC>`"]
pub type DFCR = crate::Reg<dfcr::DFCR_SPEC>;
#[doc = "Digital Filter Clock Register"]
pub mod dfcr;
#[doc = "DFWR (rw) register accessor: an alias for `Reg<DFWR_SPEC>`"]
pub type DFWR = crate::Reg<dfwr::DFWR_SPEC>;
#[doc = "Digital Filter Width Register"]
pub mod dfwr;
