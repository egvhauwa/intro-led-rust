#[doc = "Register `SCGC6` reader"]
pub struct R(crate::R<SCGC6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCGC6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCGC6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCGC6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCGC6` writer"]
pub struct W(crate::W<SCGC6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCGC6_SPEC>;
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
impl From<crate::W<SCGC6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCGC6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FTF` reader - Flash Memory Clock Gate Control"]
pub type FTF_R = crate::BitReader<FTF_A>;
#[doc = "Flash Memory Clock Gate Control\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTF_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTF_A> for bool {
    #[inline(always)]
    fn from(variant: FTF_A) -> Self {
        variant as u8 != 0
    }
}
impl FTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTF_A {
        match self.bits {
            false => FTF_A::_0,
            true => FTF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTF_A::_1
    }
}
#[doc = "Field `FTF` writer - Flash Memory Clock Gate Control"]
pub type FTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, FTF_A, O>;
impl<'a, const O: u8> FTF_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTF_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTF_A::_1)
    }
}
#[doc = "Field `DMAMUX` reader - DMA Mux Clock Gate Control"]
pub type DMAMUX_R = crate::BitReader<DMAMUX_A>;
#[doc = "DMA Mux Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAMUX_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DMAMUX_A> for bool {
    #[inline(always)]
    fn from(variant: DMAMUX_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAMUX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAMUX_A {
        match self.bits {
            false => DMAMUX_A::_0,
            true => DMAMUX_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DMAMUX_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DMAMUX_A::_1
    }
}
#[doc = "Field `DMAMUX` writer - DMA Mux Clock Gate Control"]
pub type DMAMUX_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, DMAMUX_A, O>;
impl<'a, const O: u8> DMAMUX_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DMAMUX_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DMAMUX_A::_1)
    }
}
#[doc = "Field `SPI0` reader - SPI0 Clock Gate Control"]
pub type SPI0_R = crate::BitReader<SPI0_A>;
#[doc = "SPI0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPI0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<SPI0_A> for bool {
    #[inline(always)]
    fn from(variant: SPI0_A) -> Self {
        variant as u8 != 0
    }
}
impl SPI0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI0_A {
        match self.bits {
            false => SPI0_A::_0,
            true => SPI0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SPI0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SPI0_A::_1
    }
}
#[doc = "Field `SPI0` writer - SPI0 Clock Gate Control"]
pub type SPI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, SPI0_A, O>;
impl<'a, const O: u8> SPI0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SPI0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SPI0_A::_1)
    }
}
#[doc = "Field `CRC` reader - CRC Clock Gate Control"]
pub type CRC_R = crate::BitReader<CRC_A>;
#[doc = "CRC Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRC_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<CRC_A> for bool {
    #[inline(always)]
    fn from(variant: CRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRC_A {
        match self.bits {
            false => CRC_A::_0,
            true => CRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRC_A::_1
    }
}
#[doc = "Field `CRC` writer - CRC Clock Gate Control"]
pub type CRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, CRC_A, O>;
impl<'a, const O: u8> CRC_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CRC_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CRC_A::_1)
    }
}
#[doc = "Field `PDB` reader - PDB Clock Gate Control"]
pub type PDB_R = crate::BitReader<PDB_A>;
#[doc = "PDB Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PDB_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PDB_A> for bool {
    #[inline(always)]
    fn from(variant: PDB_A) -> Self {
        variant as u8 != 0
    }
}
impl PDB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PDB_A {
        match self.bits {
            false => PDB_A::_0,
            true => PDB_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PDB_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PDB_A::_1
    }
}
#[doc = "Field `PDB` writer - PDB Clock Gate Control"]
pub type PDB_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, PDB_A, O>;
impl<'a, const O: u8> PDB_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PDB_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PDB_A::_1)
    }
}
#[doc = "Field `PIT` reader - PIT Clock Gate Control"]
pub type PIT_R = crate::BitReader<PIT_A>;
#[doc = "PIT Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PIT_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<PIT_A> for bool {
    #[inline(always)]
    fn from(variant: PIT_A) -> Self {
        variant as u8 != 0
    }
}
impl PIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PIT_A {
        match self.bits {
            false => PIT_A::_0,
            true => PIT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PIT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PIT_A::_1
    }
}
#[doc = "Field `PIT` writer - PIT Clock Gate Control"]
pub type PIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, PIT_A, O>;
impl<'a, const O: u8> PIT_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PIT_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PIT_A::_1)
    }
}
#[doc = "Field `FTM0` reader - FTM0 Clock Gate Control"]
pub type FTM0_R = crate::BitReader<FTM0_A>;
#[doc = "FTM0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTM0_A> for bool {
    #[inline(always)]
    fn from(variant: FTM0_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM0_A {
        match self.bits {
            false => FTM0_A::_0,
            true => FTM0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM0_A::_1
    }
}
#[doc = "Field `FTM0` writer - FTM0 Clock Gate Control"]
pub type FTM0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, FTM0_A, O>;
impl<'a, const O: u8> FTM0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM0_A::_1)
    }
}
#[doc = "Field `FTM1` reader - FTM1 Clock Gate Control"]
pub type FTM1_R = crate::BitReader<FTM1_A>;
#[doc = "FTM1 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM1_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTM1_A> for bool {
    #[inline(always)]
    fn from(variant: FTM1_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM1_A {
        match self.bits {
            false => FTM1_A::_0,
            true => FTM1_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM1_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM1_A::_1
    }
}
#[doc = "Field `FTM1` writer - FTM1 Clock Gate Control"]
pub type FTM1_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, FTM1_A, O>;
impl<'a, const O: u8> FTM1_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM1_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM1_A::_1)
    }
}
#[doc = "Field `FTM2` reader - FTM2 Clock Gate Control"]
pub type FTM2_R = crate::BitReader<FTM2_A>;
#[doc = "FTM2 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FTM2_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<FTM2_A> for bool {
    #[inline(always)]
    fn from(variant: FTM2_A) -> Self {
        variant as u8 != 0
    }
}
impl FTM2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTM2_A {
        match self.bits {
            false => FTM2_A::_0,
            true => FTM2_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FTM2_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FTM2_A::_1
    }
}
#[doc = "Field `FTM2` writer - FTM2 Clock Gate Control"]
pub type FTM2_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, FTM2_A, O>;
impl<'a, const O: u8> FTM2_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(FTM2_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FTM2_A::_1)
    }
}
#[doc = "Field `ADC0` reader - ADC0 Clock Gate Control"]
pub type ADC0_R = crate::BitReader<ADC0_A>;
#[doc = "ADC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<ADC0_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0_A {
        match self.bits {
            false => ADC0_A::_0,
            true => ADC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ADC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ADC0_A::_1
    }
}
#[doc = "Field `ADC0` writer - ADC0 Clock Gate Control"]
pub type ADC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, ADC0_A, O>;
impl<'a, const O: u8> ADC0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ADC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ADC0_A::_1)
    }
}
#[doc = "Field `DAC0` reader - DAC0 Clock Gate Control"]
pub type DAC0_R = crate::BitReader<DAC0_A>;
#[doc = "DAC0 Clock Gate Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DAC0_A {
    #[doc = "0: Clock disabled"]
    _0 = 0,
    #[doc = "1: Clock enabled"]
    _1 = 1,
}
impl From<DAC0_A> for bool {
    #[inline(always)]
    fn from(variant: DAC0_A) -> Self {
        variant as u8 != 0
    }
}
impl DAC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DAC0_A {
        match self.bits {
            false => DAC0_A::_0,
            true => DAC0_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DAC0_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DAC0_A::_1
    }
}
#[doc = "Field `DAC0` writer - DAC0 Clock Gate Control"]
pub type DAC0_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCGC6_SPEC, DAC0_A, O>;
impl<'a, const O: u8> DAC0_W<'a, O> {
    #[doc = "Clock disabled"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DAC0_A::_0)
    }
    #[doc = "Clock enabled"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DAC0_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    pub fn ftf(&self) -> FTF_R {
        FTF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    pub fn dmamux(&self) -> DMAMUX_R {
        DMAMUX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI0 Clock Gate Control"]
    #[inline(always)]
    pub fn spi0(&self) -> SPI0_R {
        SPI0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 18 - CRC Clock Gate Control"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 22 - PDB Clock Gate Control"]
    #[inline(always)]
    pub fn pdb(&self) -> PDB_R {
        PDB_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    pub fn pit(&self) -> PIT_R {
        PIT_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - FTM0 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm0(&self) -> FTM0_R {
        FTM0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FTM1 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm1(&self) -> FTM1_R {
        FTM1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FTM2 Clock Gate Control"]
    #[inline(always)]
    pub fn ftm2(&self) -> FTM2_R {
        FTM2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    pub fn adc0(&self) -> ADC0_R {
        ADC0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline(always)]
    pub fn dac0(&self) -> DAC0_R {
        DAC0_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Memory Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftf(&mut self) -> FTF_W<0> {
        FTF_W::new(self)
    }
    #[doc = "Bit 1 - DMA Mux Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dmamux(&mut self) -> DMAMUX_W<1> {
        DMAMUX_W::new(self)
    }
    #[doc = "Bit 12 - SPI0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn spi0(&mut self) -> SPI0_W<12> {
        SPI0_W::new(self)
    }
    #[doc = "Bit 18 - CRC Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn crc(&mut self) -> CRC_W<18> {
        CRC_W::new(self)
    }
    #[doc = "Bit 22 - PDB Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn pdb(&mut self) -> PDB_W<22> {
        PDB_W::new(self)
    }
    #[doc = "Bit 23 - PIT Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn pit(&mut self) -> PIT_W<23> {
        PIT_W::new(self)
    }
    #[doc = "Bit 24 - FTM0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftm0(&mut self) -> FTM0_W<24> {
        FTM0_W::new(self)
    }
    #[doc = "Bit 25 - FTM1 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftm1(&mut self) -> FTM1_W<25> {
        FTM1_W::new(self)
    }
    #[doc = "Bit 26 - FTM2 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn ftm2(&mut self) -> FTM2_W<26> {
        FTM2_W::new(self)
    }
    #[doc = "Bit 27 - ADC0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn adc0(&mut self) -> ADC0_W<27> {
        ADC0_W::new(self)
    }
    #[doc = "Bit 31 - DAC0 Clock Gate Control"]
    #[inline(always)]
    #[must_use]
    pub fn dac0(&mut self) -> DAC0_W<31> {
        DAC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Gating Control Register 6\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scgc6](index.html) module"]
pub struct SCGC6_SPEC;
impl crate::RegisterSpec for SCGC6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scgc6::R](R) reader structure"]
impl crate::Readable for SCGC6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scgc6::W](W) writer structure"]
impl crate::Writable for SCGC6_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCGC6 to value 0x01"]
impl crate::Resettable for SCGC6_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
