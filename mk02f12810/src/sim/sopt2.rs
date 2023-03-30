#[doc = "Register `SOPT2` reader"]
pub struct R(crate::R<SOPT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT2` writer"]
pub struct W(crate::W<SOPT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT2_SPEC>;
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
impl From<crate::W<SOPT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKOUTSEL` reader - CLKOUT select"]
pub type CLKOUTSEL_R = crate::FieldReader<u8, CLKOUTSEL_A>;
#[doc = "CLKOUT select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUTSEL_A {
    #[doc = "2: Flash clock"]
    _010 = 2,
    #[doc = "3: LPO clock (1 kHz)"]
    _011 = 3,
    #[doc = "4: MCGIRCLK"]
    _100 = 4,
    #[doc = "6: OSCERCLK0"]
    _110 = 6,
    #[doc = "7: IRC 48 MHz clock"]
    _111 = 7,
}
impl From<CLKOUTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUTSEL_A) -> Self {
        variant as _
    }
}
impl CLKOUTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUTSEL_A> {
        match self.bits {
            2 => Some(CLKOUTSEL_A::_010),
            3 => Some(CLKOUTSEL_A::_011),
            4 => Some(CLKOUTSEL_A::_100),
            6 => Some(CLKOUTSEL_A::_110),
            7 => Some(CLKOUTSEL_A::_111),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == CLKOUTSEL_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == CLKOUTSEL_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == CLKOUTSEL_A::_100
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == CLKOUTSEL_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == CLKOUTSEL_A::_111
    }
}
#[doc = "Field `CLKOUTSEL` writer - CLKOUT select"]
pub type CLKOUTSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT2_SPEC, u8, CLKOUTSEL_A, 3, O>;
impl<'a, const O: u8> CLKOUTSEL_W<'a, O> {
    #[doc = "Flash clock"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_010)
    }
    #[doc = "LPO clock (1 kHz)"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_011)
    }
    #[doc = "MCGIRCLK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_100)
    }
    #[doc = "OSCERCLK0"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_110)
    }
    #[doc = "IRC 48 MHz clock"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(CLKOUTSEL_A::_111)
    }
}
#[doc = "Field `TRACECLKSEL` reader - Debug trace clock select"]
pub type TRACECLKSEL_R = crate::BitReader<TRACECLKSEL_A>;
#[doc = "Debug trace clock select\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRACECLKSEL_A {
    #[doc = "0: MCGOUTCLK"]
    _0 = 0,
    #[doc = "1: Core/system clock"]
    _1 = 1,
}
impl From<TRACECLKSEL_A> for bool {
    #[inline(always)]
    fn from(variant: TRACECLKSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl TRACECLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRACECLKSEL_A {
        match self.bits {
            false => TRACECLKSEL_A::_0,
            true => TRACECLKSEL_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == TRACECLKSEL_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == TRACECLKSEL_A::_1
    }
}
#[doc = "Field `TRACECLKSEL` writer - Debug trace clock select"]
pub type TRACECLKSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT2_SPEC, TRACECLKSEL_A, O>;
impl<'a, const O: u8> TRACECLKSEL_W<'a, O> {
    #[doc = "MCGOUTCLK"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(TRACECLKSEL_A::_0)
    }
    #[doc = "Core/system clock"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(TRACECLKSEL_A::_1)
    }
}
#[doc = "Field `PLLFLLSEL` reader - PLL/FLL clock select"]
pub type PLLFLLSEL_R = crate::FieldReader<u8, PLLFLLSEL_A>;
#[doc = "PLL/FLL clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLFLLSEL_A {
    #[doc = "0: MCGFLLCLK clock"]
    _00 = 0,
    #[doc = "3: IRC48 MHz clock"]
    _11 = 3,
}
impl From<PLLFLLSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLFLLSEL_A) -> Self {
        variant as _
    }
}
impl PLLFLLSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLFLLSEL_A> {
        match self.bits {
            0 => Some(PLLFLLSEL_A::_00),
            3 => Some(PLLFLLSEL_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PLLFLLSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PLLFLLSEL_A::_11
    }
}
#[doc = "Field `PLLFLLSEL` writer - PLL/FLL clock select"]
pub type PLLFLLSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT2_SPEC, u8, PLLFLLSEL_A, 2, O>;
impl<'a, const O: u8> PLLFLLSEL_W<'a, O> {
    #[doc = "MCGFLLCLK clock"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_00)
    }
    #[doc = "IRC48 MHz clock"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(PLLFLLSEL_A::_11)
    }
}
impl R {
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    pub fn clkoutsel(&self) -> CLKOUTSEL_R {
        CLKOUTSEL_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    pub fn traceclksel(&self) -> TRACECLKSEL_R {
        TRACECLKSEL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline(always)]
    pub fn pllfllsel(&self) -> PLLFLLSEL_R {
        PLLFLLSEL_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 5:7 - CLKOUT select"]
    #[inline(always)]
    #[must_use]
    pub fn clkoutsel(&mut self) -> CLKOUTSEL_W<5> {
        CLKOUTSEL_W::new(self)
    }
    #[doc = "Bit 12 - Debug trace clock select"]
    #[inline(always)]
    #[must_use]
    pub fn traceclksel(&mut self) -> TRACECLKSEL_W<12> {
        TRACECLKSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - PLL/FLL clock select"]
    #[inline(always)]
    #[must_use]
    pub fn pllfllsel(&mut self) -> PLLFLLSEL_W<16> {
        PLLFLLSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt2](index.html) module"]
pub struct SOPT2_SPEC;
impl crate::RegisterSpec for SOPT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt2::R](R) reader structure"]
impl crate::Readable for SOPT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt2::W](W) writer structure"]
impl crate::Writable for SOPT2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPT2 to value 0x1000"]
impl crate::Resettable for SOPT2_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000;
}
