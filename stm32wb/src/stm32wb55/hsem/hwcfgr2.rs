///Register `HWCFGR2` reader
pub struct R(crate::R<HWCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
///Field `MASTERID1` reader - Hardware Configuration valid bus masters ID1
pub type MASTERID1_R = crate::FieldReader<u8, u8>;
///Field `MASTERID2` reader - Hardware Configuration valid bus masters ID2
pub type MASTERID2_R = crate::FieldReader<u8, u8>;
///Field `MASTERID3` reader - Hardware Configuration valid bus masters ID3
pub type MASTERID3_R = crate::FieldReader<u8, u8>;
///Field `MASTERID4` reader - Hardware Configuration valid bus masters ID4
pub type MASTERID4_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:3 - Hardware Configuration valid bus masters ID1
    #[inline(always)]
    pub fn masterid1(&self) -> MASTERID1_R {
        MASTERID1_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Hardware Configuration valid bus masters ID2
    #[inline(always)]
    pub fn masterid2(&self) -> MASTERID2_R {
        MASTERID2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Hardware Configuration valid bus masters ID3
    #[inline(always)]
    pub fn masterid3(&self) -> MASTERID3_R {
        MASTERID3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Hardware Configuration valid bus masters ID4
    #[inline(always)]
    pub fn masterid4(&self) -> MASTERID4_R {
        MASTERID4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
///Semaphore hardware configuration register 2
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [hwcfgr2](index.html) module
pub struct HWCFGR2_SPEC;
impl crate::RegisterSpec for HWCFGR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [hwcfgr2::R](R) reader structure
impl crate::Readable for HWCFGR2_SPEC {
    type Reader = R;
}
///`reset()` method sets HWCFGR2 to value 0x84
impl crate::Resettable for HWCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x84;
}
