#[doc = "Register `PCOR` writer"]
pub struct W(crate::W<PCOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCOR_SPEC>;
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
impl From<crate::W<PCOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Port Clear Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PTCO_AW {
    #[doc = "0: Corresponding bit in PDORn does not change."]
    _0 = 0,
    #[doc = "1: Corresponding bit in PDORn is cleared to logic 0."]
    _1 = 1,
}
impl From<PTCO_AW> for u32 {
    #[inline(always)]
    fn from(variant: PTCO_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PTCO` writer - Port Clear Output"]
pub type PTCO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PCOR_SPEC, u32, PTCO_AW, 32, O>;
impl<'a, const O: u8> PTCO_W<'a, O> {
    #[doc = "Corresponding bit in PDORn does not change."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PTCO_AW::_0)
    }
    #[doc = "Corresponding bit in PDORn is cleared to logic 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PTCO_AW::_1)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Clear Output"]
    #[inline(always)]
    #[must_use]
    pub fn ptco(&mut self) -> PTCO_W<0> {
        PTCO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Clear Output Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcor](index.html) module"]
pub struct PCOR_SPEC;
impl crate::RegisterSpec for PCOR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [pcor::W](W) writer structure"]
impl crate::Writable for PCOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PCOR to value 0"]
impl crate::Resettable for PCOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
