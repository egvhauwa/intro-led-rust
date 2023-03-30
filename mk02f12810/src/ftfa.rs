#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Flash Status Register"]
    pub fstat: FSTAT,
    #[doc = "0x01 - Flash Configuration Register"]
    pub fcnfg: FCNFG,
    #[doc = "0x02 - Flash Security Register"]
    pub fsec: FSEC,
    #[doc = "0x03 - Flash Option Register"]
    pub fopt: FOPT,
    #[doc = "0x04..0x10 - Flash Common Command Object Registers"]
    pub fccob: [FCCOB; 12],
    #[doc = "0x10 - Program Flash Protection Registers"]
    pub fprot: [FPROT; 4],
    _reserved6: [u8; 0x04],
    #[doc = "0x18..0x20 - Execute-only Access Registers"]
    pub xacc: [XACC; 8],
    #[doc = "0x20..0x28 - Supervisor-only Access Registers"]
    pub sacc: [SACC; 8],
    #[doc = "0x28 - Flash Access Segment Size Register"]
    pub facss: FACSS,
    _reserved9: [u8; 0x02],
    #[doc = "0x2b - Flash Access Segment Number Register"]
    pub facsn: FACSN,
}
impl RegisterBlock {
    #[doc = "0x04 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob3(&self) -> &FCCOB {
        &self.fccob[0]
    }
    #[doc = "0x05 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob2(&self) -> &FCCOB {
        &self.fccob[1]
    }
    #[doc = "0x06 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob1(&self) -> &FCCOB {
        &self.fccob[2]
    }
    #[doc = "0x07 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob0(&self) -> &FCCOB {
        &self.fccob[3]
    }
    #[doc = "0x08 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob7(&self) -> &FCCOB {
        &self.fccob[4]
    }
    #[doc = "0x09 - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob6(&self) -> &FCCOB {
        &self.fccob[5]
    }
    #[doc = "0x0a - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob5(&self) -> &FCCOB {
        &self.fccob[6]
    }
    #[doc = "0x0b - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob4(&self) -> &FCCOB {
        &self.fccob[7]
    }
    #[doc = "0x0c - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccobb(&self) -> &FCCOB {
        &self.fccob[8]
    }
    #[doc = "0x0d - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccoba(&self) -> &FCCOB {
        &self.fccob[9]
    }
    #[doc = "0x0e - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob9(&self) -> &FCCOB {
        &self.fccob[10]
    }
    #[doc = "0x0f - Flash Common Command Object Registers"]
    #[inline(always)]
    pub fn fccob8(&self) -> &FCCOB {
        &self.fccob[11]
    }
    #[doc = "0x10 - Program Flash Protection Registers"]
    #[inline(always)]
    pub fn fprot3(&self) -> &FPROT {
        &self.fprot[0]
    }
    #[doc = "0x11 - Program Flash Protection Registers"]
    #[inline(always)]
    pub fn fprot2(&self) -> &FPROT {
        &self.fprot[1]
    }
    #[doc = "0x12 - Program Flash Protection Registers"]
    #[inline(always)]
    pub fn fprot1(&self) -> &FPROT {
        &self.fprot[2]
    }
    #[doc = "0x13 - Program Flash Protection Registers"]
    #[inline(always)]
    pub fn fprot0(&self) -> &FPROT {
        &self.fprot[3]
    }
    #[doc = "0x18 - Execute-only Access Registers"]
    #[inline(always)]
    pub fn xacch3(&self) -> &XACC {
        &self.xacc[0]
    }
    #[doc = "0x19 - Execute-only Access Registers"]
    #[inline(always)]
    pub fn xacch2(&self) -> &XACC {
        &self.xacc[1]
    }
    #[doc = "0x1a - Execute-only Access Registers"]
    #[inline(always)]
    pub fn xacch1(&self) -> &XACC {
        &self.xacc[2]
    }
    #[doc = "0x1b - Execute-only Access Registers"]
    #[inline(always)]
    pub fn xacch0(&self) -> &XACC {
        &self.xacc[3]
    }
    #[doc = "0x1c - Execute-only Access Registers"]
    #[inline(always)]
    pub fn xaccl3(&self) -> &XACC {
        &self.xacc[4]
    }
    #[doc = "0x1d - Execute-only Access Registers"]
    #[inline(always)]
    pub fn xaccl2(&self) -> &XACC {
        &self.xacc[5]
    }
    #[doc = "0x1e - Execute-only Access Registers"]
    #[inline(always)]
    pub fn xaccl1(&self) -> &XACC {
        &self.xacc[6]
    }
    #[doc = "0x1f - Execute-only Access Registers"]
    #[inline(always)]
    pub fn xaccl0(&self) -> &XACC {
        &self.xacc[7]
    }
    #[doc = "0x20 - Supervisor-only Access Registers"]
    #[inline(always)]
    pub fn sacch3(&self) -> &SACC {
        &self.sacc[0]
    }
    #[doc = "0x21 - Supervisor-only Access Registers"]
    #[inline(always)]
    pub fn sacch2(&self) -> &SACC {
        &self.sacc[1]
    }
    #[doc = "0x22 - Supervisor-only Access Registers"]
    #[inline(always)]
    pub fn sacch1(&self) -> &SACC {
        &self.sacc[2]
    }
    #[doc = "0x23 - Supervisor-only Access Registers"]
    #[inline(always)]
    pub fn sacch0(&self) -> &SACC {
        &self.sacc[3]
    }
    #[doc = "0x24 - Supervisor-only Access Registers"]
    #[inline(always)]
    pub fn saccl3(&self) -> &SACC {
        &self.sacc[4]
    }
    #[doc = "0x25 - Supervisor-only Access Registers"]
    #[inline(always)]
    pub fn saccl2(&self) -> &SACC {
        &self.sacc[5]
    }
    #[doc = "0x26 - Supervisor-only Access Registers"]
    #[inline(always)]
    pub fn saccl1(&self) -> &SACC {
        &self.sacc[6]
    }
    #[doc = "0x27 - Supervisor-only Access Registers"]
    #[inline(always)]
    pub fn saccl0(&self) -> &SACC {
        &self.sacc[7]
    }
}
#[doc = "FSTAT (rw) register accessor: an alias for `Reg<FSTAT_SPEC>`"]
pub type FSTAT = crate::Reg<fstat::FSTAT_SPEC>;
#[doc = "Flash Status Register"]
pub mod fstat;
#[doc = "FCNFG (rw) register accessor: an alias for `Reg<FCNFG_SPEC>`"]
pub type FCNFG = crate::Reg<fcnfg::FCNFG_SPEC>;
#[doc = "Flash Configuration Register"]
pub mod fcnfg;
#[doc = "FSEC (r) register accessor: an alias for `Reg<FSEC_SPEC>`"]
pub type FSEC = crate::Reg<fsec::FSEC_SPEC>;
#[doc = "Flash Security Register"]
pub mod fsec;
#[doc = "FOPT (r) register accessor: an alias for `Reg<FOPT_SPEC>`"]
pub type FOPT = crate::Reg<fopt::FOPT_SPEC>;
#[doc = "Flash Option Register"]
pub mod fopt;
#[doc = "FCCOB (rw) register accessor: an alias for `Reg<FCCOB_SPEC>`"]
pub type FCCOB = crate::Reg<fccob::FCCOB_SPEC>;
#[doc = "Flash Common Command Object Registers"]
pub mod fccob;
#[doc = "FPROT (rw) register accessor: an alias for `Reg<FPROT_SPEC>`"]
pub type FPROT = crate::Reg<fprot::FPROT_SPEC>;
#[doc = "Program Flash Protection Registers"]
pub mod fprot;
#[doc = "XACC (r) register accessor: an alias for `Reg<XACC_SPEC>`"]
pub type XACC = crate::Reg<xacc::XACC_SPEC>;
#[doc = "Execute-only Access Registers"]
pub mod xacc;
#[doc = "SACC (r) register accessor: an alias for `Reg<SACC_SPEC>`"]
pub type SACC = crate::Reg<sacc::SACC_SPEC>;
#[doc = "Supervisor-only Access Registers"]
pub mod sacc;
#[doc = "FACSS (r) register accessor: an alias for `Reg<FACSS_SPEC>`"]
pub type FACSS = crate::Reg<facss::FACSS_SPEC>;
#[doc = "Flash Access Segment Size Register"]
pub mod facss;
#[doc = "FACSN (r) register accessor: an alias for `Reg<FACSN_SPEC>`"]
pub type FACSN = crate::Reg<facsn::FACSN_SPEC>;
#[doc = "Flash Access Segment Number Register"]
pub mod facsn;
