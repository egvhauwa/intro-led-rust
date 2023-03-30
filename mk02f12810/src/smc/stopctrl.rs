#[doc = "Register `STOPCTRL` reader"]
pub struct R(crate::R<STOPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STOPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STOPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STOPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STOPCTRL` writer"]
pub struct W(crate::W<STOPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STOPCTRL_SPEC>;
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
impl From<crate::W<STOPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STOPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LLSM` reader - LLS or VLLS Mode Control"]
pub type LLSM_R = crate::FieldReader<u8, LLSM_A>;
#[doc = "LLS or VLLS Mode Control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LLSM_A {
    #[doc = "0: VLLS0 if PMCTRL\\[STOPM\\]=VLLSx, reserved if PMCTRL\\[STOPM\\]=LLSx"]
    _000 = 0,
    #[doc = "1: VLLS1 if PMCTRL\\[STOPM\\]=VLLSx, reserved if PMCTRL\\[STOPM\\]=LLSx"]
    _001 = 1,
    #[doc = "2: VLLS2 if PMCTRL\\[STOPM\\]=VLLSx, LLS2 if PMCTRL\\[STOPM\\]=LLSx"]
    _010 = 2,
    #[doc = "3: VLLS3 if PMCTRL\\[STOPM\\]=VLLSx, LLS3 if PMCTRL\\[STOPM\\]=LLSx"]
    _011 = 3,
}
impl From<LLSM_A> for u8 {
    #[inline(always)]
    fn from(variant: LLSM_A) -> Self {
        variant as _
    }
}
impl LLSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LLSM_A> {
        match self.bits {
            0 => Some(LLSM_A::_000),
            1 => Some(LLSM_A::_001),
            2 => Some(LLSM_A::_010),
            3 => Some(LLSM_A::_011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == LLSM_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == LLSM_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == LLSM_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == LLSM_A::_011
    }
}
#[doc = "Field `LLSM` writer - LLS or VLLS Mode Control"]
pub type LLSM_W<'a, const O: u8> = crate::FieldWriter<'a, u8, STOPCTRL_SPEC, u8, LLSM_A, 3, O>;
impl<'a, const O: u8> LLSM_W<'a, O> {
    #[doc = "VLLS0 if PMCTRL\\[STOPM\\]=VLLSx, reserved if PMCTRL\\[STOPM\\]=LLSx"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(LLSM_A::_000)
    }
    #[doc = "VLLS1 if PMCTRL\\[STOPM\\]=VLLSx, reserved if PMCTRL\\[STOPM\\]=LLSx"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(LLSM_A::_001)
    }
    #[doc = "VLLS2 if PMCTRL\\[STOPM\\]=VLLSx, LLS2 if PMCTRL\\[STOPM\\]=LLSx"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(LLSM_A::_010)
    }
    #[doc = "VLLS3 if PMCTRL\\[STOPM\\]=VLLSx, LLS3 if PMCTRL\\[STOPM\\]=LLSx"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(LLSM_A::_011)
    }
}
#[doc = "Field `PORPO` reader - POR Power Option"]
pub type PORPO_R = crate::BitReader<PORPO_A>;
#[doc = "POR Power Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PORPO_A {
    #[doc = "0: POR detect circuit is enabled in VLLS0"]
    _0 = 0,
    #[doc = "1: POR detect circuit is disabled in VLLS0"]
    _1 = 1,
}
impl From<PORPO_A> for bool {
    #[inline(always)]
    fn from(variant: PORPO_A) -> Self {
        variant as u8 != 0
    }
}
impl PORPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PORPO_A {
        match self.bits {
            false => PORPO_A::_0,
            true => PORPO_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PORPO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PORPO_A::_1
    }
}
#[doc = "Field `PORPO` writer - POR Power Option"]
pub type PORPO_W<'a, const O: u8> = crate::BitWriter<'a, u8, STOPCTRL_SPEC, PORPO_A, O>;
impl<'a, const O: u8> PORPO_W<'a, O> {
    #[doc = "POR detect circuit is enabled in VLLS0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PORPO_A::_0)
    }
    #[doc = "POR detect circuit is disabled in VLLS0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PORPO_A::_1)
    }
}
#[doc = "Field `PSTOPO` reader - Partial Stop Option"]
pub type PSTOPO_R = crate::FieldReader<u8, PSTOPO_A>;
#[doc = "Partial Stop Option\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSTOPO_A {
    #[doc = "0: STOP - Normal Stop mode"]
    _00 = 0,
    #[doc = "1: PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    _01 = 1,
    #[doc = "2: PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    _10 = 2,
}
impl From<PSTOPO_A> for u8 {
    #[inline(always)]
    fn from(variant: PSTOPO_A) -> Self {
        variant as _
    }
}
impl PSTOPO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSTOPO_A> {
        match self.bits {
            0 => Some(PSTOPO_A::_00),
            1 => Some(PSTOPO_A::_01),
            2 => Some(PSTOPO_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PSTOPO_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PSTOPO_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PSTOPO_A::_10
    }
}
#[doc = "Field `PSTOPO` writer - Partial Stop Option"]
pub type PSTOPO_W<'a, const O: u8> = crate::FieldWriter<'a, u8, STOPCTRL_SPEC, u8, PSTOPO_A, 2, O>;
impl<'a, const O: u8> PSTOPO_W<'a, O> {
    #[doc = "STOP - Normal Stop mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PSTOPO_A::_00)
    }
    #[doc = "PSTOP1 - Partial Stop with both system and bus clocks disabled"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(PSTOPO_A::_01)
    }
    #[doc = "PSTOP2 - Partial Stop with system clock disabled and bus clock enabled"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(PSTOPO_A::_10)
    }
}
impl R {
    #[doc = "Bits 0:2 - LLS or VLLS Mode Control"]
    #[inline(always)]
    pub fn llsm(&self) -> LLSM_R {
        LLSM_R::new(self.bits & 7)
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    pub fn porpo(&self) -> PORPO_R {
        PORPO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Partial Stop Option"]
    #[inline(always)]
    pub fn pstopo(&self) -> PSTOPO_R {
        PSTOPO_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:2 - LLS or VLLS Mode Control"]
    #[inline(always)]
    #[must_use]
    pub fn llsm(&mut self) -> LLSM_W<0> {
        LLSM_W::new(self)
    }
    #[doc = "Bit 5 - POR Power Option"]
    #[inline(always)]
    #[must_use]
    pub fn porpo(&mut self) -> PORPO_W<5> {
        PORPO_W::new(self)
    }
    #[doc = "Bits 6:7 - Partial Stop Option"]
    #[inline(always)]
    #[must_use]
    pub fn pstopo(&mut self) -> PSTOPO_W<6> {
        PSTOPO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Stop Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stopctrl](index.html) module"]
pub struct STOPCTRL_SPEC;
impl crate::RegisterSpec for STOPCTRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [stopctrl::R](R) reader structure"]
impl crate::Readable for STOPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stopctrl::W](W) writer structure"]
impl crate::Writable for STOPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STOPCTRL to value 0x03"]
impl crate::Resettable for STOPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
