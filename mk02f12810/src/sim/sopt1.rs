#[doc = "Register `SOPT1` reader"]
pub struct R(crate::R<SOPT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SOPT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SOPT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SOPT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SOPT1` writer"]
pub struct W(crate::W<SOPT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SOPT1_SPEC>;
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
impl From<crate::W<SOPT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SOPT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RAMSIZE` reader - RAM size"]
pub type RAMSIZE_R = crate::FieldReader<u8, RAMSIZE_A>;
#[doc = "RAM size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RAMSIZE_A {
    #[doc = "1: 8 KB"]
    _0001 = 1,
    #[doc = "3: 16 KB"]
    _0011 = 3,
    #[doc = "4: 24 KB"]
    _0100 = 4,
    #[doc = "5: 32 KB"]
    _0101 = 5,
    #[doc = "6: 48 KB"]
    _0110 = 6,
    #[doc = "7: 64 KB"]
    _0111 = 7,
    #[doc = "8: 96 KB"]
    _1000 = 8,
    #[doc = "9: 128 KB"]
    _1001 = 9,
    #[doc = "11: 256 KB"]
    _1011 = 11,
}
impl From<RAMSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: RAMSIZE_A) -> Self {
        variant as _
    }
}
impl RAMSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RAMSIZE_A> {
        match self.bits {
            1 => Some(RAMSIZE_A::_0001),
            3 => Some(RAMSIZE_A::_0011),
            4 => Some(RAMSIZE_A::_0100),
            5 => Some(RAMSIZE_A::_0101),
            6 => Some(RAMSIZE_A::_0110),
            7 => Some(RAMSIZE_A::_0111),
            8 => Some(RAMSIZE_A::_1000),
            9 => Some(RAMSIZE_A::_1001),
            11 => Some(RAMSIZE_A::_1011),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0001`"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == RAMSIZE_A::_0001
    }
    #[doc = "Checks if the value of the field is `_0011`"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == RAMSIZE_A::_0011
    }
    #[doc = "Checks if the value of the field is `_0100`"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == RAMSIZE_A::_0100
    }
    #[doc = "Checks if the value of the field is `_0101`"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == RAMSIZE_A::_0101
    }
    #[doc = "Checks if the value of the field is `_0110`"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == RAMSIZE_A::_0110
    }
    #[doc = "Checks if the value of the field is `_0111`"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == RAMSIZE_A::_0111
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == RAMSIZE_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == RAMSIZE_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == RAMSIZE_A::_1011
    }
}
#[doc = "Field `OSC32KOUT` reader - 32K Oscillator Clock Output"]
pub type OSC32KOUT_R = crate::FieldReader<u8, OSC32KOUT_A>;
#[doc = "32K Oscillator Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSC32KOUT_A {
    #[doc = "0: ERCLK32K is not output."]
    _00 = 0,
    #[doc = "1: ERCLK32K is output on PTE0."]
    _01 = 1,
    #[doc = "2: ERCLK32K is output on PTE26."]
    _10 = 2,
}
impl From<OSC32KOUT_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC32KOUT_A) -> Self {
        variant as _
    }
}
impl OSC32KOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSC32KOUT_A> {
        match self.bits {
            0 => Some(OSC32KOUT_A::_00),
            1 => Some(OSC32KOUT_A::_01),
            2 => Some(OSC32KOUT_A::_10),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OSC32KOUT_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == OSC32KOUT_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == OSC32KOUT_A::_10
    }
}
#[doc = "Field `OSC32KOUT` writer - 32K Oscillator Clock Output"]
pub type OSC32KOUT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT1_SPEC, u8, OSC32KOUT_A, 2, O>;
impl<'a, const O: u8> OSC32KOUT_W<'a, O> {
    #[doc = "ERCLK32K is not output."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC32KOUT_A::_00)
    }
    #[doc = "ERCLK32K is output on PTE0."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(OSC32KOUT_A::_01)
    }
    #[doc = "ERCLK32K is output on PTE26."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(OSC32KOUT_A::_10)
    }
}
#[doc = "Field `OSC32KSEL` reader - 32K oscillator clock select"]
pub type OSC32KSEL_R = crate::FieldReader<u8, OSC32KSEL_A>;
#[doc = "32K oscillator clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSC32KSEL_A {
    #[doc = "0: System oscillator (OSC32KCLK)"]
    _00 = 0,
    #[doc = "3: LPO 1 kHz"]
    _11 = 3,
}
impl From<OSC32KSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: OSC32KSEL_A) -> Self {
        variant as _
    }
}
impl OSC32KSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OSC32KSEL_A> {
        match self.bits {
            0 => Some(OSC32KSEL_A::_00),
            3 => Some(OSC32KSEL_A::_11),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == OSC32KSEL_A::_00
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == OSC32KSEL_A::_11
    }
}
#[doc = "Field `OSC32KSEL` writer - 32K oscillator clock select"]
pub type OSC32KSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SOPT1_SPEC, u8, OSC32KSEL_A, 2, O>;
impl<'a, const O: u8> OSC32KSEL_W<'a, O> {
    #[doc = "System oscillator (OSC32KCLK)"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_00)
    }
    #[doc = "LPO 1 kHz"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(OSC32KSEL_A::_11)
    }
}
impl R {
    #[doc = "Bits 12:15 - RAM size"]
    #[inline(always)]
    pub fn ramsize(&self) -> RAMSIZE_R {
        RAMSIZE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - 32K Oscillator Clock Output"]
    #[inline(always)]
    pub fn osc32kout(&self) -> OSC32KOUT_R {
        OSC32KOUT_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - 32K oscillator clock select"]
    #[inline(always)]
    pub fn osc32ksel(&self) -> OSC32KSEL_R {
        OSC32KSEL_R::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 16:17 - 32K Oscillator Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn osc32kout(&mut self) -> OSC32KOUT_W<16> {
        OSC32KOUT_W::new(self)
    }
    #[doc = "Bits 18:19 - 32K oscillator clock select"]
    #[inline(always)]
    #[must_use]
    pub fn osc32ksel(&mut self) -> OSC32KSEL_W<18> {
        OSC32KSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Options Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sopt1](index.html) module"]
pub struct SOPT1_SPEC;
impl crate::RegisterSpec for SOPT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sopt1::R](R) reader structure"]
impl crate::Readable for SOPT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sopt1::W](W) writer structure"]
impl crate::Writable for SOPT1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SOPT1 to value 0"]
impl crate::Resettable for SOPT1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
