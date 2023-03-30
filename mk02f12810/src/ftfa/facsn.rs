#[doc = "Register `FACSN` reader"]
pub struct R(crate::R<FACSN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FACSN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FACSN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FACSN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `NUMSG` reader - Number of Segments Indicator"]
pub type NUMSG_R = crate::FieldReader<u8, NUMSG_A>;
#[doc = "Number of Segments Indicator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum NUMSG_A {
    #[doc = "32: Program flash memory is divided into 32 segments (64 Kbytes, 128 Kbytes)"]
    _100000 = 32,
    #[doc = "40: Program flash memory is divided into 40 segments (160 Kbytes)"]
    _101000 = 40,
    #[doc = "64: Program flash memory is divided into 64 segments (256 Kbytes, 512 Kbytes)"]
    _1000000 = 64,
}
impl From<NUMSG_A> for u8 {
    #[inline(always)]
    fn from(variant: NUMSG_A) -> Self {
        variant as _
    }
}
impl NUMSG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NUMSG_A> {
        match self.bits {
            32 => Some(NUMSG_A::_100000),
            40 => Some(NUMSG_A::_101000),
            64 => Some(NUMSG_A::_1000000),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_100000`"]
    #[inline(always)]
    pub fn is_100000(&self) -> bool {
        *self == NUMSG_A::_100000
    }
    #[doc = "Checks if the value of the field is `_101000`"]
    #[inline(always)]
    pub fn is_101000(&self) -> bool {
        *self == NUMSG_A::_101000
    }
    #[doc = "Checks if the value of the field is `_1000000`"]
    #[inline(always)]
    pub fn is_1000000(&self) -> bool {
        *self == NUMSG_A::_1000000
    }
}
impl R {
    #[doc = "Bits 0:7 - Number of Segments Indicator"]
    #[inline(always)]
    pub fn numsg(&self) -> NUMSG_R {
        NUMSG_R::new(self.bits)
    }
}
#[doc = "Flash Access Segment Number Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [facsn](index.html) module"]
pub struct FACSN_SPEC;
impl crate::RegisterSpec for FACSN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [facsn::R](R) reader structure"]
impl crate::Readable for FACSN_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FACSN to value 0"]
impl crate::Resettable for FACSN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
