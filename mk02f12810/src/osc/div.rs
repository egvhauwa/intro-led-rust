#[doc = "Register `DIV` reader"]
pub struct R(crate::R<DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV` writer"]
pub struct W(crate::W<DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_SPEC>;
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
impl From<crate::W<DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERPS` reader - ERCLK prescaler"]
pub type ERPS_R = crate::FieldReader<u8, ERPS_A>;
#[doc = "ERCLK prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERPS_A {
    #[doc = "0: The divisor ratio is 1."]
    _00 = 0,
    #[doc = "1: The divisor ratio is 2."]
    _01 = 1,
    #[doc = "2: The divisor ratio is 4."]
    _10 = 2,
    #[doc = "3: The divisor ratio is 8."]
    _11 = 3,
}
impl From<ERPS_A> for u8 {
    #[inline(always)]
    fn from(variant: ERPS_A) -> Self {
        variant as _
    }
}
impl ERPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERPS_A {
        match self.bits {
            0 => ERPS_A::_00,
            1 => ERPS_A::_01,
            2 => ERPS_A::_10,
            3 => ERPS_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_00`"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ERPS_A::_00
    }
    #[doc = "Checks if the value of the field is `_01`"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ERPS_A::_01
    }
    #[doc = "Checks if the value of the field is `_10`"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == ERPS_A::_10
    }
    #[doc = "Checks if the value of the field is `_11`"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == ERPS_A::_11
    }
}
#[doc = "Field `ERPS` writer - ERCLK prescaler"]
pub type ERPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, DIV_SPEC, u8, ERPS_A, 2, O>;
impl<'a, const O: u8> ERPS_W<'a, O> {
    #[doc = "The divisor ratio is 1."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut W {
        self.variant(ERPS_A::_00)
    }
    #[doc = "The divisor ratio is 2."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut W {
        self.variant(ERPS_A::_01)
    }
    #[doc = "The divisor ratio is 4."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut W {
        self.variant(ERPS_A::_10)
    }
    #[doc = "The divisor ratio is 8."]
    #[inline(always)]
    pub fn _11(self) -> &'a mut W {
        self.variant(ERPS_A::_11)
    }
}
impl R {
    #[doc = "Bits 6:7 - ERCLK prescaler"]
    #[inline(always)]
    pub fn erps(&self) -> ERPS_R {
        ERPS_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 6:7 - ERCLK prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn erps(&mut self) -> ERPS_W<6> {
        ERPS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OSC_DIV\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div](index.html) module"]
pub struct DIV_SPEC;
impl crate::RegisterSpec for DIV_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [div::R](R) reader structure"]
impl crate::Readable for DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div::W](W) writer structure"]
impl crate::Writable for DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DIV to value 0"]
impl crate::Resettable for DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
