///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `SOF0` reader - Synchronization Overrun Flag 0
pub type SOF0_R = crate::BitReader<bool>;
///Field `SOF1` reader - Synchronization Overrun Flag 1
pub type SOF1_R = crate::BitReader<bool>;
///Field `SOF2` reader - Synchronization Overrun Flag 2
pub type SOF2_R = crate::BitReader<bool>;
///Field `SOF3` reader - Synchronization Overrun Flag 3
pub type SOF3_R = crate::BitReader<bool>;
///Field `SOF4` reader - Synchronization Overrun Flag 4
pub type SOF4_R = crate::BitReader<bool>;
///Field `SOF5` reader - Synchronization Overrun Flag 5
pub type SOF5_R = crate::BitReader<bool>;
///Field `SOF6` reader - Synchronization Overrun Flag 6
pub type SOF6_R = crate::BitReader<bool>;
///Field `SOF7` reader - Synchronization Overrun Flag 7
pub type SOF7_R = crate::BitReader<bool>;
///Field `SOF8` reader - Synchronization Overrun Flag 8
pub type SOF8_R = crate::BitReader<bool>;
///Field `SOF9` reader - Synchronization Overrun Flag 9
pub type SOF9_R = crate::BitReader<bool>;
///Field `SOF10` reader - Synchronization Overrun Flag 10
pub type SOF10_R = crate::BitReader<bool>;
///Field `SOF11` reader - Synchronization Overrun Flag 11
pub type SOF11_R = crate::BitReader<bool>;
///Field `SOF12` reader - Synchronization Overrun Flag 12
pub type SOF12_R = crate::BitReader<bool>;
///Field `SOF13` reader - Synchronization Overrun Flag 13
pub type SOF13_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Synchronization Overrun Flag 0
    #[inline(always)]
    pub fn sof0(&self) -> SOF0_R {
        SOF0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Synchronization Overrun Flag 1
    #[inline(always)]
    pub fn sof1(&self) -> SOF1_R {
        SOF1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization Overrun Flag 2
    #[inline(always)]
    pub fn sof2(&self) -> SOF2_R {
        SOF2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Synchronization Overrun Flag 3
    #[inline(always)]
    pub fn sof3(&self) -> SOF3_R {
        SOF3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Synchronization Overrun Flag 4
    #[inline(always)]
    pub fn sof4(&self) -> SOF4_R {
        SOF4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Synchronization Overrun Flag 5
    #[inline(always)]
    pub fn sof5(&self) -> SOF5_R {
        SOF5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Synchronization Overrun Flag 6
    #[inline(always)]
    pub fn sof6(&self) -> SOF6_R {
        SOF6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Synchronization Overrun Flag 7
    #[inline(always)]
    pub fn sof7(&self) -> SOF7_R {
        SOF7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Synchronization Overrun Flag 8
    #[inline(always)]
    pub fn sof8(&self) -> SOF8_R {
        SOF8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Synchronization Overrun Flag 9
    #[inline(always)]
    pub fn sof9(&self) -> SOF9_R {
        SOF9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Synchronization Overrun Flag 10
    #[inline(always)]
    pub fn sof10(&self) -> SOF10_R {
        SOF10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization Overrun Flag 11
    #[inline(always)]
    pub fn sof11(&self) -> SOF11_R {
        SOF11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Synchronization Overrun Flag 12
    #[inline(always)]
    pub fn sof12(&self) -> SOF12_R {
        SOF12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Synchronization Overrun Flag 13
    #[inline(always)]
    pub fn sof13(&self) -> SOF13_R {
        SOF13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
///DMA Multiplexer Channel Status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
