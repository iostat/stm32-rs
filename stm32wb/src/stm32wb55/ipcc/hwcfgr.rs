///Register `HWCFGR` reader
pub struct R(crate::R<HWCFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CHANNELS` reader - Number of channels per CPU supported by the IP, range 1 to 16
pub type CHANNELS_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:7 - Number of channels per CPU supported by the IP, range 1 to 16
    #[inline(always)]
    pub fn channels(&self) -> CHANNELS_R {
        CHANNELS_R::new((self.bits & 0xff) as u8)
    }
}
///IPCC Hardware configuration register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr](index.html) module
pub struct HWCFGR_SPEC;
impl crate::RegisterSpec for HWCFGR_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr::R](R) reader structure
impl crate::Readable for HWCFGR_SPEC {
    type Reader = R;
}
///`reset()` method sets HWCFGR to value 0x06
impl crate::Resettable for HWCFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
