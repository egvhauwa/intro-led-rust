#[doc = "Register `SOPT8` reader"]
pub struct R(crate::R<SOPT8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT8` writer"]
pub struct W(crate::W<SOPT8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT8_SPEC>;
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
impl From<crate::W<SOPT8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTM0SYNCBIT` reader - FTM0 Hardware Trigger 0 Software Synchronization"]
pub type FTM0SYNCBIT_R = crate::BitReader<FTM0SYNCBIT_A>;
#[doc = "FTM0 Hardware Trigger 0 Software Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0SYNCBIT_A {
    #[doc = "0: No effect"]
    _0 = 0,
    #[doc = "1: Write 1 to assert the TRIG0 input to FTM0, software must clear this bit to allow other trigger sources to assert."]
    _1 = 1,
}
impl From<FTM0SYNCBIT_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0SYNCBIT_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0SYNCBIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0SYNCBIT_A {
        match self.bits {
            false => FTM0SYNCBIT_A::_0,
            true => FTM0SYNCBIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0SYNCBIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0SYNCBIT_A::_1
    }
}
#[doc = "Field `FTM0SYNCBIT` writer - FTM0 Hardware Trigger 0 Software Synchronization"]
pub type FTM0SYNCBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT8_SPEC, FTM0SYNCBIT_A, O>;
impl<'a, const O: u8> FTM0SYNCBIT_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0SYNCBIT_A::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM0, software must clear this bit to allow other trigger sources to assert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0SYNCBIT_A::_1)
    }
}
#[doc = "Field `FTM1SYNCBIT` reader - FTM1 Hardware Trigger 0 Software Synchronization"]
pub type FTM1SYNCBIT_R = crate::BitReader<FTM1SYNCBIT_A>;
#[doc = "FTM1 Hardware Trigger 0 Software Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM1SYNCBIT_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Write 1 to assert the TRIG0 input to FTM1, software must clear this bit to allow other trigger sources to assert."]
    _1 = 1,
}
impl From<FTM1SYNCBIT_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1SYNCBIT_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM1SYNCBIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1SYNCBIT_A {
        match self.bits {
            false => FTM1SYNCBIT_A::_0,
            true => FTM1SYNCBIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1SYNCBIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1SYNCBIT_A::_1
    }
}
#[doc = "Field `FTM1SYNCBIT` writer - FTM1 Hardware Trigger 0 Software Synchronization"]
pub type FTM1SYNCBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT8_SPEC, FTM1SYNCBIT_A, O>;
impl<'a, const O: u8> FTM1SYNCBIT_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1SYNCBIT_A::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM1, software must clear this bit to allow other trigger sources to assert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1SYNCBIT_A::_1)
    }
}
#[doc = "Field `FTM2SYNCBIT` reader - FTM2 Hardware Trigger 0 Software Synchronization"]
pub type FTM2SYNCBIT_R = crate::BitReader<FTM2SYNCBIT_A>;
#[doc = "FTM2 Hardware Trigger 0 Software Synchronization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM2SYNCBIT_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: Write 1 to assert the TRIG0 input to FTM2, software must clear this bit to allow other trigger sources to assert."]
    _1 = 1,
}
impl From<FTM2SYNCBIT_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2SYNCBIT_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM2SYNCBIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2SYNCBIT_A {
        match self.bits {
            false => FTM2SYNCBIT_A::_0,
            true => FTM2SYNCBIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2SYNCBIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2SYNCBIT_A::_1
    }
}
#[doc = "Field `FTM2SYNCBIT` writer - FTM2 Hardware Trigger 0 Software Synchronization"]
pub type FTM2SYNCBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT8_SPEC, FTM2SYNCBIT_A, O>;
impl<'a, const O: u8> FTM2SYNCBIT_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2SYNCBIT_A::_0)
    }
    #[doc = "Write 1 to assert the TRIG0 input to FTM2, software must clear this bit to allow other trigger sources to assert."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2SYNCBIT_A::_1)
    }
}
#[doc = "Field `FTM0OCH0SRC` reader - FTM0 channel 0 output source"]
pub type FTM0OCH0SRC_R = crate::BitReader<FTM0OCH0SRC_A>;
#[doc = "FTM0 channel 0 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0OCH0SRC_A {
    #[doc = "0: FTM0_CH0 pin is output of FTM0 channel 0 output"]
    _0 = 0,
    #[doc = "1: FTM0_CH0 pin is output of FTM0 channel 0 output, modulated by FTM1 channel 1 output"]
    _1 = 1,
}
impl From<FTM0OCH0SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH0SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0OCH0SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH0SRC_A {
        match self.bits {
            false => FTM0OCH0SRC_A::_0,
            true => FTM0OCH0SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH0SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH0SRC_A::_1
    }
}
#[doc = "Field `FTM0OCH0SRC` writer - FTM0 channel 0 output source"]
pub type FTM0OCH0SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT8_SPEC, FTM0OCH0SRC_A, O>;
impl<'a, const O: u8> FTM0OCH0SRC_W<'a, O> {
    #[doc = "FTM0_CH0 pin is output of FTM0 channel 0 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH0SRC_A::_0)
    }
    #[doc = "FTM0_CH0 pin is output of FTM0 channel 0 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH0SRC_A::_1)
    }
}
#[doc = "Field `FTM0OCH1SRC` reader - FTM0 channel 1 output source"]
pub type FTM0OCH1SRC_R = crate::BitReader<FTM0OCH1SRC_A>;
#[doc = "FTM0 channel 1 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0OCH1SRC_A {
    #[doc = "0: FTM0_CH1 pin is output of FTM0 channel 1 output"]
    _0 = 0,
    #[doc = "1: FTM0_CH1 pin is output of FTM0 channel 1 output, modulated by FTM1 channel 1 output"]
    _1 = 1,
}
impl From<FTM0OCH1SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH1SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0OCH1SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH1SRC_A {
        match self.bits {
            false => FTM0OCH1SRC_A::_0,
            true => FTM0OCH1SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH1SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH1SRC_A::_1
    }
}
#[doc = "Field `FTM0OCH1SRC` writer - FTM0 channel 1 output source"]
pub type FTM0OCH1SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT8_SPEC, FTM0OCH1SRC_A, O>;
impl<'a, const O: u8> FTM0OCH1SRC_W<'a, O> {
    #[doc = "FTM0_CH1 pin is output of FTM0 channel 1 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH1SRC_A::_0)
    }
    #[doc = "FTM0_CH1 pin is output of FTM0 channel 1 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH1SRC_A::_1)
    }
}
#[doc = "Field `FTM0OCH2SRC` reader - FTM0 channel 2 output source"]
pub type FTM0OCH2SRC_R = crate::BitReader<FTM0OCH2SRC_A>;
#[doc = "FTM0 channel 2 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0OCH2SRC_A {
    #[doc = "0: FTM0_CH2 pin is output of FTM0 channel 2 output"]
    _0 = 0,
    #[doc = "1: FTM0_CH2 pin is output of FTM0 channel 2 output, modulated by FTM1 channel 1 output"]
    _1 = 1,
}
impl From<FTM0OCH2SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH2SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0OCH2SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH2SRC_A {
        match self.bits {
            false => FTM0OCH2SRC_A::_0,
            true => FTM0OCH2SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH2SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH2SRC_A::_1
    }
}
#[doc = "Field `FTM0OCH2SRC` writer - FTM0 channel 2 output source"]
pub type FTM0OCH2SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT8_SPEC, FTM0OCH2SRC_A, O>;
impl<'a, const O: u8> FTM0OCH2SRC_W<'a, O> {
    #[doc = "FTM0_CH2 pin is output of FTM0 channel 2 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH2SRC_A::_0)
    }
    #[doc = "FTM0_CH2 pin is output of FTM0 channel 2 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH2SRC_A::_1)
    }
}
#[doc = "Field `FTM0OCH3SRC` reader - FTM0 channel 3 output source"]
pub type FTM0OCH3SRC_R = crate::BitReader<FTM0OCH3SRC_A>;
#[doc = "FTM0 channel 3 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0OCH3SRC_A {
    #[doc = "0: FTM0_CH3 pin is output of FTM0 channel 3 output"]
    _0 = 0,
    #[doc = "1: FTM0_CH3 pin is output of FTM0 channel 3 output, modulated by FTM1 channel 1 output"]
    _1 = 1,
}
impl From<FTM0OCH3SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH3SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0OCH3SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH3SRC_A {
        match self.bits {
            false => FTM0OCH3SRC_A::_0,
            true => FTM0OCH3SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH3SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH3SRC_A::_1
    }
}
#[doc = "Field `FTM0OCH3SRC` writer - FTM0 channel 3 output source"]
pub type FTM0OCH3SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT8_SPEC, FTM0OCH3SRC_A, O>;
impl<'a, const O: u8> FTM0OCH3SRC_W<'a, O> {
    #[doc = "FTM0_CH3 pin is output of FTM0 channel 3 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH3SRC_A::_0)
    }
    #[doc = "FTM0_CH3 pin is output of FTM0 channel 3 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH3SRC_A::_1)
    }
}
#[doc = "Field `FTM0OCH4SRC` reader - FTM0 channel 4 output source"]
pub type FTM0OCH4SRC_R = crate::BitReader<FTM0OCH4SRC_A>;
#[doc = "FTM0 channel 4 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0OCH4SRC_A {
    #[doc = "0: FTM0_CH4 pin is output of FTM0 channel 4 output"]
    _0 = 0,
    #[doc = "1: FTM0_CH4 pin is output of FTM0 channel 4 output, modulated by FTM1 channel 1 output"]
    _1 = 1,
}
impl From<FTM0OCH4SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH4SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0OCH4SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH4SRC_A {
        match self.bits {
            false => FTM0OCH4SRC_A::_0,
            true => FTM0OCH4SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH4SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH4SRC_A::_1
    }
}
#[doc = "Field `FTM0OCH4SRC` writer - FTM0 channel 4 output source"]
pub type FTM0OCH4SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT8_SPEC, FTM0OCH4SRC_A, O>;
impl<'a, const O: u8> FTM0OCH4SRC_W<'a, O> {
    #[doc = "FTM0_CH4 pin is output of FTM0 channel 4 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH4SRC_A::_0)
    }
    #[doc = "FTM0_CH4 pin is output of FTM0 channel 4 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH4SRC_A::_1)
    }
}
#[doc = "Field `FTM0OCH5SRC` reader - FTM0 channel 5 output source"]
pub type FTM0OCH5SRC_R = crate::BitReader<FTM0OCH5SRC_A>;
#[doc = "FTM0 channel 5 output source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0OCH5SRC_A {
    #[doc = "0: FTM0_CH5 pin is output of FTM0 channel 5 output"]
    _0 = 0,
    #[doc = "1: FTM0_CH5 pin is output of FTM0 channel 5 output, modulated by FTM1 channel 1 output"]
    _1 = 1,
}
impl From<FTM0OCH5SRC_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0OCH5SRC_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0OCH5SRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0OCH5SRC_A {
        match self.bits {
            false => FTM0OCH5SRC_A::_0,
            true => FTM0OCH5SRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0OCH5SRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0OCH5SRC_A::_1
    }
}
#[doc = "Field `FTM0OCH5SRC` writer - FTM0 channel 5 output source"]
pub type FTM0OCH5SRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SOPT8_SPEC, FTM0OCH5SRC_A, O>;
impl<'a, const O: u8> FTM0OCH5SRC_W<'a, O> {
    #[doc = "FTM0_CH5 pin is output of FTM0 channel 5 output"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0OCH5SRC_A::_0)
    }
    #[doc = "FTM0_CH5 pin is output of FTM0 channel 5 output, modulated by FTM1 channel 1 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0OCH5SRC_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - FTM0 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm0syncbit(&self) -> FTM0SYNCBIT_R {
        FTM0SYNCBIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FTM1 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm1syncbit(&self) -> FTM1SYNCBIT_R {
        FTM1SYNCBIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FTM2 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    pub fn ftm2syncbit(&self) -> FTM2SYNCBIT_R {
        FTM2SYNCBIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 16 - FTM0 channel 0 output source"]
    #[inline(always)]
    pub fn ftm0och0src(&self) -> FTM0OCH0SRC_R {
        FTM0OCH0SRC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FTM0 channel 1 output source"]
    #[inline(always)]
    pub fn ftm0och1src(&self) -> FTM0OCH1SRC_R {
        FTM0OCH1SRC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FTM0 channel 2 output source"]
    #[inline(always)]
    pub fn ftm0och2src(&self) -> FTM0OCH2SRC_R {
        FTM0OCH2SRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - FTM0 channel 3 output source"]
    #[inline(always)]
    pub fn ftm0och3src(&self) -> FTM0OCH3SRC_R {
        FTM0OCH3SRC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - FTM0 channel 4 output source"]
    #[inline(always)]
    pub fn ftm0och4src(&self) -> FTM0OCH4SRC_R {
        FTM0OCH4SRC_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FTM0 channel 5 output source"]
    #[inline(always)]
    pub fn ftm0och5src(&self) -> FTM0OCH5SRC_R {
        FTM0OCH5SRC_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FTM0 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0syncbit(&mut self) -> FTM0SYNCBIT_W<0> {
        FTM0SYNCBIT_W::new(self)
    }
    #[doc = "Bit 1 - FTM1 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn ftm1syncbit(&mut self) -> FTM1SYNCBIT_W<1> {
        FTM1SYNCBIT_W::new(self)
    }
    #[doc = "Bit 2 - FTM2 Hardware Trigger 0 Software Synchronization"]
    #[inline(always)]
    #[must_use]
    pub fn ftm2syncbit(&mut self) -> FTM2SYNCBIT_W<2> {
        FTM2SYNCBIT_W::new(self)
    }
    #[doc = "Bit 16 - FTM0 channel 0 output source"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0och0src(&mut self) -> FTM0OCH0SRC_W<16> {
        FTM0OCH0SRC_W::new(self)
    }
    #[doc = "Bit 17 - FTM0 channel 1 output source"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0och1src(&mut self) -> FTM0OCH1SRC_W<17> {
        FTM0OCH1SRC_W::new(self)
    }
    #[doc = "Bit 18 - FTM0 channel 2 output source"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0och2src(&mut self) -> FTM0OCH2SRC_W<18> {
        FTM0OCH2SRC_W::new(self)
    }
    #[doc = "Bit 19 - FTM0 channel 3 output source"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0och3src(&mut self) -> FTM0OCH3SRC_W<19> {
        FTM0OCH3SRC_W::new(self)
    }
    #[doc = "Bit 20 - FTM0 channel 4 output source"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0och4src(&mut self) -> FTM0OCH4SRC_W<20> {
        FTM0OCH4SRC_W::new(self)
    }
    #[doc = "Bit 21 - FTM0 channel 5 output source"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0och5src(&mut self) -> FTM0OCH5SRC_W<21> {
        FTM0OCH5SRC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt8](index.html) module"]
pub struct SOPT8_SPEC;
impl crate::RegisterSpec for SOPT8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt8::R](R) reader structure"]
impl crate::Readable for SOPT8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt8::W](W) writer structure"]
impl crate::Writable for SOPT8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPT8 to value 0"]
impl crate::Resettable for SOPT8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
