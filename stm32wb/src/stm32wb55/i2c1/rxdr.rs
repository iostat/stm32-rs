///Register `RXDR` reader
pub struct R(crate::R<RXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RXDATA` reader - 8-bit receive data
pub type RXDATA_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - 8-bit receive data
    #[inline(always)]
    pub fn rxdata(&self) -> RXDATA_R {
        RXDATA_R::new((self.bits & 0xff) as u8)
    }
}
///Receive data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxdr](index.html) module
pub struct RXDR_SPEC;
impl crate::RegisterSpec for RXDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxdr::R](R) reader structure
impl crate::Readable for RXDR_SPEC {
    type Reader = R;
}
///`reset()` method sets RXDR to value 0
impl crate::Resettable for RXDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
