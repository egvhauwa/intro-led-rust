#[doc = "Register `XACC%s` reader"]
pub struct R(crate::R<XACC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XACC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XACC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XACC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `XA` reader - Execute-only access control"]
pub type XA_R = crate::FieldReader<u8, XA_A>;
#[doc = "Execute-only access control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum XA_A {
    #[doc = "0: Associated segment is accessible in execute mode only (as an instruction fetch)"]
    _0 = 0,
    #[doc = "1: Associated segment is accessible as data or in execute mode"]
    _1 = 1,
}
impl From<XA_A> for u8 {
    #[inline(always)]
    fn from(variant: XA_A) -> Self {
        variant as _
    }
}
impl XA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<XA_A> {
        match self.bits {
            0 => Some(XA_A::_0),
            1 => Some(XA_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == XA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == XA_A::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Execute-only access control"]
    #[inline(always)]
    pub fn xa(&self) -> XA_R {
        XA_R::new(self.bits)
    }
}
#[doc = "Execute-only Access Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xacc](index.html) module"]
pub struct XACC_SPEC;
impl crate::RegisterSpec for XACC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [xacc::R](R) reader structure"]
impl crate::Readable for XACC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets XACC%s to value 0"]
impl crate::Resettable for XACC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
