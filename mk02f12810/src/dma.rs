#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Control Register"]
    pub cr: CR,
    #[doc = "0x04 - Error Status Register"]
    pub es: ES,
    _reserved2: [u8; 0x04],
    #[doc = "0x0c - Enable Request Register"]
    pub erq: ERQ,
    _reserved3: [u8; 0x04],
    #[doc = "0x14 - Enable Error Interrupt Register"]
    pub eei: EEI,
    #[doc = "0x18 - Clear Enable Error Interrupt Register"]
    pub ceei: CEEI,
    #[doc = "0x19 - Set Enable Error Interrupt Register"]
    pub seei: SEEI,
    #[doc = "0x1a - Clear Enable Request Register"]
    pub cerq: CERQ,
    #[doc = "0x1b - Set Enable Request Register"]
    pub serq: SERQ,
    #[doc = "0x1c - Clear DONE Status Bit Register"]
    pub cdne: CDNE,
    #[doc = "0x1d - Set START Bit Register"]
    pub ssrt: SSRT,
    #[doc = "0x1e - Clear Error Register"]
    pub cerr: CERR,
    #[doc = "0x1f - Clear Interrupt Request Register"]
    pub cint: CINT,
    _reserved12: [u8; 0x04],
    #[doc = "0x24 - Interrupt Request Register"]
    pub int: INT,
    _reserved13: [u8; 0x04],
    #[doc = "0x2c - Error Register"]
    pub err: ERR,
    _reserved14: [u8; 0x04],
    #[doc = "0x34 - Hardware Request Status Register"]
    pub hrs: HRS,
    _reserved15: [u8; 0x0c],
    #[doc = "0x44 - Enable Asynchronous Request in Stop Register"]
    pub ears: EARS,
    _reserved16: [u8; 0xb8],
    #[doc = "0x100 - Channel n Priority Register"]
    pub dchpri: [DCHPRI; 4],
    _reserved17: [u8; 0x0efc],
    #[doc = "0x1000 - TCD Source Address"]
    pub tcd0_saddr: TCD_SADDR,
    #[doc = "0x1004 - TCD Signed Source Address Offset"]
    pub tcd0_soff: TCD_SOFF,
    #[doc = "0x1006 - TCD Transfer Attributes"]
    pub tcd0_attr: TCD_ATTR,
    _reserved_20_dma_tcd0_nbytes: [u8; 0x04],
    #[doc = "0x100c - TCD Last Source Address Adjustment"]
    pub tcd0_slast: TCD_SLAST,
    #[doc = "0x1010 - TCD Destination Address"]
    pub tcd0_daddr: TCD_DADDR,
    #[doc = "0x1014 - TCD Signed Destination Address Offset"]
    pub tcd0_doff: TCD_DOFF,
    _reserved_24_dma_tcd0_citer: [u8; 0x02],
    #[doc = "0x1018 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd0_dlastsga: TCD_DLASTSGA,
    #[doc = "0x101c - TCD Control and Status"]
    pub tcd0_csr: TCD_CSR,
    _reserved_27_dma_tcd0_biter: [u8; 0x02],
    #[doc = "0x1020 - TCD Source Address"]
    pub tcd1_saddr: TCD_SADDR,
    #[doc = "0x1024 - TCD Signed Source Address Offset"]
    pub tcd1_soff: TCD_SOFF,
    #[doc = "0x1026 - TCD Transfer Attributes"]
    pub tcd1_attr: TCD_ATTR,
    _reserved_31_dma_tcd1_nbytes: [u8; 0x04],
    #[doc = "0x102c - TCD Last Source Address Adjustment"]
    pub tcd1_slast: TCD_SLAST,
    #[doc = "0x1030 - TCD Destination Address"]
    pub tcd1_daddr: TCD_DADDR,
    #[doc = "0x1034 - TCD Signed Destination Address Offset"]
    pub tcd1_doff: TCD_DOFF,
    _reserved_35_dma_tcd1_citer: [u8; 0x02],
    #[doc = "0x1038 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd1_dlastsga: TCD_DLASTSGA,
    #[doc = "0x103c - TCD Control and Status"]
    pub tcd1_csr: TCD_CSR,
    _reserved_38_dma_tcd1_biter: [u8; 0x02],
    #[doc = "0x1040 - TCD Source Address"]
    pub tcd2_saddr: TCD_SADDR,
    #[doc = "0x1044 - TCD Signed Source Address Offset"]
    pub tcd2_soff: TCD_SOFF,
    #[doc = "0x1046 - TCD Transfer Attributes"]
    pub tcd2_attr: TCD_ATTR,
    _reserved_42_dma_tcd2_nbytes: [u8; 0x04],
    #[doc = "0x104c - TCD Last Source Address Adjustment"]
    pub tcd2_slast: TCD_SLAST,
    #[doc = "0x1050 - TCD Destination Address"]
    pub tcd2_daddr: TCD_DADDR,
    #[doc = "0x1054 - TCD Signed Destination Address Offset"]
    pub tcd2_doff: TCD_DOFF,
    _reserved_46_dma_tcd2_citer: [u8; 0x02],
    #[doc = "0x1058 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd2_dlastsga: TCD_DLASTSGA,
    #[doc = "0x105c - TCD Control and Status"]
    pub tcd2_csr: TCD_CSR,
    _reserved_49_dma_tcd2_biter: [u8; 0x02],
    #[doc = "0x1060 - TCD Source Address"]
    pub tcd3_saddr: TCD_SADDR,
    #[doc = "0x1064 - TCD Signed Source Address Offset"]
    pub tcd3_soff: TCD_SOFF,
    #[doc = "0x1066 - TCD Transfer Attributes"]
    pub tcd3_attr: TCD_ATTR,
    _reserved_53_dma_tcd3_nbytes: [u8; 0x04],
    #[doc = "0x106c - TCD Last Source Address Adjustment"]
    pub tcd3_slast: TCD_SLAST,
    #[doc = "0x1070 - TCD Destination Address"]
    pub tcd3_daddr: TCD_DADDR,
    #[doc = "0x1074 - TCD Signed Destination Address Offset"]
    pub tcd3_doff: TCD_DOFF,
    _reserved_57_dma_tcd3_citer: [u8; 0x02],
    #[doc = "0x1078 - TCD Last Destination Address Adjustment/Scatter Gather Address"]
    pub tcd3_dlastsga: TCD_DLASTSGA,
    #[doc = "0x107c - TCD Control and Status"]
    pub tcd3_csr: TCD_CSR,
    _reserved_60_dma_tcd3_biter: [u8; 0x02],
}
impl RegisterBlock {
    #[doc = "0x100 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri3(&self) -> &DCHPRI {
        &self.dchpri[0]
    }
    #[doc = "0x101 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri2(&self) -> &DCHPRI {
        &self.dchpri[1]
    }
    #[doc = "0x102 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri1(&self) -> &DCHPRI {
        &self.dchpri[2]
    }
    #[doc = "0x103 - Channel n Priority Register"]
    #[inline(always)]
    pub fn dchpri0(&self) -> &DCHPRI {
        &self.dchpri[3]
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4104usize).cast() }
    }
    #[doc = "0x1008 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4104usize).cast() }
    }
    #[doc = "0x1008 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4104usize).cast() }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4118usize).cast() }
    }
    #[doc = "0x1016 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4118usize).cast() }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4126usize).cast() }
    }
    #[doc = "0x101e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd0_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4126usize).cast() }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4136usize).cast() }
    }
    #[doc = "0x1028 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4136usize).cast() }
    }
    #[doc = "0x1028 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4136usize).cast() }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4150usize).cast() }
    }
    #[doc = "0x1036 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4150usize).cast() }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4158usize).cast() }
    }
    #[doc = "0x103e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd1_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4158usize).cast() }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4168usize).cast() }
    }
    #[doc = "0x1048 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4168usize).cast() }
    }
    #[doc = "0x1048 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4168usize).cast() }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4182usize).cast() }
    }
    #[doc = "0x1056 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4182usize).cast() }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4190usize).cast() }
    }
    #[doc = "0x105e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd2_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4190usize).cast() }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_nbytes_mloffyes(&self) -> &DMA_TCD_NBYTES_MLOFFYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4200usize).cast() }
    }
    #[doc = "0x1068 - TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_nbytes_mloffno(&self) -> &DMA_TCD_NBYTES_MLOFFNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4200usize).cast() }
    }
    #[doc = "0x1068 - TCD Minor Byte Count (Minor Loop Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_nbytes_mlno(&self) -> &DMA_TCD_NBYTES_MLNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4200usize).cast() }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_citer_elinkyes(&self) -> &DMA_TCD_CITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4214usize).cast() }
    }
    #[doc = "0x1076 - TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_citer_elinkno(&self) -> &DMA_TCD_CITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4214usize).cast() }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_biter_elinkyes(&self) -> &DMA_TCD_BITER_ELINKYES {
        unsafe { &*(self as *const Self).cast::<u8>().add(4222usize).cast() }
    }
    #[doc = "0x107e - TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
    #[inline(always)]
    pub const fn dma_tcd3_biter_elinkno(&self) -> &DMA_TCD_BITER_ELINKNO {
        unsafe { &*(self as *const Self).cast::<u8>().add(4222usize).cast() }
    }
}
#[doc = "CR (rw) register accessor: an alias for `Reg<CR_SPEC>`"]
pub type CR = crate::Reg<cr::CR_SPEC>;
#[doc = "Control Register"]
pub mod cr;
#[doc = "ES (r) register accessor: an alias for `Reg<ES_SPEC>`"]
pub type ES = crate::Reg<es::ES_SPEC>;
#[doc = "Error Status Register"]
pub mod es;
#[doc = "ERQ (rw) register accessor: an alias for `Reg<ERQ_SPEC>`"]
pub type ERQ = crate::Reg<erq::ERQ_SPEC>;
#[doc = "Enable Request Register"]
pub mod erq;
#[doc = "EEI (rw) register accessor: an alias for `Reg<EEI_SPEC>`"]
pub type EEI = crate::Reg<eei::EEI_SPEC>;
#[doc = "Enable Error Interrupt Register"]
pub mod eei;
#[doc = "CEEI (w) register accessor: an alias for `Reg<CEEI_SPEC>`"]
pub type CEEI = crate::Reg<ceei::CEEI_SPEC>;
#[doc = "Clear Enable Error Interrupt Register"]
pub mod ceei;
#[doc = "SEEI (w) register accessor: an alias for `Reg<SEEI_SPEC>`"]
pub type SEEI = crate::Reg<seei::SEEI_SPEC>;
#[doc = "Set Enable Error Interrupt Register"]
pub mod seei;
#[doc = "CERQ (w) register accessor: an alias for `Reg<CERQ_SPEC>`"]
pub type CERQ = crate::Reg<cerq::CERQ_SPEC>;
#[doc = "Clear Enable Request Register"]
pub mod cerq;
#[doc = "SERQ (w) register accessor: an alias for `Reg<SERQ_SPEC>`"]
pub type SERQ = crate::Reg<serq::SERQ_SPEC>;
#[doc = "Set Enable Request Register"]
pub mod serq;
#[doc = "CDNE (w) register accessor: an alias for `Reg<CDNE_SPEC>`"]
pub type CDNE = crate::Reg<cdne::CDNE_SPEC>;
#[doc = "Clear DONE Status Bit Register"]
pub mod cdne;
#[doc = "SSRT (w) register accessor: an alias for `Reg<SSRT_SPEC>`"]
pub type SSRT = crate::Reg<ssrt::SSRT_SPEC>;
#[doc = "Set START Bit Register"]
pub mod ssrt;
#[doc = "CERR (w) register accessor: an alias for `Reg<CERR_SPEC>`"]
pub type CERR = crate::Reg<cerr::CERR_SPEC>;
#[doc = "Clear Error Register"]
pub mod cerr;
#[doc = "CINT (w) register accessor: an alias for `Reg<CINT_SPEC>`"]
pub type CINT = crate::Reg<cint::CINT_SPEC>;
#[doc = "Clear Interrupt Request Register"]
pub mod cint;
#[doc = "INT (rw) register accessor: an alias for `Reg<INT_SPEC>`"]
pub type INT = crate::Reg<int::INT_SPEC>;
#[doc = "Interrupt Request Register"]
pub mod int;
#[doc = "ERR (rw) register accessor: an alias for `Reg<ERR_SPEC>`"]
pub type ERR = crate::Reg<err::ERR_SPEC>;
#[doc = "Error Register"]
pub mod err;
#[doc = "HRS (r) register accessor: an alias for `Reg<HRS_SPEC>`"]
pub type HRS = crate::Reg<hrs::HRS_SPEC>;
#[doc = "Hardware Request Status Register"]
pub mod hrs;
#[doc = "EARS (rw) register accessor: an alias for `Reg<EARS_SPEC>`"]
pub type EARS = crate::Reg<ears::EARS_SPEC>;
#[doc = "Enable Asynchronous Request in Stop Register"]
pub mod ears;
#[doc = "DCHPRI (rw) register accessor: an alias for `Reg<DCHPRI_SPEC>`"]
pub type DCHPRI = crate::Reg<dchpri::DCHPRI_SPEC>;
#[doc = "Channel n Priority Register"]
pub mod dchpri;
#[doc = "TCD_SADDR (rw) register accessor: an alias for `Reg<TCD_SADDR_SPEC>`"]
pub type TCD_SADDR = crate::Reg<tcd_saddr::TCD_SADDR_SPEC>;
#[doc = "TCD Source Address"]
pub mod tcd_saddr;
#[doc = "TCD_SOFF (rw) register accessor: an alias for `Reg<TCD_SOFF_SPEC>`"]
pub type TCD_SOFF = crate::Reg<tcd_soff::TCD_SOFF_SPEC>;
#[doc = "TCD Signed Source Address Offset"]
pub mod tcd_soff;
#[doc = "TCD_ATTR (rw) register accessor: an alias for `Reg<TCD_ATTR_SPEC>`"]
pub type TCD_ATTR = crate::Reg<tcd_attr::TCD_ATTR_SPEC>;
#[doc = "TCD Transfer Attributes"]
pub mod tcd_attr;
#[doc = "DMA_TCD_NBYTES_MLNO (rw) register accessor: an alias for `Reg<DMA_TCD_NBYTES_MLNO_SPEC>`"]
pub type DMA_TCD_NBYTES_MLNO = crate::Reg<dma_tcd_nbytes_mlno::DMA_TCD_NBYTES_MLNO_SPEC>;
#[doc = "TCD Minor Byte Count (Minor Loop Disabled)"]
pub mod dma_tcd_nbytes_mlno;
#[doc = "DMA_TCD_NBYTES_MLOFFNO (rw) register accessor: an alias for `Reg<DMA_TCD_NBYTES_MLOFFNO_SPEC>`"]
pub type DMA_TCD_NBYTES_MLOFFNO = crate::Reg<dma_tcd_nbytes_mloffno::DMA_TCD_NBYTES_MLOFFNO_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop Enabled and Offset Disabled)"]
pub mod dma_tcd_nbytes_mloffno;
#[doc = "DMA_TCD_NBYTES_MLOFFYES (rw) register accessor: an alias for `Reg<DMA_TCD_NBYTES_MLOFFYES_SPEC>`"]
pub type DMA_TCD_NBYTES_MLOFFYES =
    crate::Reg<dma_tcd_nbytes_mloffyes::DMA_TCD_NBYTES_MLOFFYES_SPEC>;
#[doc = "TCD Signed Minor Loop Offset (Minor Loop and Offset Enabled)"]
pub mod dma_tcd_nbytes_mloffyes;
#[doc = "TCD_SLAST (rw) register accessor: an alias for `Reg<TCD_SLAST_SPEC>`"]
pub type TCD_SLAST = crate::Reg<tcd_slast::TCD_SLAST_SPEC>;
#[doc = "TCD Last Source Address Adjustment"]
pub mod tcd_slast;
#[doc = "TCD_DADDR (rw) register accessor: an alias for `Reg<TCD_DADDR_SPEC>`"]
pub type TCD_DADDR = crate::Reg<tcd_daddr::TCD_DADDR_SPEC>;
#[doc = "TCD Destination Address"]
pub mod tcd_daddr;
#[doc = "TCD_DOFF (rw) register accessor: an alias for `Reg<TCD_DOFF_SPEC>`"]
pub type TCD_DOFF = crate::Reg<tcd_doff::TCD_DOFF_SPEC>;
#[doc = "TCD Signed Destination Address Offset"]
pub mod tcd_doff;
#[doc = "DMA_TCD_CITER_ELINKNO (rw) register accessor: an alias for `Reg<DMA_TCD_CITER_ELINKNO_SPEC>`"]
pub type DMA_TCD_CITER_ELINKNO = crate::Reg<dma_tcd_citer_elinkno::DMA_TCD_CITER_ELINKNO_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd_citer_elinkno;
#[doc = "DMA_TCD_CITER_ELINKYES (rw) register accessor: an alias for `Reg<DMA_TCD_CITER_ELINKYES_SPEC>`"]
pub type DMA_TCD_CITER_ELINKYES = crate::Reg<dma_tcd_citer_elinkyes::DMA_TCD_CITER_ELINKYES_SPEC>;
#[doc = "TCD Current Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd_citer_elinkyes;
#[doc = "TCD_DLASTSGA (rw) register accessor: an alias for `Reg<TCD_DLASTSGA_SPEC>`"]
pub type TCD_DLASTSGA = crate::Reg<tcd_dlastsga::TCD_DLASTSGA_SPEC>;
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address"]
pub mod tcd_dlastsga;
#[doc = "TCD_CSR (rw) register accessor: an alias for `Reg<TCD_CSR_SPEC>`"]
pub type TCD_CSR = crate::Reg<tcd_csr::TCD_CSR_SPEC>;
#[doc = "TCD Control and Status"]
pub mod tcd_csr;
#[doc = "DMA_TCD_BITER_ELINKNO (rw) register accessor: an alias for `Reg<DMA_TCD_BITER_ELINKNO_SPEC>`"]
pub type DMA_TCD_BITER_ELINKNO = crate::Reg<dma_tcd_biter_elinkno::DMA_TCD_BITER_ELINKNO_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Disabled)"]
pub mod dma_tcd_biter_elinkno;
#[doc = "DMA_TCD_BITER_ELINKYES (rw) register accessor: an alias for `Reg<DMA_TCD_BITER_ELINKYES_SPEC>`"]
pub type DMA_TCD_BITER_ELINKYES = crate::Reg<dma_tcd_biter_elinkyes::DMA_TCD_BITER_ELINKYES_SPEC>;
#[doc = "TCD Beginning Minor Loop Link, Major Loop Count (Channel Linking Enabled)"]
pub mod dma_tcd_biter_elinkyes;
