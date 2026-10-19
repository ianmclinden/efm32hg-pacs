#[doc = "Register `AUXHFRCOCAL0` reader"]
pub struct R(crate::R<AUXHFRCOCAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXHFRCOCAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXHFRCOCAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXHFRCOCAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BAND1` reader - TUNING value for the 1.2 MHz AUXHFRCO band"]
pub type BAND1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BAND7` reader - TUNING value for the 6.6 MHz AUXHFRCO band"]
pub type BAND7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BAND11` reader - TUNING value for the 11 MHz AUXHFRCO band"]
pub type BAND11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BAND14` reader - TUNING value for the 14 MHz AUXHFRCO band"]
pub type BAND14_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - TUNING value for the 1.2 MHz AUXHFRCO band"]
    #[inline(always)]
    pub fn band1(&self) -> BAND1_R {
        BAND1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TUNING value for the 6.6 MHz AUXHFRCO band"]
    #[inline(always)]
    pub fn band7(&self) -> BAND7_R {
        BAND7_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TUNING value for the 11 MHz AUXHFRCO band"]
    #[inline(always)]
    pub fn band11(&self) -> BAND11_R {
        BAND11_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TUNING value for the 14 MHz AUXHFRCO band"]
    #[inline(always)]
    pub fn band14(&self) -> BAND14_R {
        BAND14_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "AUXHFRCO calibration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxhfrcocal0](index.html) module"]
pub struct AUXHFRCOCAL0_SPEC;
impl crate::RegisterSpec for AUXHFRCOCAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxhfrcocal0::R](R) reader structure"]
impl crate::Readable for AUXHFRCOCAL0_SPEC {
    type Reader = R;
}
