///Register `RXCRCR` reader
pub struct R(crate::R<RXCRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXCRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXCRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXCRCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RxCRC` reader - Rx CRC register
pub type RX_CRC_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Rx CRC register
    #[inline(always)]
    pub fn rx_crc(&self) -> RX_CRC_R {
        RX_CRC_R::new((self.bits & 0xffff) as u16)
    }
}
///RX CRC register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rxcrcr](index.html) module
pub struct RXCRCR_SPEC;
impl crate::RegisterSpec for RXCRCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rxcrcr::R](R) reader structure
impl crate::Readable for RXCRCR_SPEC {
    type Reader = R;
}
///`reset()` method sets RXCRCR to value 0
impl crate::Resettable for RXCRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
