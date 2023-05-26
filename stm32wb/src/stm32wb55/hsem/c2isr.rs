///Register `C2ISR` reader
pub struct R(crate::R<C2ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `ISFm` reader - CPU(2) semaphore m status bit before enable (mask).
pub type ISFM_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - CPU(2) semaphore m status bit before enable (mask).
    #[inline(always)]
    pub fn isfm(&self) -> ISFM_R {
        ISFM_R::new(self.bits)
    }
}
///HSEM Interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2isr](index.html) module
pub struct C2ISR_SPEC;
impl crate::RegisterSpec for C2ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2isr::R](R) reader structure
impl crate::Readable for C2ISR_SPEC {
    type Reader = R;
}
///`reset()` method sets C2ISR to value 0
impl crate::Resettable for C2ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
