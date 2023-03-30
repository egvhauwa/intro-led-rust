#[doc = "Register `SSRS0` reader"]
pub struct R(crate::R<SSRS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSRS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSRS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSRS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSRS0` writer"]
pub struct W(crate::W<SSRS0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSRS0_SPEC>;
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
impl From<crate::W<SSRS0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSRS0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWAKEUP` reader - Sticky Low Leakage Wakeup Reset"]
pub type SWAKEUP_R = crate::BitReader<SWAKEUP_A>;
#[doc = "Sticky Low Leakage Wakeup Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWAKEUP_A {
    #[doc = "0: Reset not caused by LLWU module wakeup source"]
    _0 = 0,
    #[doc = "1: Reset caused by LLWU module wakeup source"]
    _1 = 1,
}
impl From<SWAKEUP_A> for bool {
    #[inline(always)]
    fn from(variant: SWAKEUP_A) -> Self {
        variant as u8 != 0
    }
}
impl SWAKEUP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWAKEUP_A {
        match self.bits {
            false => SWAKEUP_A::_0,
            true => SWAKEUP_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWAKEUP_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWAKEUP_A::_1
    }
}
#[doc = "Field `SWAKEUP` writer - Sticky Low Leakage Wakeup Reset"]
pub type SWAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSRS0_SPEC, SWAKEUP_A, O>;
impl<'a, const O: u8> SWAKEUP_W<'a, O> {
    #[doc = "Reset not caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWAKEUP_A::_0)
    }
    #[doc = "Reset caused by LLWU module wakeup source"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWAKEUP_A::_1)
    }
}
#[doc = "Field `SLVD` reader - Sticky Low-Voltage Detect Reset"]
pub type SLVD_R = crate::BitReader<SLVD_A>;
#[doc = "Sticky Low-Voltage Detect Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLVD_A {
    #[doc = "0: Reset not caused by LVD trip or POR"]
    _0 = 0,
    #[doc = "1: Reset caused by LVD trip or POR"]
    _1 = 1,
}
impl From<SLVD_A> for bool {
    #[inline(always)]
    fn from(variant: SLVD_A) -> Self {
        variant as u8 != 0
    }
}
impl SLVD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLVD_A {
        match self.bits {
            false => SLVD_A::_0,
            true => SLVD_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLVD_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLVD_A::_1
    }
}
#[doc = "Field `SLVD` writer - Sticky Low-Voltage Detect Reset"]
pub type SLVD_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSRS0_SPEC, SLVD_A, O>;
impl<'a, const O: u8> SLVD_W<'a, O> {
    #[doc = "Reset not caused by LVD trip or POR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLVD_A::_0)
    }
    #[doc = "Reset caused by LVD trip or POR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLVD_A::_1)
    }
}
#[doc = "Field `SLOC` reader - Sticky Loss-of-Clock Reset"]
pub type SLOC_R = crate::BitReader<SLOC_A>;
#[doc = "Sticky Loss-of-Clock Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLOC_A {
    #[doc = "0: Reset not caused by a loss of external clock."]
    _0 = 0,
    #[doc = "1: Reset caused by a loss of external clock."]
    _1 = 1,
}
impl From<SLOC_A> for bool {
    #[inline(always)]
    fn from(variant: SLOC_A) -> Self {
        variant as u8 != 0
    }
}
impl SLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOC_A {
        match self.bits {
            false => SLOC_A::_0,
            true => SLOC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SLOC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SLOC_A::_1
    }
}
#[doc = "Field `SLOC` writer - Sticky Loss-of-Clock Reset"]
pub type SLOC_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSRS0_SPEC, SLOC_A, O>;
impl<'a, const O: u8> SLOC_W<'a, O> {
    #[doc = "Reset not caused by a loss of external clock."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SLOC_A::_0)
    }
    #[doc = "Reset caused by a loss of external clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SLOC_A::_1)
    }
}
#[doc = "Field `SWDOG` reader - Sticky Watchdog"]
pub type SWDOG_R = crate::BitReader<SWDOG_A>;
#[doc = "Sticky Watchdog\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWDOG_A {
    #[doc = "0: Reset not caused by watchdog timeout"]
    _0 = 0,
    #[doc = "1: Reset caused by watchdog timeout"]
    _1 = 1,
}
impl From<SWDOG_A> for bool {
    #[inline(always)]
    fn from(variant: SWDOG_A) -> Self {
        variant as u8 != 0
    }
}
impl SWDOG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWDOG_A {
        match self.bits {
            false => SWDOG_A::_0,
            true => SWDOG_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SWDOG_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SWDOG_A::_1
    }
}
#[doc = "Field `SWDOG` writer - Sticky Watchdog"]
pub type SWDOG_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSRS0_SPEC, SWDOG_A, O>;
impl<'a, const O: u8> SWDOG_W<'a, O> {
    #[doc = "Reset not caused by watchdog timeout"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SWDOG_A::_0)
    }
    #[doc = "Reset caused by watchdog timeout"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SWDOG_A::_1)
    }
}
#[doc = "Field `SPIN` reader - Sticky External Reset Pin"]
pub type SPIN_R = crate::BitReader<SPIN_A>;
#[doc = "Sticky External Reset Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPIN_A {
    #[doc = "0: Reset not caused by external reset pin"]
    _0 = 0,
    #[doc = "1: Reset caused by external reset pin"]
    _1 = 1,
}
impl From<SPIN_A> for bool {
    #[inline(always)]
    fn from(variant: SPIN_A) -> Self {
        variant as u8 != 0
    }
}
impl SPIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPIN_A {
        match self.bits {
            false => SPIN_A::_0,
            true => SPIN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPIN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPIN_A::_1
    }
}
#[doc = "Field `SPIN` writer - Sticky External Reset Pin"]
pub type SPIN_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSRS0_SPEC, SPIN_A, O>;
impl<'a, const O: u8> SPIN_W<'a, O> {
    #[doc = "Reset not caused by external reset pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPIN_A::_0)
    }
    #[doc = "Reset caused by external reset pin"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPIN_A::_1)
    }
}
#[doc = "Field `SPOR` reader - Sticky Power-On Reset"]
pub type SPOR_R = crate::BitReader<SPOR_A>;
#[doc = "Sticky Power-On Reset\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPOR_A {
    #[doc = "0: Reset not caused by POR"]
    _0 = 0,
    #[doc = "1: Reset caused by POR"]
    _1 = 1,
}
impl From<SPOR_A> for bool {
    #[inline(always)]
    fn from(variant: SPOR_A) -> Self {
        variant as u8 != 0
    }
}
impl SPOR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPOR_A {
        match self.bits {
            false => SPOR_A::_0,
            true => SPOR_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPOR_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPOR_A::_1
    }
}
#[doc = "Field `SPOR` writer - Sticky Power-On Reset"]
pub type SPOR_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSRS0_SPEC, SPOR_A, O>;
impl<'a, const O: u8> SPOR_W<'a, O> {
    #[doc = "Reset not caused by POR"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPOR_A::_0)
    }
    #[doc = "Reset caused by POR"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPOR_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Sticky Low Leakage Wakeup Reset"]
    #[inline(always)]
    pub fn swakeup(&self) -> SWAKEUP_R {
        SWAKEUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    pub fn slvd(&self) -> SLVD_R {
        SLVD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline(always)]
    pub fn sloc(&self) -> SLOC_R {
        SLOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    pub fn swdog(&self) -> SWDOG_R {
        SWDOG_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    pub fn spin(&self) -> SPIN_R {
        SPIN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    pub fn spor(&self) -> SPOR_R {
        SPOR_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Sticky Low Leakage Wakeup Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swakeup(&mut self) -> SWAKEUP_W<0> {
        SWAKEUP_W::new(self)
    }
    #[doc = "Bit 1 - Sticky Low-Voltage Detect Reset"]
    #[inline(always)]
    #[must_use]
    pub fn slvd(&mut self) -> SLVD_W<1> {
        SLVD_W::new(self)
    }
    #[doc = "Bit 2 - Sticky Loss-of-Clock Reset"]
    #[inline(always)]
    #[must_use]
    pub fn sloc(&mut self) -> SLOC_W<2> {
        SLOC_W::new(self)
    }
    #[doc = "Bit 5 - Sticky Watchdog"]
    #[inline(always)]
    #[must_use]
    pub fn swdog(&mut self) -> SWDOG_W<5> {
        SWDOG_W::new(self)
    }
    #[doc = "Bit 6 - Sticky External Reset Pin"]
    #[inline(always)]
    #[must_use]
    pub fn spin(&mut self) -> SPIN_W<6> {
        SPIN_W::new(self)
    }
    #[doc = "Bit 7 - Sticky Power-On Reset"]
    #[inline(always)]
    #[must_use]
    pub fn spor(&mut self) -> SPOR_W<7> {
        SPOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sticky System Reset Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrs0](index.html) module"]
pub struct SSRS0_SPEC;
impl crate::RegisterSpec for SSRS0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ssrs0::R](R) reader structure"]
impl crate::Readable for SSRS0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssrs0::W](W) writer structure"]
impl crate::Writable for SSRS0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSRS0 to value 0x82"]
impl crate::Resettable for SSRS0_SPEC {
    const RESET_VALUE: Self::Ux = 0x82;
}
