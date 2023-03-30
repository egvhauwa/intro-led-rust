#[doc = "Register `POEN` reader"]
pub struct R(crate::R<POEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POEN` writer"]
pub struct W(crate::W<POEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POEN_SPEC>;
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
impl From<crate::W<POEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POEN` reader - PDB Pulse-Out Enable"]
pub type POEN_R = crate::FieldReader<u8, POEN_A>;
#[doc = "PDB Pulse-Out Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POEN_A {
    #[doc = "0: PDB Pulse-Out disabled"]
    _0 = 0,
    #[doc = "1: PDB Pulse-Out enabled"]
    _1 = 1,
}
impl From<POEN_A> for u8 {
    #[inline(always)]
    fn from(variant: POEN_A) -> Self {
        variant as _
    }
}
impl POEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<POEN_A> {
        match self.bits {
            0 => Some(POEN_A::_0),
            1 => Some(POEN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == POEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == POEN_A::_1
    }
}
#[doc = "Field `POEN` writer - PDB Pulse-Out Enable"]
pub type POEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, POEN_SPEC, u8, POEN_A, 8, O>;
impl<'a, const O: u8> POEN_W<'a, O> {
    #[doc = "PDB Pulse-Out disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(POEN_A::_0)
    }
    #[doc = "PDB Pulse-Out enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(POEN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    pub fn poen(&self) -> POEN_R {
        POEN_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Pulse-Out Enable"]
    #[inline(always)]
    #[must_use]
    pub fn poen(&mut self) -> POEN_W<0> {
        POEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pulse-Out n Enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [poen](index.html) module"]
pub struct POEN_SPEC;
impl crate::RegisterSpec for POEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [poen::R](R) reader structure"]
impl crate::Readable for POEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [poen::W](W) writer structure"]
impl crate::Writable for POEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets POEN to value 0"]
impl crate::Resettable for POEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
