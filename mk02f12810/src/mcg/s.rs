#[doc = "Register `S` reader"]
pub struct R(crate::R<S_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IRCST` reader - Internal Reference Clock Status"]
pub type IRCST_R = crate::BitReader<IRCST_A>;
#[doc = "Internal Reference Clock Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRCST_A {
    #[doc = "0: Source of internal reference clock is the slow clock (32 kHz IRC)."]
    _0 = 0,
    #[doc = "1: Source of internal reference clock is the fast clock (4 MHz IRC)."]
    _1 = 1,
}
impl From<IRCST_A> for bool {
    #[inline(always)]
    fn from(variant: IRCST_A) -> Self {
        variant as u8 != 0
    }
}
impl IRCST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRCST_A {
        match self.bits {
            false => IRCST_A::_0,
            true => IRCST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IRCST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IRCST_A::_1
    }
}
#[doc = "Field `OSCINIT0` reader - OSC Initialization"]
pub type OSCINIT0_R = crate::BitReader<bool>;
#[doc = "Field `CLKST` reader - Clock Mode Status"]
pub type CLKST_R = crate::FieldReader<u8, CLKST_A>;
#[doc = "Clock Mode Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKST_A {
    #[doc = "0: Encoding 0 - Output of the FLL is selected (reset default)."]
    _00 = 0,
    #[doc = "1: Encoding 1 - Internal reference clock is selected."]
    _01 = 1,
    #[doc = "2: Encoding 2 - External reference clock is selected."]
    _10 = 2,
}
impl From<CLKST_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKST_A) -> Self {
        variant as _
    }
}
impl CLKST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKST_A> {
        match self.bits {
            0 => Some(CLKST_A::_00),
            1 => Some(CLKST_A::_01),
            2 => Some(CLKST_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == CLKST_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == CLKST_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == CLKST_A::_10
    }
}
#[doc = "Field `IREFST` reader - Internal Reference Status"]
pub type IREFST_R = crate::BitReader<IREFST_A>;
#[doc = "Internal Reference Status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREFST_A {
    #[doc = "0: Source of FLL reference clock is the external reference clock."]
    _0 = 0,
    #[doc = "1: Source of FLL reference clock is the internal reference clock."]
    _1 = 1,
}
impl From<IREFST_A> for bool {
    #[inline(always)]
    fn from(variant: IREFST_A) -> Self {
        variant as u8 != 0
    }
}
impl IREFST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREFST_A {
        match self.bits {
            false => IREFST_A::_0,
            true => IREFST_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == IREFST_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == IREFST_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Internal Reference Clock Status"]
    #[inline(always)]
    pub fn ircst(&self) -> IRCST_R {
        IRCST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OSC Initialization"]
    #[inline(always)]
    pub fn oscinit0(&self) -> OSCINIT0_R {
        OSCINIT0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Clock Mode Status"]
    #[inline(always)]
    pub fn clkst(&self) -> CLKST_R {
        CLKST_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - Internal Reference Status"]
    #[inline(always)]
    pub fn irefst(&self) -> IREFST_R {
        IREFST_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "MCG Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s](index.html) module"]
pub struct S_SPEC;
impl crate::RegisterSpec for S_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [s::R](R) reader structure"]
impl crate::Readable for S_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets S to value 0x10"]
impl crate::Resettable for S_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
