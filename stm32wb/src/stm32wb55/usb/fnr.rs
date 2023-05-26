///Register `FNR` reader
pub struct R(crate::R<FNR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FNR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FNR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FNR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `FN` reader - Frame number
pub type FN_R = crate::FieldReader<u16, u16>;
///Field `LSOF` reader - Lost SOF
pub type LSOF_R = crate::FieldReader<u8, u8>;
///Field `LCK` reader - Locked
pub type LCK_R = crate::BitReader<bool>;
///Field `RXDM` reader - Receive data - line status
pub type RXDM_R = crate::BitReader<bool>;
///Field `RXDP` reader - Receive data + line status
pub type RXDP_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:10 - Frame number
    #[inline(always)]
    pub fn fn_(&self) -> FN_R {
        FN_R::new(self.bits & 0x07ff)
    }
    ///Bits 11:12 - Lost SOF
    #[inline(always)]
    pub fn lsof(&self) -> LSOF_R {
        LSOF_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bit 13 - Locked
    #[inline(always)]
    pub fn lck(&self) -> LCK_R {
        LCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Receive data - line status
    #[inline(always)]
    pub fn rxdm(&self) -> RXDM_R {
        RXDM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Receive data + line status
    #[inline(always)]
    pub fn rxdp(&self) -> RXDP_R {
        RXDP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
///frame number register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [fnr](index.html) module
pub struct FNR_SPEC;
impl crate::RegisterSpec for FNR_SPEC {
    type Ux = u16;
}
///`read()` method returns [fnr::R](R) reader structure
impl crate::Readable for FNR_SPEC {
    type Reader = R;
}
///`reset()` method sets FNR to value 0
impl crate::Resettable for FNR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
