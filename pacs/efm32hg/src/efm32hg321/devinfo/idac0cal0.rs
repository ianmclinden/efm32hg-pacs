#[doc = "Register `IDAC0CAL0` reader"]
pub struct R(crate::R<IDAC0CAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDAC0CAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDAC0CAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDAC0CAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RANGE0` reader - Current range 0 tuning value for IDAC0"]
pub type RANGE0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RANGE1` reader - Current range 1 tuning value for IDAC0"]
pub type RANGE1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RANGE2` reader - Current range 2 tuning value for IDAC0"]
pub type RANGE2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RANGE3` reader - Current range 3 tuning value for IDAC0"]
pub type RANGE3_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Current range 0 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range0(&self) -> RANGE0_R {
        RANGE0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Current range 1 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range1(&self) -> RANGE1_R {
        RANGE1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Current range 2 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range2(&self) -> RANGE2_R {
        RANGE2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Current range 3 tuning value for IDAC0"]
    #[inline(always)]
    pub fn range3(&self) -> RANGE3_R {
        RANGE3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "IDAC0 calibration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [idac0cal0](index.html) module"]
pub struct IDAC0CAL0_SPEC;
impl crate::RegisterSpec for IDAC0CAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [idac0cal0::R](R) reader structure"]
impl crate::Readable for IDAC0CAL0_SPEC {
    type Reader = R;
}
