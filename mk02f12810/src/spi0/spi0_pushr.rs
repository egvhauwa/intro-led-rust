#[doc = "Register `PUSHR` reader"]
pub struct R(crate::R<SPI0_PUSHR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI0_PUSHR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI0_PUSHR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI0_PUSHR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PUSHR` writer"]
pub struct W(crate::W<SPI0_PUSHR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI0_PUSHR_SPEC>;
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
impl From<crate::W<SPI0_PUSHR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI0_PUSHR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXDATA` reader - Transmit Data"]
pub type TXDATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXDATA` writer - Transmit Data"]
pub type TXDATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI0_PUSHR_SPEC, u16, u16, 16, O>;
#[doc = "Field `PCS` reader - Select which PCS signals are to be asserted for the transfer"]
pub type PCS_R = crate::FieldReader<u8, PCS_A>;
#[doc = "Select which PCS signals are to be asserted for the transfer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PCS_A {
    #[doc = "0: Negate the PCS\\[x\\]
signal."]
    _0 = 0,
    #[doc = "1: Assert the PCS\\[x\\]
signal."]
    _1 = 1,
}
impl From<PCS_A> for u8 {
    #[inline(always)]
    fn from(variant: PCS_A) -> Self {
        variant as _
    }
}
impl PCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PCS_A> {
        match self.bits {
            0 => Some(PCS_A::_0),
            1 => Some(PCS_A::_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PCS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PCS_A::_1
    }
}
#[doc = "Field `PCS` writer - Select which PCS signals are to be asserted for the transfer"]
pub type PCS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI0_PUSHR_SPEC, u8, PCS_A, 6, O>;
impl<'a, const O: u8> PCS_W<'a, O> {
    #[doc = "Negate the PCS\\[x\\]
signal."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PCS_A::_0)
    }
    #[doc = "Assert the PCS\\[x\\]
signal."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PCS_A::_1)
    }
}
#[doc = "Field `CTCNT` reader - Clear Transfer Counter"]
pub type CTCNT_R = crate::BitReader<CTCNT_A>;
#[doc = "Clear Transfer Counter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTCNT_A {
    #[doc = "0: Do not clear the TCR\\[TCNT\\]
field."]
    _0 = 0,
    #[doc = "1: Clear the TCR\\[TCNT\\]
field."]
    _1 = 1,
}
impl From<CTCNT_A> for bool {
    #[inline(always)]
    fn from(variant: CTCNT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTCNT_A {
        match self.bits {
            false => CTCNT_A::_0,
            true => CTCNT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTCNT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTCNT_A::_1
    }
}
#[doc = "Field `CTCNT` writer - Clear Transfer Counter"]
pub type CTCNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI0_PUSHR_SPEC, CTCNT_A, O>;
impl<'a, const O: u8> CTCNT_W<'a, O> {
    #[doc = "Do not clear the TCR\\[TCNT\\]
field."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CTCNT_A::_0)
    }
    #[doc = "Clear the TCR\\[TCNT\\]
field."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CTCNT_A::_1)
    }
}
#[doc = "Field `EOQ` reader - End Of Queue"]
pub type EOQ_R = crate::BitReader<EOQ_A>;
#[doc = "End Of Queue\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOQ_A {
    #[doc = "0: The SPI data is not the last data to transfer."]
    _0 = 0,
    #[doc = "1: The SPI data is the last data to transfer."]
    _1 = 1,
}
impl From<EOQ_A> for bool {
    #[inline(always)]
    fn from(variant: EOQ_A) -> Self {
        variant as u8 != 0
    }
}
impl EOQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOQ_A {
        match self.bits {
            false => EOQ_A::_0,
            true => EOQ_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == EOQ_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == EOQ_A::_1
    }
}
#[doc = "Field `EOQ` writer - End Of Queue"]
pub type EOQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI0_PUSHR_SPEC, EOQ_A, O>;
impl<'a, const O: u8> EOQ_W<'a, O> {
    #[doc = "The SPI data is not the last data to transfer."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(EOQ_A::_0)
    }
    #[doc = "The SPI data is the last data to transfer."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(EOQ_A::_1)
    }
}
#[doc = "Field `CTAS` reader - Clock and Transfer Attributes Select"]
pub type CTAS_R = crate::FieldReader<u8, CTAS_A>;
#[doc = "Clock and Transfer Attributes Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTAS_A {
    #[doc = "0: CTAR0"]
    _000 = 0,
    #[doc = "1: CTAR1"]
    _001 = 1,
}
impl From<CTAS_A> for u8 {
    #[inline(always)]
    fn from(variant: CTAS_A) -> Self {
        variant as _
    }
}
impl CTAS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CTAS_A> {
        match self.bits {
            0 => Some(CTAS_A::_000),
            1 => Some(CTAS_A::_001),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == CTAS_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == CTAS_A::_001
    }
}
#[doc = "Field `CTAS` writer - Clock and Transfer Attributes Select"]
pub type CTAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI0_PUSHR_SPEC, u8, CTAS_A, 3, O>;
impl<'a, const O: u8> CTAS_W<'a, O> {
    #[doc = "CTAR0"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(CTAS_A::_000)
    }
    #[doc = "CTAR1"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(CTAS_A::_001)
    }
}
#[doc = "Field `CONT` reader - Continuous Peripheral Chip Select Enable"]
pub type CONT_R = crate::BitReader<CONT_A>;
#[doc = "Continuous Peripheral Chip Select Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CONT_A {
    #[doc = "0: Return PCSn signals to their inactive state between transfers."]
    _0 = 0,
    #[doc = "1: Keep PCSn signals asserted between transfers."]
    _1 = 1,
}
impl From<CONT_A> for bool {
    #[inline(always)]
    fn from(variant: CONT_A) -> Self {
        variant as u8 != 0
    }
}
impl CONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CONT_A {
        match self.bits {
            false => CONT_A::_0,
            true => CONT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CONT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CONT_A::_1
    }
}
#[doc = "Field `CONT` writer - Continuous Peripheral Chip Select Enable"]
pub type CONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SPI0_PUSHR_SPEC, CONT_A, O>;
impl<'a, const O: u8> CONT_W<'a, O> {
    #[doc = "Return PCSn signals to their inactive state between transfers."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CONT_A::_0)
    }
    #[doc = "Keep PCSn signals asserted between transfers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CONT_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    pub fn txdata(&self) -> TXDATA_R {
        TXDATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:21 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    pub fn pcs(&self) -> PCS_R {
        PCS_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - Clear Transfer Counter"]
    #[inline(always)]
    pub fn ctcnt(&self) -> CTCNT_R {
        CTCNT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End Of Queue"]
    #[inline(always)]
    pub fn eoq(&self) -> EOQ_R {
        EOQ_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Clock and Transfer Attributes Select"]
    #[inline(always)]
    pub fn ctas(&self) -> CTAS_R {
        CTAS_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Continuous Peripheral Chip Select Enable"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata(&mut self) -> TXDATA_W<0> {
        TXDATA_W::new(self)
    }
    #[doc = "Bits 16:21 - Select which PCS signals are to be asserted for the transfer"]
    #[inline(always)]
    #[must_use]
    pub fn pcs(&mut self) -> PCS_W<16> {
        PCS_W::new(self)
    }
    #[doc = "Bit 26 - Clear Transfer Counter"]
    #[inline(always)]
    #[must_use]
    pub fn ctcnt(&mut self) -> CTCNT_W<26> {
        CTCNT_W::new(self)
    }
    #[doc = "Bit 27 - End Of Queue"]
    #[inline(always)]
    #[must_use]
    pub fn eoq(&mut self) -> EOQ_W<27> {
        EOQ_W::new(self)
    }
    #[doc = "Bits 28:30 - Clock and Transfer Attributes Select"]
    #[inline(always)]
    #[must_use]
    pub fn ctas(&mut self) -> CTAS_W<28> {
        CTAS_W::new(self)
    }
    #[doc = "Bit 31 - Continuous Peripheral Chip Select Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cont(&mut self) -> CONT_W<31> {
        CONT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PUSH TX FIFO Register In Master Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi0_pushr](index.html) module"]
pub struct SPI0_PUSHR_SPEC;
impl crate::RegisterSpec for SPI0_PUSHR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi0_pushr::R](R) reader structure"]
impl crate::Readable for SPI0_PUSHR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi0_pushr::W](W) writer structure"]
impl crate::Writable for SPI0_PUSHR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PUSHR to value 0"]
impl crate::Resettable for SPI0_PUSHR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
