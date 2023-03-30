#[doc = "Register `SACC%s` reader"]
pub struct R(crate::R<SACC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SACC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SACC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SACC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SA` reader - Supervisor-only access control"]
pub type SA_R = crate::FieldReader<u8, SA_A>;
#[doc = "Supervisor-only access control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SA_A {
    #[doc = "0: Associated segment is accessible in supervisor mode only"]
    _0 = 0,
    #[doc = "1: Associated segment is accessible in user or supervisor mode"]
    _1 = 1,
}
impl From<SA_A> for u8 {
    #[inline(always)]
    fn from(variant: SA_A) -> Self {
        variant as _
    }
}
impl SA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SA_A> {
        match self.bits {
            0 => Some(SA_A::_0),
            1 => Some(SA_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SA_A::_1
    }
}
impl R {
    #[doc = "Bits 0:7 - Supervisor-only access control"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits)
    }
}
#[doc = "Supervisor-only Access Registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sacc](index.html) module"]
pub struct SACC_SPEC;
impl crate::RegisterSpec for SACC_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [sacc::R](R) reader structure"]
impl crate::Readable for SACC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SACC%s to value 0"]
impl crate::Resettable for SACC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
