///Register `DOUTR` reader
pub struct R(crate::R<DOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOUTR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `AES_DOUTR` reader - Data output register
pub type AES_DOUTR_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Data output register
    #[inline(always)]
    pub fn aes_doutr(&self) -> AES_DOUTR_R {
        AES_DOUTR_R::new(self.bits)
    }
}
///data output register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [doutr](index.html) module
pub struct DOUTR_SPEC;
impl crate::RegisterSpec for DOUTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [doutr::R](R) reader structure
impl crate::Readable for DOUTR_SPEC {
    type Reader = R;
}
///`reset()` method sets DOUTR to value 0
impl crate::Resettable for DOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
