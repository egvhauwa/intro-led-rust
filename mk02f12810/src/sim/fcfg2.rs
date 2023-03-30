#[doc = "Register `FCFG2` reader"]
pub struct R(crate::R<FCFG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FCFG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FCFG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FCFG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAXADDR0` reader - Max address block 0"]
pub type MAXADDR0_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 24:30 - Max address block 0"]
    #[inline(always)]
    pub fn maxaddr0(&self) -> MAXADDR0_R {
        MAXADDR0_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "Flash Configuration Register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fcfg2](index.html) module"]
pub struct FCFG2_SPEC;
impl crate::RegisterSpec for FCFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fcfg2::R](R) reader structure"]
impl crate::Readable for FCFG2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FCFG2 to value 0xf900_007f"]
impl crate::Resettable for FCFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0xf900_007f;
}
