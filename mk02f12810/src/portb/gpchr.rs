#[doc = "Register `GPCHR` writer"]
pub struct W(crate::W<GPCHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPCHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GPCHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPCHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPWD` writer - Global Pin Write Data"]
pub type GPWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPCHR_SPEC, u16, u16, 16, O>;
#[doc = "Global Pin Write Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum GPWE_AW {
    #[doc = "0: Corresponding Pin Control Register is not updated with the value in GPWD."]
    _0 = 0,
    #[doc = "1: Corresponding Pin Control Register is updated with the value in GPWD."]
    _1 = 1,
}
impl From<GPWE_AW> for u16 {
    #[inline(always)]
    fn from(variant: GPWE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `GPWE` writer - Global Pin Write Enable"]
pub type GPWE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GPCHR_SPEC, u16, GPWE_AW, 16, O>;
impl<'a, const O: u8> GPWE_W<'a, O> {
    #[doc = "Corresponding Pin Control Register is not updated with the value in GPWD."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(GPWE_AW::_0)
    }
    #[doc = "Corresponding Pin Control Register is updated with the value in GPWD."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(GPWE_AW::_1)
    }
}
impl W {
    #[doc = "Bits 0:15 - Global Pin Write Data"]
    #[inline(always)]
    #[must_use]
    pub fn gpwd(&mut self) -> GPWD_W<0> {
        GPWD_W::new(self)
    }
    #[doc = "Bits 16:31 - Global Pin Write Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gpwe(&mut self) -> GPWE_W<16> {
        GPWE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Pin Control High Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpchr](index.html) module"]
pub struct GPCHR_SPEC;
impl crate::RegisterSpec for GPCHR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [gpchr::W](W) writer structure"]
impl crate::Writable for GPCHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPCHR to value 0"]
impl crate::Resettable for GPCHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
