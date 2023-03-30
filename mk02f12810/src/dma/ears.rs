#[doc = "Register `EARS` reader"]
pub struct R(crate::R<EARS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EARS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EARS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EARS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EARS` writer"]
pub struct W(crate::W<EARS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EARS_SPEC>;
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
impl From<crate::W<EARS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EARS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDREQ_0` reader - Enable asynchronous DMA request in stop for channel 0."]
pub type EDREQ_0_R = crate::BitReader<EDREQ_0_A>;
#[doc = "Enable asynchronous DMA request in stop for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_0_A {
    #[doc = "0: Disable asynchronous DMA request for channel 0."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 0."]
    _1 = 1,
}
impl From<EDREQ_0_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_0_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_0_A {
        match self.bits {
            false => EDREQ_0_A::_0,
            true => EDREQ_0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_0_A::_1
    }
}
#[doc = "Field `EDREQ_0` writer - Enable asynchronous DMA request in stop for channel 0."]
pub type EDREQ_0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_0_A, O>;
impl<'a, const O: u8> EDREQ_0_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_0_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 0."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_0_A::_1)
    }
}
#[doc = "Field `EDREQ_1` reader - Enable asynchronous DMA request in stop for channel 1."]
pub type EDREQ_1_R = crate::BitReader<EDREQ_1_A>;
#[doc = "Enable asynchronous DMA request in stop for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_1_A {
    #[doc = "0: Disable asynchronous DMA request for channel 1"]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 1."]
    _1 = 1,
}
impl From<EDREQ_1_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_1_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_1_A {
        match self.bits {
            false => EDREQ_1_A::_0,
            true => EDREQ_1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_1_A::_1
    }
}
#[doc = "Field `EDREQ_1` writer - Enable asynchronous DMA request in stop for channel 1."]
pub type EDREQ_1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_1_A, O>;
impl<'a, const O: u8> EDREQ_1_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 1"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_1_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 1."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_1_A::_1)
    }
}
#[doc = "Field `EDREQ_2` reader - Enable asynchronous DMA request in stop for channel 2."]
pub type EDREQ_2_R = crate::BitReader<EDREQ_2_A>;
#[doc = "Enable asynchronous DMA request in stop for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_2_A {
    #[doc = "0: Disable asynchronous DMA request for channel 2."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 2."]
    _1 = 1,
}
impl From<EDREQ_2_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_2_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_2_A {
        match self.bits {
            false => EDREQ_2_A::_0,
            true => EDREQ_2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_2_A::_1
    }
}
#[doc = "Field `EDREQ_2` writer - Enable asynchronous DMA request in stop for channel 2."]
pub type EDREQ_2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_2_A, O>;
impl<'a, const O: u8> EDREQ_2_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 2."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_2_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 2."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_2_A::_1)
    }
}
#[doc = "Field `EDREQ_3` reader - Enable asynchronous DMA request in stop for channel 3."]
pub type EDREQ_3_R = crate::BitReader<EDREQ_3_A>;
#[doc = "Enable asynchronous DMA request in stop for channel 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EDREQ_3_A {
    #[doc = "0: Disable asynchronous DMA request for channel 3."]
    _0 = 0,
    #[doc = "1: Enable asynchronous DMA request for channel 3."]
    _1 = 1,
}
impl From<EDREQ_3_A> for bool {
    #[inline(always)]
    fn from(variant: EDREQ_3_A) -> Self {
        variant as u8 != 0
    }
}
impl EDREQ_3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDREQ_3_A {
        match self.bits {
            false => EDREQ_3_A::_0,
            true => EDREQ_3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EDREQ_3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EDREQ_3_A::_1
    }
}
#[doc = "Field `EDREQ_3` writer - Enable asynchronous DMA request in stop for channel 3."]
pub type EDREQ_3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EARS_SPEC, EDREQ_3_A, O>;
impl<'a, const O: u8> EDREQ_3_W<'a, O> {
    #[doc = "Disable asynchronous DMA request for channel 3."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EDREQ_3_A::_0)
    }
    #[doc = "Enable asynchronous DMA request for channel 3."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EDREQ_3_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop for channel 0."]
    #[inline(always)]
    pub fn edreq_0(&self) -> EDREQ_0_R {
        EDREQ_0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop for channel 1."]
    #[inline(always)]
    pub fn edreq_1(&self) -> EDREQ_1_R {
        EDREQ_1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop for channel 2."]
    #[inline(always)]
    pub fn edreq_2(&self) -> EDREQ_2_R {
        EDREQ_2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop for channel 3."]
    #[inline(always)]
    pub fn edreq_3(&self) -> EDREQ_3_R {
        EDREQ_3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable asynchronous DMA request in stop for channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_0(&mut self) -> EDREQ_0_W<0> {
        EDREQ_0_W::new(self)
    }
    #[doc = "Bit 1 - Enable asynchronous DMA request in stop for channel 1."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_1(&mut self) -> EDREQ_1_W<1> {
        EDREQ_1_W::new(self)
    }
    #[doc = "Bit 2 - Enable asynchronous DMA request in stop for channel 2."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_2(&mut self) -> EDREQ_2_W<2> {
        EDREQ_2_W::new(self)
    }
    #[doc = "Bit 3 - Enable asynchronous DMA request in stop for channel 3."]
    #[inline(always)]
    #[must_use]
    pub fn edreq_3(&mut self) -> EDREQ_3_W<3> {
        EDREQ_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Asynchronous Request in Stop Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ears](index.html) module"]
pub struct EARS_SPEC;
impl crate::RegisterSpec for EARS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ears::R](R) reader structure"]
impl crate::Readable for EARS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ears::W](W) writer structure"]
impl crate::Writable for EARS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EARS to value 0"]
impl crate::Resettable for EARS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
