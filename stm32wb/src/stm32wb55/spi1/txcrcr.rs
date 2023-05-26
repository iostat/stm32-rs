///Register `TXCRCR` reader
pub struct R(crate::R<TXCRCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXCRCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXCRCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXCRCR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TxCRC` reader - Tx CRC register
pub type TX_CRC_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Tx CRC register
    #[inline(always)]
    pub fn tx_crc(&self) -> TX_CRC_R {
        TX_CRC_R::new((self.bits & 0xffff) as u16)
    }
}
///TX CRC register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txcrcr](index.html) module
pub struct TXCRCR_SPEC;
impl crate::RegisterSpec for TXCRCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [txcrcr::R](R) reader structure
impl crate::Readable for TXCRCR_SPEC {
    type Reader = R;
}
///`reset()` method sets TXCRCR to value 0
impl crate::Resettable for TXCRCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
