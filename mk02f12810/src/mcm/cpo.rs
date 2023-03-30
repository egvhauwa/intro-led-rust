#[doc = "Register `CPO` reader"]
pub struct R(crate::R<CPO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CPO` writer"]
pub struct W(crate::W<CPO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPO_SPEC>;
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
impl From<crate::W<CPO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CPOREQ` reader - Compute Operation request"]
pub type CPOREQ_R = crate::BitReader<CPOREQ_A>;
#[doc = "Compute Operation request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOREQ_A {
    #[doc = "0: Request is cleared."]
    _0 = 0,
    #[doc = "1: Request Compute Operation."]
    _1 = 1,
}
impl From<CPOREQ_A> for bool {
    #[inline(always)]
    fn from(variant: CPOREQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOREQ_A {
        match self.bits {
            false => CPOREQ_A::_0,
            true => CPOREQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOREQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOREQ_A::_1
    }
}
#[doc = "Field `CPOREQ` writer - Compute Operation request"]
pub type CPOREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPO_SPEC, CPOREQ_A, O>;
impl<'a, const O: u8> CPOREQ_W<'a, O> {
    #[doc = "Request is cleared."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOREQ_A::_0)
    }
    #[doc = "Request Compute Operation."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOREQ_A::_1)
    }
}
#[doc = "Field `CPOACK` reader - Compute Operation acknowledge"]
pub type CPOACK_R = crate::BitReader<CPOACK_A>;
#[doc = "Compute Operation acknowledge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOACK_A {
    #[doc = "0: Compute operation entry has not completed or compute operation exit has completed."]
    _0 = 0,
    #[doc = "1: Compute operation entry has completed or compute operation exit has not completed."]
    _1 = 1,
}
impl From<CPOACK_A> for bool {
    #[inline(always)]
    fn from(variant: CPOACK_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOACK_A {
        match self.bits {
            false => CPOACK_A::_0,
            true => CPOACK_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOACK_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOACK_A::_1
    }
}
#[doc = "Field `CPOWOI` reader - Compute Operation wakeup on interrupt"]
pub type CPOWOI_R = crate::BitReader<CPOWOI_A>;
#[doc = "Compute Operation wakeup on interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOWOI_A {
    #[doc = "0: No effect."]
    _0 = 0,
    #[doc = "1: When set, the CPOREQ is cleared on any interrupt or exception vector fetch."]
    _1 = 1,
}
impl From<CPOWOI_A> for bool {
    #[inline(always)]
    fn from(variant: CPOWOI_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOWOI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOWOI_A {
        match self.bits {
            false => CPOWOI_A::_0,
            true => CPOWOI_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CPOWOI_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CPOWOI_A::_1
    }
}
#[doc = "Field `CPOWOI` writer - Compute Operation wakeup on interrupt"]
pub type CPOWOI_W<'a, const O: u8> = crate::BitWriter<'a, u32, CPO_SPEC, CPOWOI_A, O>;
impl<'a, const O: u8> CPOWOI_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CPOWOI_A::_0)
    }
    #[doc = "When set, the CPOREQ is cleared on any interrupt or exception vector fetch."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CPOWOI_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compute Operation request"]
    #[inline(always)]
    pub fn cporeq(&self) -> CPOREQ_R {
        CPOREQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compute Operation acknowledge"]
    #[inline(always)]
    pub fn cpoack(&self) -> CPOACK_R {
        CPOACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compute Operation wakeup on interrupt"]
    #[inline(always)]
    pub fn cpowoi(&self) -> CPOWOI_R {
        CPOWOI_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compute Operation request"]
    #[inline(always)]
    #[must_use]
    pub fn cporeq(&mut self) -> CPOREQ_W<0> {
        CPOREQ_W::new(self)
    }
    #[doc = "Bit 2 - Compute Operation wakeup on interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn cpowoi(&mut self) -> CPOWOI_W<2> {
        CPOWOI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Compute Operation Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cpo](index.html) module"]
pub struct CPO_SPEC;
impl crate::RegisterSpec for CPO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cpo::R](R) reader structure"]
impl crate::Readable for CPO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cpo::W](W) writer structure"]
impl crate::Writable for CPO_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CPO to value 0"]
impl crate::Resettable for CPO_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
