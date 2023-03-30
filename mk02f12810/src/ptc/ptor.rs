#[doc = "Register `PTOR` writer"]
pub struct W(crate::W<PTOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PTOR_SPEC>;
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
impl From<crate::W<PTOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PTOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Toggle Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PTTO_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    _1 = 1,
}
impl From<PTTO_AW> for u32 {
    #[inline(always)]
    fn from(variant: PTTO_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PTTO` writer - Port Toggle Output"]
pub type PTTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PTOR_SPEC, u32, PTTO_AW, 32, O>;
impl<'a, const O: u8> PTTO_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTTO_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is set to the inverse of its existing logic state."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTTO_AW::_1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Toggle Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptto(&mut self) -> PTTO_W<0> {
        PTTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Toggle Output Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ptor](index.html) module"]
pub struct PTOR_SPEC;
impl crate::RegisterSpec for PTOR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ptor::W](W) writer structure"]
impl crate::Writable for PTOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PTOR to value 0"]
impl crate::Resettable for PTOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
