#[doc = "Register `SCGC7` reader"]
pub struct R(crate::R<SCGC7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGC7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGC7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC7` writer"]
pub struct W(crate::W<SCGC7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC7_SPEC>;
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
impl From<crate::W<SCGC7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGC7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA` reader - DMA Clock Gate Control"]
pub type DMA_R = crate::BitReader<DMA_A>;
#[doc = "DMA Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DMA_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_A {
        match self.bits {
            false => DMA_A::_0,
            true => DMA_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMA_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMA_A::_1
    }
}
#[doc = "Field `DMA` writer - DMA Clock Gate Control"]
pub type DMA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC7_SPEC, DMA_A, O>;
impl<'a, const O: u8> DMA_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMA_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMA_A::_1)
    }
}
impl R {
    #[doc = "Bit 1 - DMA Clock Gate Control"]
    #[inline(always)]
    pub fn dma(&self) -> DMA_R {
        DMA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DMA Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dma(&mut self) -> DMA_W<1> {
        DMA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc7](index.html) module"]
pub struct SCGC7_SPEC;
impl crate::RegisterSpec for SCGC7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc7::R](R) reader structure"]
impl crate::Readable for SCGC7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc7::W](W) writer structure"]
impl crate::Writable for SCGC7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGC7 to value 0x02"]
impl crate::Resettable for SCGC7_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
