///Register `IPIDR` reader
pub struct R(crate::R<IPIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPIDR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `IPID` reader - Identification Code
pub type IPID_R = crate::FieldReader<u32, u32>;
impl R {
    ///Bits 0:31 - Identification Code
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
///IPCC indentification register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ipidr](index.html) module
pub struct IPIDR_SPEC;
impl crate::RegisterSpec for IPIDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ipidr::R](R) reader structure
impl crate::Readable for IPIDR_SPEC {
    type Reader = R;
}
///`reset()` method sets IPIDR to value 0x0010_0071
impl crate::Resettable for IPIDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0071;
}
