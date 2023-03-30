#[doc = "Register `PLACR` reader"]
pub struct R(crate::R<PLACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PLACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PLACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PLACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PLACR` writer"]
pub struct W(crate::W<PLACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PLACR_SPEC>;
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
impl From<crate::W<PLACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PLACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARB` reader - Arbitration select"]
pub type ARB_R = crate::BitReader<ARB_A>;
#[doc = "Arbitration select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARB_A {
    #[doc = "0: Fixed-priority arbitration for the crossbar masters"]
    _0 = 0,
    #[doc = "1: Round-robin arbitration for the crossbar masters"]
    _1 = 1,
}
impl From<ARB_A> for bool {
    #[inline(always)]
    fn from(variant: ARB_A) -> Self {
        variant as u8 != 0
    }
}
impl ARB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARB_A {
        match self.bits {
            false => ARB_A::_0,
            true => ARB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ARB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ARB_A::_1
    }
}
#[doc = "Field `ARB` writer - Arbitration select"]
pub type ARB_W<'a, const O: u8> = crate::BitWriter<'a, u32, PLACR_SPEC, ARB_A, O>;
impl<'a, const O: u8> ARB_W<'a, O> {
    #[doc = "Fixed-priority arbitration for the crossbar masters"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ARB_A::_0)
    }
    #[doc = "Round-robin arbitration for the crossbar masters"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ARB_A::_1)
    }
}
impl R {
    #[doc = "Bit 9 - Arbitration select"]
    #[inline(always)]
    pub fn arb(&self) -> ARB_R {
        ARB_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 9 - Arbitration select"]
    #[inline(always)]
    #[must_use]
    pub fn arb(&mut self) -> ARB_W<9> {
        ARB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar Switch (AXBS) Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [placr](index.html) module"]
pub struct PLACR_SPEC;
impl crate::RegisterSpec for PLACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [placr::R](R) reader structure"]
impl crate::Readable for PLACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [placr::W](W) writer structure"]
impl crate::Writable for PLACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PLACR to value 0"]
impl crate::Resettable for PLACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
