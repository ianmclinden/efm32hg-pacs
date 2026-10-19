#[doc = "Register `ADC0CAL0` reader"]
pub struct R(crate::R<ADC0CAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0CAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0CAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0CAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFFSET_1V25` reader - Offset for the 1V25 reference"]
pub type OFFSET_1V25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN_1V25` reader - Gain for the 1V25 reference"]
pub type GAIN_1V25_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET_2V5` reader - Offset for the 2V5 reference"]
pub type OFFSET_2V5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN_2V5` reader - Gain for the 2V5 reference"]
pub type GAIN_2V5_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Offset for the 1V25 reference"]
    #[inline(always)]
    pub fn offset_1v25(&self) -> OFFSET_1V25_R {
        OFFSET_1V25_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Gain for the 1V25 reference"]
    #[inline(always)]
    pub fn gain_1v25(&self) -> GAIN_1V25_R {
        GAIN_1V25_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Offset for the 2V5 reference"]
    #[inline(always)]
    pub fn offset_2v5(&self) -> OFFSET_2V5_R {
        OFFSET_2V5_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Gain for the 2V5 reference"]
    #[inline(always)]
    pub fn gain_2v5(&self) -> GAIN_2V5_R {
        GAIN_2V5_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "ADC0 calibration register 0 (1V25 and 2V5 references)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0cal0](index.html) module"]
pub struct ADC0CAL0_SPEC;
impl crate::RegisterSpec for ADC0CAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0cal0::R](R) reader structure"]
impl crate::Readable for ADC0CAL0_SPEC {
    type Reader = R;
}
