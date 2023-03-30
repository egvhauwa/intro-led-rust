#[doc = "Register `CH%sC1` reader"]
pub struct R(crate::R<CHC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sC1` writer"]
pub struct W(crate::W<CHC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHC1_SPEC>;
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
impl From<crate::W<CHC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - PDB Channel Pre-Trigger Enable"]
pub type EN_R = crate::FieldReader<u8, EN_A>;
#[doc = "PDB Channel Pre-Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EN_A {
    #[doc = "0: PDB channel's corresponding pre-trigger disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger enabled."]
    _1 = 1,
}
impl From<EN_A> for u8 {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as _
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EN_A> {
        match self.bits {
            0 => Some(EN_A::_0),
            1 => Some(EN_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EN_A::_1
    }
}
#[doc = "Field `EN` writer - PDB Channel Pre-Trigger Enable"]
pub type EN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHC1_SPEC, u8, EN_A, 8, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EN_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EN_A::_1)
    }
}
#[doc = "Field `TOS` reader - PDB Channel Pre-Trigger Output Select"]
pub type TOS_R = crate::FieldReader<u8, TOS_A>;
#[doc = "PDB Channel Pre-Trigger Output Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TOS_A {
    #[doc = "0: PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    _1 = 1,
}
impl From<TOS_A> for u8 {
    #[inline(always)]
    fn from(variant: TOS_A) -> Self {
        variant as _
    }
}
impl TOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TOS_A> {
        match self.bits {
            0 => Some(TOS_A::_0),
            1 => Some(TOS_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TOS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TOS_A::_1
    }
}
#[doc = "Field `TOS` writer - PDB Channel Pre-Trigger Output Select"]
pub type TOS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHC1_SPEC, u8, TOS_A, 8, O>;
impl<'a, const O: u8> TOS_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger is in bypassed mode. The pre-trigger asserts one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SWTRIG is written with 1."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TOS_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger asserts when the counter reaches the channel delay register and one peripheral clock cycle after a rising edge is detected on selected trigger input source or software trigger is selected and SETRIG is written with 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TOS_A::_1)
    }
}
#[doc = "Field `BB` reader - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB_R = crate::FieldReader<u8, BB_A>;
#[doc = "PDB Channel Pre-Trigger Back-to-Back Operation Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BB_A {
    #[doc = "0: PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    _0 = 0,
    #[doc = "1: PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    _1 = 1,
}
impl From<BB_A> for u8 {
    #[inline(always)]
    fn from(variant: BB_A) -> Self {
        variant as _
    }
}
impl BB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BB_A> {
        match self.bits {
            0 => Some(BB_A::_0),
            1 => Some(BB_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == BB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == BB_A::_1
    }
}
#[doc = "Field `BB` writer - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
pub type BB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHC1_SPEC, u8, BB_A, 8, O>;
impl<'a, const O: u8> BB_W<'a, O> {
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(BB_A::_0)
    }
    #[doc = "PDB channel's corresponding pre-trigger back-to-back operation enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(BB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    pub fn tos(&self) -> TOS_R {
        TOS_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Channel Pre-Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bits 8:15 - PDB Channel Pre-Trigger Output Select"]
    #[inline(always)]
    #[must_use]
    pub fn tos(&mut self) -> TOS_W<8> {
        TOS_W::new(self)
    }
    #[doc = "Bits 16:23 - PDB Channel Pre-Trigger Back-to-Back Operation Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bb(&mut self) -> BB_W<16> {
        BB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chc1](index.html) module"]
pub struct CHC1_SPEC;
impl crate::RegisterSpec for CHC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chc1::R](R) reader structure"]
impl crate::Readable for CHC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chc1::W](W) writer structure"]
impl crate::Writable for CHC1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%sC1 to value 0"]
impl crate::Resettable for CHC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
