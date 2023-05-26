///Register `VERR` reader
pub struct R(crate::R<VERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VERR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MINREV` reader - Minor Revision
pub type MINREV_R = crate::FieldReader<u8, u8>;
///Field `MAJREV` reader - Major Revision
pub type MAJREV_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - Minor Revision
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Major Revision
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
///HSEM version register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [verr](index.html) module
pub struct VERR_SPEC;
impl crate::RegisterSpec for VERR_SPEC {
    type Ux = u32;
}
///`read()` method returns [verr::R](R) reader structure
impl crate::Readable for VERR_SPEC {
    type Reader = R;
}
///`reset()` method sets VERR to value 0x20
impl crate::Resettable for VERR_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
