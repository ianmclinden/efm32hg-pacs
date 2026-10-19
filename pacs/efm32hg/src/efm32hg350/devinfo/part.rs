#[doc = "Register `PART` reader"]
pub struct R(crate::R<PART_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PART_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PART_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PART_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PART_NUMBER` reader - EFM32 part number, for example 309"]
pub type PART_NUMBER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DEVICE_FAMILY` reader - EFM32 part family number, 77 for Happy Gecko"]
pub type DEVICE_FAMILY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PROD_REV` reader - Production revision"]
pub type PROD_REV_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - EFM32 part number, for example 309"]
    #[inline(always)]
    pub fn part_number(&self) -> PART_NUMBER_R {
        PART_NUMBER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - EFM32 part family number, 77 for Happy Gecko"]
    #[inline(always)]
    pub fn device_family(&self) -> DEVICE_FAMILY_R {
        DEVICE_FAMILY_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Production revision"]
    #[inline(always)]
    pub fn prod_rev(&self) -> PROD_REV_R {
        PROD_REV_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Part description\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [part](index.html) module"]
pub struct PART_SPEC;
impl crate::RegisterSpec for PART_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [part::R](R) reader structure"]
impl crate::Readable for PART_SPEC {
    type Reader = R;
}
