#[doc = "Register `CHCFG%s` reader"]
pub struct R(crate::R<CHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCFG%s` writer"]
pub struct W(crate::W<CHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCFG_SPEC>;
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
impl From<crate::W<CHCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOURCE` reader - DMA Channel Source (Slot)"]
pub type SOURCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOURCE` writer - DMA Channel Source (Slot)"]
pub type SOURCE_W<'a, const O: u8> = crate::FieldWriter<'a, u8, CHCFG_SPEC, u8, u8, 6, O>;
#[doc = "Field `TRIG` reader - DMA Channel Trigger Enable"]
pub type TRIG_R = crate::BitReader<TRIG_A>;
#[doc = "DMA Channel Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG_A {
    #[doc = "0: Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    _0 = 0,
    #[doc = "1: Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    _1 = 1,
}
impl From<TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG_A {
        match self.bits {
            false => TRIG_A::_0,
            true => TRIG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRIG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRIG_A::_1
    }
}
#[doc = "Field `TRIG` writer - DMA Channel Trigger Enable"]
pub type TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHCFG_SPEC, TRIG_A, O>;
impl<'a, const O: u8> TRIG_W<'a, O> {
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRIG_A::_0)
    }
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMAMUX is in Periodic Trigger mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRIG_A::_1)
    }
}
#[doc = "Field `ENBL` reader - DMA Channel Enable"]
pub type ENBL_R = crate::BitReader<ENBL_A>;
#[doc = "DMA Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENBL_A {
    #[doc = "0: DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    _0 = 0,
    #[doc = "1: DMA channel is enabled"]
    _1 = 1,
}
impl From<ENBL_A> for bool {
    #[inline(always)]
    fn from(variant: ENBL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBL_A {
        match self.bits {
            false => ENBL_A::_0,
            true => ENBL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENBL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENBL_A::_1
    }
}
#[doc = "Field `ENBL` writer - DMA Channel Enable"]
pub type ENBL_W<'a, const O: u8> = crate::BitWriter<'a, u8, CHCFG_SPEC, ENBL_A, O>;
impl<'a, const O: u8> ENBL_W<'a, O> {
    #[doc = "DMA channel is disabled. This mode is primarily used during configuration of the DMAMux. The DMA has separate channel enables/disables, which should be used to disable or reconfigure a DMA channel."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ENBL_A::_0)
    }
    #[doc = "DMA channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ENBL_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline(always)]
    pub fn enbl(&self) -> ENBL_R {
        ENBL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA Channel Source (Slot)"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<0> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 6 - DMA Channel Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TRIG_W<6> {
        TRIG_W::new(self)
    }
    #[doc = "Bit 7 - DMA Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbl(&mut self) -> ENBL_W<7> {
        ENBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chcfg](index.html) module"]
pub struct CHCFG_SPEC;
impl crate::RegisterSpec for CHCFG_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [chcfg::R](R) reader structure"]
impl crate::Readable for CHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chcfg::W](W) writer structure"]
impl crate::Writable for CHCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCFG%s to value 0"]
impl crate::Resettable for CHCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
