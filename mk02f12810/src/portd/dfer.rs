#[doc = "Register `DFER` reader"]
pub struct R(crate::R<DFER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFER` writer"]
pub struct W(crate::W<DFER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFER_SPEC>;
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
impl From<crate::W<DFER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DFE` reader - Digital Filter Enable"]
pub type DFE_R = crate::FieldReader<u32, DFE_A>;
#[doc = "Digital Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum DFE_A {
    #[doc = "0: Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    _0 = 0,
    #[doc = "1: Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<DFE_A> for u32 {
    #[inline(always)]
    fn from(variant: DFE_A) -> Self {
        variant as _
    }
}
impl DFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DFE_A> {
        match self.bits {
            0 => Some(DFE_A::_0),
            1 => Some(DFE_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DFE_A::_1
    }
}
#[doc = "Field `DFE` writer - Digital Filter Enable"]
pub type DFE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DFER_SPEC, u32, DFE_A, 32, O>;
impl<'a, const O: u8> DFE_W<'a, O> {
    #[doc = "Digital filter is disabled on the corresponding pin and output of the digital filter is reset to zero."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DFE_A::_0)
    }
    #[doc = "Digital filter is enabled on the corresponding pin, if the pin is configured as a digital input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DFE_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Digital Filter Enable"]
    #[inline(always)]
    pub fn dfe(&self) -> DFE_R {
        DFE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dfe(&mut self) -> DFE_W<0> {
        DFE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Digital Filter Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfer](index.html) module"]
pub struct DFER_SPEC;
impl crate::RegisterSpec for DFER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfer::R](R) reader structure"]
impl crate::Readable for DFER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfer::W](W) writer structure"]
impl crate::Writable for DFER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFER to value 0"]
impl crate::Resettable for DFER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
