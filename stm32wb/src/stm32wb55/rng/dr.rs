///Register `DR` reader
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `RNDATA` reader - Random data
pub type RNDATA_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Random data
    #[inline(always)]
    pub fn rndata(&self) -> RNDATA_R {
        RNDATA_R::new(self.bits)
    }
}
///data register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](index.html) module
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dr::R](R) reader structure
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
///`reset()` method sets DR to value 0
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
