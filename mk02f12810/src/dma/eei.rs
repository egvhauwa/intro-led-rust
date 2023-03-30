#[doc = "Register `EEI` reader"]
pub struct R(crate::R<EEI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EEI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EEI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EEI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EEI` writer"]
pub struct W(crate::W<EEI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EEI_SPEC>;
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
impl From<crate::W<EEI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EEI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EEI0` reader - Enable Error Interrupt 0"]
pub type EEI0_R = crate::BitReader<EEI0_A>;
#[doc = "Enable Error Interrupt 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI0_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI0_A> for bool {
    #[inline(always)]
    fn from(variant: EEI0_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI0_A {
        match self.bits {
            false => EEI0_A::_0,
            true => EEI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI0_A::_1
    }
}
#[doc = "Field `EEI0` writer - Enable Error Interrupt 0"]
pub type EEI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI0_A, O>;
impl<'a, const O: u8> EEI0_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI0_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI0_A::_1)
    }
}
#[doc = "Field `EEI1` reader - Enable Error Interrupt 1"]
pub type EEI1_R = crate::BitReader<EEI1_A>;
#[doc = "Enable Error Interrupt 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI1_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI1_A> for bool {
    #[inline(always)]
    fn from(variant: EEI1_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI1_A {
        match self.bits {
            false => EEI1_A::_0,
            true => EEI1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI1_A::_1
    }
}
#[doc = "Field `EEI1` writer - Enable Error Interrupt 1"]
pub type EEI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI1_A, O>;
impl<'a, const O: u8> EEI1_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI1_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI1_A::_1)
    }
}
#[doc = "Field `EEI2` reader - Enable Error Interrupt 2"]
pub type EEI2_R = crate::BitReader<EEI2_A>;
#[doc = "Enable Error Interrupt 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI2_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI2_A> for bool {
    #[inline(always)]
    fn from(variant: EEI2_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI2_A {
        match self.bits {
            false => EEI2_A::_0,
            true => EEI2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI2_A::_1
    }
}
#[doc = "Field `EEI2` writer - Enable Error Interrupt 2"]
pub type EEI2_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI2_A, O>;
impl<'a, const O: u8> EEI2_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI2_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI2_A::_1)
    }
}
#[doc = "Field `EEI3` reader - Enable Error Interrupt 3"]
pub type EEI3_R = crate::BitReader<EEI3_A>;
#[doc = "Enable Error Interrupt 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EEI3_A {
    #[doc = "0: The error signal for corresponding channel does not generate an error interrupt"]
    _0 = 0,
    #[doc = "1: The assertion of the error signal for corresponding channel generates an error interrupt request"]
    _1 = 1,
}
impl From<EEI3_A> for bool {
    #[inline(always)]
    fn from(variant: EEI3_A) -> Self {
        variant as u8 != 0
    }
}
impl EEI3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EEI3_A {
        match self.bits {
            false => EEI3_A::_0,
            true => EEI3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EEI3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EEI3_A::_1
    }
}
#[doc = "Field `EEI3` writer - Enable Error Interrupt 3"]
pub type EEI3_W<'a, const O: u8> = crate::BitWriter<'a, u32, EEI_SPEC, EEI3_A, O>;
impl<'a, const O: u8> EEI3_W<'a, O> {
    #[doc = "The error signal for corresponding channel does not generate an error interrupt"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EEI3_A::_0)
    }
    #[doc = "The assertion of the error signal for corresponding channel generates an error interrupt request"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EEI3_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    pub fn eei0(&self) -> EEI0_R {
        EEI0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    pub fn eei1(&self) -> EEI1_R {
        EEI1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    pub fn eei2(&self) -> EEI2_R {
        EEI2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    pub fn eei3(&self) -> EEI3_R {
        EEI3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Error Interrupt 0"]
    #[inline(always)]
    #[must_use]
    pub fn eei0(&mut self) -> EEI0_W<0> {
        EEI0_W::new(self)
    }
    #[doc = "Bit 1 - Enable Error Interrupt 1"]
    #[inline(always)]
    #[must_use]
    pub fn eei1(&mut self) -> EEI1_W<1> {
        EEI1_W::new(self)
    }
    #[doc = "Bit 2 - Enable Error Interrupt 2"]
    #[inline(always)]
    #[must_use]
    pub fn eei2(&mut self) -> EEI2_W<2> {
        EEI2_W::new(self)
    }
    #[doc = "Bit 3 - Enable Error Interrupt 3"]
    #[inline(always)]
    #[must_use]
    pub fn eei3(&mut self) -> EEI3_W<3> {
        EEI3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Error Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [eei](index.html) module"]
pub struct EEI_SPEC;
impl crate::RegisterSpec for EEI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [eei::R](R) reader structure"]
impl crate::Readable for EEI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [eei::W](W) writer structure"]
impl crate::Writable for EEI_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EEI to value 0"]
impl crate::Resettable for EEI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
