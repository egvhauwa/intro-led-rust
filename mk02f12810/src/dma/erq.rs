#[doc = "Register `ERQ` reader"]
pub struct R(crate::R<ERQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ERQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ERQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ERQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ERQ` writer"]
pub struct W(crate::W<ERQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ERQ_SPEC>;
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
impl From<crate::W<ERQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ERQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERQ0` reader - Enable DMA Request 0"]
pub type ERQ0_R = crate::BitReader<ERQ0_A>;
#[doc = "Enable DMA Request 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ0_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ0_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ0_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ0_A {
        match self.bits {
            false => ERQ0_A::_0,
            true => ERQ0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ0_A::_1
    }
}
#[doc = "Field `ERQ0` writer - Enable DMA Request 0"]
pub type ERQ0_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ0_A, O>;
impl<'a, const O: u8> ERQ0_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ0_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ0_A::_1)
    }
}
#[doc = "Field `ERQ1` reader - Enable DMA Request 1"]
pub type ERQ1_R = crate::BitReader<ERQ1_A>;
#[doc = "Enable DMA Request 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ1_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ1_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ1_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ1_A {
        match self.bits {
            false => ERQ1_A::_0,
            true => ERQ1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ1_A::_1
    }
}
#[doc = "Field `ERQ1` writer - Enable DMA Request 1"]
pub type ERQ1_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ1_A, O>;
impl<'a, const O: u8> ERQ1_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ1_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ1_A::_1)
    }
}
#[doc = "Field `ERQ2` reader - Enable DMA Request 2"]
pub type ERQ2_R = crate::BitReader<ERQ2_A>;
#[doc = "Enable DMA Request 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ2_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ2_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ2_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ2_A {
        match self.bits {
            false => ERQ2_A::_0,
            true => ERQ2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ2_A::_1
    }
}
#[doc = "Field `ERQ2` writer - Enable DMA Request 2"]
pub type ERQ2_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ2_A, O>;
impl<'a, const O: u8> ERQ2_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ2_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ2_A::_1)
    }
}
#[doc = "Field `ERQ3` reader - Enable DMA Request 3"]
pub type ERQ3_R = crate::BitReader<ERQ3_A>;
#[doc = "Enable DMA Request 3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERQ3_A {
    #[doc = "0: The DMA request signal for the corresponding channel is disabled"]
    _0 = 0,
    #[doc = "1: The DMA request signal for the corresponding channel is enabled"]
    _1 = 1,
}
impl From<ERQ3_A> for bool {
    #[inline(always)]
    fn from(variant: ERQ3_A) -> Self {
        variant as u8 != 0
    }
}
impl ERQ3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERQ3_A {
        match self.bits {
            false => ERQ3_A::_0,
            true => ERQ3_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERQ3_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERQ3_A::_1
    }
}
#[doc = "Field `ERQ3` writer - Enable DMA Request 3"]
pub type ERQ3_W<'a, const O: u8> = crate::BitWriter<'a, u32, ERQ_SPEC, ERQ3_A, O>;
impl<'a, const O: u8> ERQ3_W<'a, O> {
    #[doc = "The DMA request signal for the corresponding channel is disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERQ3_A::_0)
    }
    #[doc = "The DMA request signal for the corresponding channel is enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERQ3_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    pub fn erq0(&self) -> ERQ0_R {
        ERQ0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    pub fn erq1(&self) -> ERQ1_R {
        ERQ1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    pub fn erq2(&self) -> ERQ2_R {
        ERQ2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    pub fn erq3(&self) -> ERQ3_R {
        ERQ3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable DMA Request 0"]
    #[inline(always)]
    #[must_use]
    pub fn erq0(&mut self) -> ERQ0_W<0> {
        ERQ0_W::new(self)
    }
    #[doc = "Bit 1 - Enable DMA Request 1"]
    #[inline(always)]
    #[must_use]
    pub fn erq1(&mut self) -> ERQ1_W<1> {
        ERQ1_W::new(self)
    }
    #[doc = "Bit 2 - Enable DMA Request 2"]
    #[inline(always)]
    #[must_use]
    pub fn erq2(&mut self) -> ERQ2_W<2> {
        ERQ2_W::new(self)
    }
    #[doc = "Bit 3 - Enable DMA Request 3"]
    #[inline(always)]
    #[must_use]
    pub fn erq3(&mut self) -> ERQ3_W<3> {
        ERQ3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Enable Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [erq](index.html) module"]
pub struct ERQ_SPEC;
impl crate::RegisterSpec for ERQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [erq::R](R) reader structure"]
impl crate::Readable for ERQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [erq::W](W) writer structure"]
impl crate::Writable for ERQ_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ERQ to value 0"]
impl crate::Resettable for ERQ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
