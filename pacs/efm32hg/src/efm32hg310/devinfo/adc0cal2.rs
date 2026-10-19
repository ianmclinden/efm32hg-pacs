#[doc = "Register `ADC0CAL2` reader"]
pub struct R(crate::R<ADC0CAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0CAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0CAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0CAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFFSET_2XVDDVSS` reader - Offset for the 2XVDDVSS reference (the gain cannot be calibrated)"]
pub type OFFSET_2XVDDVSS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEMP_READ_1V25` reader - Temperature sensor reading at the 1V25 reference at the calibration temperature"]
pub type TEMP_READ_1V25_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:6 - Offset for the 2XVDDVSS reference (the gain cannot be calibrated)"]
    #[inline(always)]
    pub fn offset_2xvddvss(&self) -> OFFSET_2XVDDVSS_R {
        OFFSET_2XVDDVSS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 20:31 - Temperature sensor reading at the 1V25 reference at the calibration temperature"]
    #[inline(always)]
    pub fn temp_read_1v25(&self) -> TEMP_READ_1V25_R {
        TEMP_READ_1V25_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "ADC0 calibration register 2 (2XVDDVSS reference and temperature reading)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0cal2](index.html) module"]
pub struct ADC0CAL2_SPEC;
impl crate::RegisterSpec for ADC0CAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0cal2::R](R) reader structure"]
impl crate::Readable for ADC0CAL2_SPEC {
    type Reader = R;
}
