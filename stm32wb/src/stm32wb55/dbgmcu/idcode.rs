///Register `IDCODE` reader
pub struct R(crate::R<IDCODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDCODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDCODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDCODE_SPEC>) -> Self {
        R(reader)
    }
}
///Field `DEV_ID` reader - Device Identifier
pub type DEV_ID_R = crate::FieldReader<u16, u16>;
///Field `REV_ID` reader - Revision Identifier
pub type REV_ID_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:11 - Device Identifier
    #[inline(always)]
    pub fn dev_id(&self) -> DEV_ID_R {
        DEV_ID_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:31 - Revision Identifier
    #[inline(always)]
    pub fn rev_id(&self) -> REV_ID_R {
        REV_ID_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
///MCU Device ID Code Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idcode](index.html) module
pub struct IDCODE_SPEC;
impl crate::RegisterSpec for IDCODE_SPEC {
    type Ux = u32;
}
///`read()` method returns [idcode::R](R) reader structure
impl crate::Readable for IDCODE_SPEC {
    type Reader = R;
}
///`reset()` method sets IDCODE to value 0
impl crate::Resettable for IDCODE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
