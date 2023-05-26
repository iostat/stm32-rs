///Register `SIDR` reader
pub struct R(crate::R<SIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ID` reader - Size Identification code
pub type ID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Size Identification code
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
///AES size ID register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sidr](index.html) module
pub struct SIDR_SPEC;
impl crate::RegisterSpec for SIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sidr::R](R) reader structure
impl crate::Readable for SIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets SIDR to value 0x0017_0023
impl crate::Resettable for SIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0017_0023;
}
