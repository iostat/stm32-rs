///Register `SSR` reader
pub struct R(crate::R<SSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SS` reader - Sub second value
pub type SS_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - Sub second value
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0xffff) as u16)
    }
}
///sub second register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ssr](index.html) module
pub struct SSR_SPEC;
impl crate::RegisterSpec for SSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ssr::R](R) reader structure
impl crate::Readable for SSR_SPEC {
    type Reader = R;
}
///`reset()` method sets SSR to value 0
impl crate::Resettable for SSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
