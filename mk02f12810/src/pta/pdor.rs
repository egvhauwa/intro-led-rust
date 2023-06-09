#[doc = "Register `PDOR` reader"]
pub struct R(crate::R<PDOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PDOR` writer"]
pub struct W(crate::W<PDOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDOR_SPEC>;
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
impl From<crate::W<PDOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PDO` reader - Port Data Output"]
pub type PDO_R = crate::FieldReader<u32, PDO_A>;
#[doc = "Port Data Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PDO_A {
    #[doc = "0: Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    _0 = 0,
    #[doc = "1: Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    _1 = 1,
}
impl From<PDO_A> for u32 {
    #[inline(always)]
    fn from(variant: PDO_A) -> Self {
        variant as _
    }
}
impl PDO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PDO_A> {
        match self.bits {
            0 => Some(PDO_A::_0),
            1 => Some(PDO_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDO_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDO_A::_1
    }
}
#[doc = "Field `PDO` writer - Port Data Output"]
pub type PDO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PDOR_SPEC, u32, PDO_A, 32, O>;
impl<'a, const O: u8> PDO_W<'a, O> {
    #[doc = "Logic level 0 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDO_A::_0)
    }
    #[doc = "Logic level 1 is driven on pin, provided pin is configured for general-purpose output."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDO_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline(always)]
    pub fn pdo(&self) -> PDO_R {
        PDO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Port Data Output"]
    #[inline(always)]
    #[must_use]
    pub fn pdo(&mut self) -> PDO_W<0> {
        PDO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Data Output Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pdor](index.html) module"]
pub struct PDOR_SPEC;
impl crate::RegisterSpec for PDOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pdor::R](R) reader structure"]
impl crate::Readable for PDOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pdor::W](W) writer structure"]
impl crate::Writable for PDOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PDOR to value 0"]
impl crate::Resettable for PDOR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
