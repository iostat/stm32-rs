///Register `HWCFGR4` reader
pub struct R(crate::R<HWCFGR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `EVENT_TRG` reader - HW configuration event trigger type
pub type EVENT_TRG_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - HW configuration event trigger type
    #[inline(always)]
    pub fn event_trg(&self) -> EVENT_TRG_R {
        EVENT_TRG_R::new(self.bits)
    }
}
///Hardware configuration registers
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr4](index.html) module
pub struct HWCFGR4_SPEC;
impl crate::RegisterSpec for HWCFGR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr4::R](R) reader structure
impl crate::Readable for HWCFGR4_SPEC {
    type Reader = R;
}
///`reset()` method sets HWCFGR4 to value 0
impl crate::Resettable for HWCFGR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
