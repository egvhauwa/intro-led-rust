#[doc = "Register `ES` reader"]
pub struct R(crate::R<ES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DBE` reader - Destination Bus Error"]
pub type DBE_R = crate::BitReader<DBE_A>;
#[doc = "Destination Bus Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBE_A {
    #[doc = "0: No destination bus error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a bus error on a destination write"]
    _1 = 1,
}
impl From<DBE_A> for bool {
    #[inline(always)]
    fn from(variant: DBE_A) -> Self {
        variant as u8 != 0
    }
}
impl DBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBE_A {
        match self.bits {
            false => DBE_A::_0,
            true => DBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DBE_A::_1
    }
}
#[doc = "Field `SBE` reader - Source Bus Error"]
pub type SBE_R = crate::BitReader<SBE_A>;
#[doc = "Source Bus Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBE_A {
    #[doc = "0: No source bus error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a bus error on a source read"]
    _1 = 1,
}
impl From<SBE_A> for bool {
    #[inline(always)]
    fn from(variant: SBE_A) -> Self {
        variant as u8 != 0
    }
}
impl SBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBE_A {
        match self.bits {
            false => SBE_A::_0,
            true => SBE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SBE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SBE_A::_1
    }
}
#[doc = "Field `SGE` reader - Scatter/Gather Configuration Error"]
pub type SGE_R = crate::BitReader<SGE_A>;
#[doc = "Scatter/Gather Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SGE_A {
    #[doc = "0: No scatter/gather configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DLASTSGA field. This field is checked at the beginning of a scatter/gather operation after major loop completion if TCDn_CSR\\[ESG\\]
is enabled. TCDn_DLASTSGA is not on a 32 byte boundary."]
    _1 = 1,
}
impl From<SGE_A> for bool {
    #[inline(always)]
    fn from(variant: SGE_A) -> Self {
        variant as u8 != 0
    }
}
impl SGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SGE_A {
        match self.bits {
            false => SGE_A::_0,
            true => SGE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SGE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SGE_A::_1
    }
}
#[doc = "Field `NCE` reader - NBYTES/CITER Configuration Error"]
pub type NCE_R = crate::BitReader<NCE_A>;
#[doc = "NBYTES/CITER Configuration Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NCE_A {
    #[doc = "0: No NBYTES/CITER configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_NBYTES or TCDn_CITER fields. TCDn_NBYTES is not a multiple of TCDn_ATTR\\[SSIZE\\]
and TCDn_ATTR\\[DSIZE\\], or TCDn_CITER\\[CITER\\]
is equal to zero, or TCDn_CITER\\[ELINK\\]
is not equal to TCDn_BITER\\[ELINK\\]"]
    _1 = 1,
}
impl From<NCE_A> for bool {
    #[inline(always)]
    fn from(variant: NCE_A) -> Self {
        variant as u8 != 0
    }
}
impl NCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NCE_A {
        match self.bits {
            false => NCE_A::_0,
            true => NCE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == NCE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == NCE_A::_1
    }
}
#[doc = "Field `DOE` reader - Destination Offset Error"]
pub type DOE_R = crate::BitReader<DOE_A>;
#[doc = "Destination Offset Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOE_A {
    #[doc = "0: No destination offset configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DOFF field. TCDn_DOFF is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
    _1 = 1,
}
impl From<DOE_A> for bool {
    #[inline(always)]
    fn from(variant: DOE_A) -> Self {
        variant as u8 != 0
    }
}
impl DOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOE_A {
        match self.bits {
            false => DOE_A::_0,
            true => DOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOE_A::_1
    }
}
#[doc = "Field `DAE` reader - Destination Address Error"]
pub type DAE_R = crate::BitReader<DAE_A>;
#[doc = "Destination Address Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAE_A {
    #[doc = "0: No destination address configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_DADDR field. TCDn_DADDR is inconsistent with TCDn_ATTR\\[DSIZE\\]."]
    _1 = 1,
}
impl From<DAE_A> for bool {
    #[inline(always)]
    fn from(variant: DAE_A) -> Self {
        variant as u8 != 0
    }
}
impl DAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAE_A {
        match self.bits {
            false => DAE_A::_0,
            true => DAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAE_A::_1
    }
}
#[doc = "Field `SOE` reader - Source Offset Error"]
pub type SOE_R = crate::BitReader<SOE_A>;
#[doc = "Source Offset Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOE_A {
    #[doc = "0: No source offset configuration error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_SOFF field. TCDn_SOFF is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
    _1 = 1,
}
impl From<SOE_A> for bool {
    #[inline(always)]
    fn from(variant: SOE_A) -> Self {
        variant as u8 != 0
    }
}
impl SOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOE_A {
        match self.bits {
            false => SOE_A::_0,
            true => SOE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SOE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SOE_A::_1
    }
}
#[doc = "Field `SAE` reader - Source Address Error"]
pub type SAE_R = crate::BitReader<SAE_A>;
#[doc = "Source Address Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAE_A {
    #[doc = "0: No source address configuration error."]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error detected in the TCDn_SADDR field. TCDn_SADDR is inconsistent with TCDn_ATTR\\[SSIZE\\]."]
    _1 = 1,
}
impl From<SAE_A> for bool {
    #[inline(always)]
    fn from(variant: SAE_A) -> Self {
        variant as u8 != 0
    }
}
impl SAE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAE_A {
        match self.bits {
            false => SAE_A::_0,
            true => SAE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SAE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SAE_A::_1
    }
}
#[doc = "Field `ERRCHN` reader - Error Channel Number or Canceled Channel Number"]
pub type ERRCHN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CPE` reader - Channel Priority Error"]
pub type CPE_R = crate::BitReader<CPE_A>;
#[doc = "Channel Priority Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPE_A {
    #[doc = "0: No channel priority error"]
    _0 = 0,
    #[doc = "1: The last recorded error was a configuration error in the channel priorities . Channel priorities are not unique."]
    _1 = 1,
}
impl From<CPE_A> for bool {
    #[inline(always)]
    fn from(variant: CPE_A) -> Self {
        variant as u8 != 0
    }
}
impl CPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPE_A {
        match self.bits {
            false => CPE_A::_0,
            true => CPE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPE_A::_1
    }
}
#[doc = "Field `ECX` reader - Transfer Canceled"]
pub type ECX_R = crate::BitReader<ECX_A>;
#[doc = "Transfer Canceled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECX_A {
    #[doc = "0: No canceled transfers"]
    _0 = 0,
    #[doc = "1: The last recorded entry was a canceled transfer by the error cancel transfer input"]
    _1 = 1,
}
impl From<ECX_A> for bool {
    #[inline(always)]
    fn from(variant: ECX_A) -> Self {
        variant as u8 != 0
    }
}
impl ECX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ECX_A {
        match self.bits {
            false => ECX_A::_0,
            true => ECX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ECX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ECX_A::_1
    }
}
#[doc = "Field `VLD` reader - Logical OR of all ERR status bits"]
pub type VLD_R = crate::BitReader<VLD_A>;
#[doc = "Logical OR of all ERR status bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLD_A {
    #[doc = "0: No ERR bits are set"]
    _0 = 0,
    #[doc = "1: At least one ERR bit is set indicating a valid error exists that has not been cleared"]
    _1 = 1,
}
impl From<VLD_A> for bool {
    #[inline(always)]
    fn from(variant: VLD_A) -> Self {
        variant as u8 != 0
    }
}
impl VLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLD_A {
        match self.bits {
            false => VLD_A::_0,
            true => VLD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == VLD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == VLD_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Destination Bus Error"]
    #[inline(always)]
    pub fn dbe(&self) -> DBE_R {
        DBE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Source Bus Error"]
    #[inline(always)]
    pub fn sbe(&self) -> SBE_R {
        SBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Scatter/Gather Configuration Error"]
    #[inline(always)]
    pub fn sge(&self) -> SGE_R {
        SGE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NBYTES/CITER Configuration Error"]
    #[inline(always)]
    pub fn nce(&self) -> NCE_R {
        NCE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Destination Offset Error"]
    #[inline(always)]
    pub fn doe(&self) -> DOE_R {
        DOE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Destination Address Error"]
    #[inline(always)]
    pub fn dae(&self) -> DAE_R {
        DAE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Source Offset Error"]
    #[inline(always)]
    pub fn soe(&self) -> SOE_R {
        SOE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Source Address Error"]
    #[inline(always)]
    pub fn sae(&self) -> SAE_R {
        SAE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Error Channel Number or Canceled Channel Number"]
    #[inline(always)]
    pub fn errchn(&self) -> ERRCHN_R {
        ERRCHN_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 14 - Channel Priority Error"]
    #[inline(always)]
    pub fn cpe(&self) -> CPE_R {
        CPE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer Canceled"]
    #[inline(always)]
    pub fn ecx(&self) -> ECX_R {
        ECX_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - Logical OR of all ERR status bits"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Error Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [es](index.html) module"]
pub struct ES_SPEC;
impl crate::RegisterSpec for ES_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [es::R](R) reader structure"]
impl crate::Readable for ES_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ES to value 0"]
impl crate::Resettable for ES_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
