#[doc = "Register `PDIR` reader"]
pub struct R(crate::R<PDIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PDI` reader - Port Data Input"]
pub type PDI_R = crate::FieldReader<u32, PDI_A>;
#[doc = "Port Data Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PDI_A {
    #[doc = "0: Pin logic level is logic 0, or is not configured for use by digital function."]
    _0 = 0,
    #[doc = "1: Pin logic level is logic 1."]
    _1 = 1,
}
impl From<PDI_A> for u32 {
    #[inline(always)]
    fn from(variant: PDI_A) -> Self {
        variant as _
    }
}
impl PDI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PDI_A> {
        match self.bits {
            0 => Some(PDI_A::_0),
            1 => Some(PDI_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDI_A::_1
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Input"]
    #[inline(always)]
    pub fn pdi(&self) -> PDI_R {
        PDI_R::new(self.bits)
    }
}
#[doc = "Port Data Input Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdir](index.html) module"]
pub struct PDIR_SPEC;
impl crate::RegisterSpec for PDIR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdir::R](R) reader structure"]
impl crate::Readable for PDIR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PDIR to value 0"]
impl crate::Resettable for PDIR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
