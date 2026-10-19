#[doc = "Register `UNIQUEH` reader"]
pub struct R(crate::R<UNIQUEH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNIQUEH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNIQUEH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNIQUEH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNIQUEH` reader - Upper part of the 64-bit unique number"]
pub type UNIQUEH_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Upper part of the 64-bit unique number"]
    #[inline(always)]
    pub fn uniqueh(&self) -> UNIQUEH_R {
        UNIQUEH_R::new(self.bits)
    }
}
#[doc = "High 32 bits of the device unique number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uniqueh](index.html) module"]
pub struct UNIQUEH_SPEC;
impl crate::RegisterSpec for UNIQUEH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uniqueh::R](R) reader structure"]
impl crate::Readable for UNIQUEH_SPEC {
    type Reader = R;
}
