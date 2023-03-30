#[doc = "Register `CH%sS` reader"]
pub struct R(crate::R<CHS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%sS` writer"]
pub struct W(crate::W<CHS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHS_SPEC>;
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
impl From<crate::W<CHS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ERR` reader - PDB Channel Sequence Error Flags"]
pub type ERR_R = crate::FieldReader<u8, ERR_A>;
#[doc = "PDB Channel Sequence Error Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ERR_A {
    #[doc = "0: Sequence error not detected on PDB channel's corresponding pre-trigger."]
    _0 = 0,
    #[doc = "1: Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    _1 = 1,
}
impl From<ERR_A> for u8 {
    #[inline(always)]
    fn from(variant: ERR_A) -> Self {
        variant as _
    }
}
impl ERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ERR_A> {
        match self.bits {
            0 => Some(ERR_A::_0),
            1 => Some(ERR_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERR_A::_1
    }
}
#[doc = "Field `ERR` writer - PDB Channel Sequence Error Flags"]
pub type ERR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHS_SPEC, u8, ERR_A, 8, O>;
impl<'a, const O: u8> ERR_W<'a, O> {
    #[doc = "Sequence error not detected on PDB channel's corresponding pre-trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ERR_A::_0)
    }
    #[doc = "Sequence error detected on PDB channel's corresponding pre-trigger. ADCn block can be triggered for a conversion by one pre-trigger from PDB channel n. When one conversion, which is triggered by one of the pre-triggers from PDB channel n, is in progress, new trigger from PDB channel's corresponding pre-trigger m cannot be accepted by ADCn, and ERR\\[m\\]
is set. Writing 0's to clear the sequence error flags."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ERR_A::_1)
    }
}
#[doc = "Field `CF` reader - PDB Channel Flags"]
pub type CF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CF` writer - PDB Channel Flags"]
pub type CF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHS_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - PDB Channel Flags"]
    #[inline(always)]
    pub fn cf(&self) -> CF_R {
        CF_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - PDB Channel Sequence Error Flags"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<0> {
        ERR_W::new(self)
    }
    #[doc = "Bits 16:23 - PDB Channel Flags"]
    #[inline(always)]
    #[must_use]
    pub fn cf(&mut self) -> CF_W<16> {
        CF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel n Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chs](index.html) module"]
pub struct CHS_SPEC;
impl crate::RegisterSpec for CHS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chs::R](R) reader structure"]
impl crate::Readable for CHS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chs::W](W) writer structure"]
impl crate::Writable for CHS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%sS to value 0"]
impl crate::Resettable for CHS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
