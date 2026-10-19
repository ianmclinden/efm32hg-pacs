#[doc = "Register `ADC0CAL1` reader"]
pub struct R(crate::R<ADC0CAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0CAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0CAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0CAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OFFSET_VDD` reader - Offset for the VDD reference"]
pub type OFFSET_VDD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN_VDD` reader - Gain for the VDD reference"]
pub type GAIN_VDD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSET_5VDIFF` reader - Offset for the 5VDIFF reference"]
pub type OFFSET_5VDIFF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAIN_5VDIFF` reader - Gain for the 5VDIFF reference"]
pub type GAIN_5VDIFF_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:6 - Offset for the VDD reference"]
    #[inline(always)]
    pub fn offset_vdd(&self) -> OFFSET_VDD_R {
        OFFSET_VDD_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Gain for the VDD reference"]
    #[inline(always)]
    pub fn gain_vdd(&self) -> GAIN_VDD_R {
        GAIN_VDD_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Offset for the 5VDIFF reference"]
    #[inline(always)]
    pub fn offset_5vdiff(&self) -> OFFSET_5VDIFF_R {
        OFFSET_5VDIFF_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Gain for the 5VDIFF reference"]
    #[inline(always)]
    pub fn gain_5vdiff(&self) -> GAIN_5VDIFF_R {
        GAIN_5VDIFF_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "ADC0 calibration register 1 (VDD and 5VDIFF references)\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0cal1](index.html) module"]
pub struct ADC0CAL1_SPEC;
impl crate::RegisterSpec for ADC0CAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0cal1::R](R) reader structure"]
impl crate::Readable for ADC0CAL1_SPEC {
    type Reader = R;
}
