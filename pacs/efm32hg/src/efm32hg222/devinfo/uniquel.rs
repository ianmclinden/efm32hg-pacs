#[doc = "Register `UNIQUEL` reader"]
pub struct R(crate::R<UNIQUEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNIQUEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNIQUEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNIQUEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `UNIQUEL` reader - Lower part of the 64-bit unique number"]
pub type UNIQUEL_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Lower part of the 64-bit unique number"]
    #[inline(always)]
    pub fn uniquel(&self) -> UNIQUEL_R {
        UNIQUEL_R::new(self.bits)
    }
}
#[doc = "Low 32 bits of the device unique number\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uniquel](index.html) module"]
pub struct UNIQUEL_SPEC;
impl crate::RegisterSpec for UNIQUEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uniquel::R](R) reader structure"]
impl crate::Readable for UNIQUEL_SPEC {
    type Reader = R;
}
