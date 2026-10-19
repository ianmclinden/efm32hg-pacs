#[doc = "Register `AUXHFRCOCAL1` reader"]
pub struct R(crate::R<AUXHFRCOCAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXHFRCOCAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXHFRCOCAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXHFRCOCAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BAND21` reader - TUNING value for the 21 MHz AUXHFRCO band"]
pub type BAND21_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - TUNING value for the 21 MHz AUXHFRCO band"]
    #[inline(always)]
    pub fn band21(&self) -> BAND21_R {
        BAND21_R::new(self.bits)
    }
}
#[doc = "AUXHFRCO calibration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxhfrcocal1](index.html) module"]
pub struct AUXHFRCOCAL1_SPEC;
impl crate::RegisterSpec for AUXHFRCOCAL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [auxhfrcocal1::R](R) reader structure"]
impl crate::Readable for AUXHFRCOCAL1_SPEC {
    type Reader = R;
}
