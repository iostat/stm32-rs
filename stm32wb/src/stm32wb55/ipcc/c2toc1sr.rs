///Register `C2TOC1SR` reader
pub struct R(crate::R<C2TOC1SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2TOC1SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2TOC1SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2TOC1SR_SPEC>) -> Self {
        R(reader)
    }
}
///Field `CH1F` reader - processor 2 transmit to process 1 Receive channel 1 status flag
pub type CH1F_R = crate::BitReader<bool>;
///Field `CH2F` reader - processor 2 transmit to process 1 Receive channel 2 status flag
pub type CH2F_R = crate::BitReader<bool>;
///Field `CH3F` reader - processor 2 transmit to process 1 Receive channel 3 status flag
pub type CH3F_R = crate::BitReader<bool>;
///Field `CH4F` reader - processor 2 transmit to process 1 Receive channel 4 status flag
pub type CH4F_R = crate::BitReader<bool>;
///Field `CH5F` reader - processor 2 transmit to process 1 Receive channel 5 status flag
pub type CH5F_R = crate::BitReader<bool>;
///Field `CH6F` reader - processor 2 transmit to process 1 Receive channel 6 status flag
pub type CH6F_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - processor 2 transmit to process 1 Receive channel 1 status flag
    #[inline(always)]
    pub fn ch1f(&self) -> CH1F_R {
        CH1F_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - processor 2 transmit to process 1 Receive channel 2 status flag
    #[inline(always)]
    pub fn ch2f(&self) -> CH2F_R {
        CH2F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - processor 2 transmit to process 1 Receive channel 3 status flag
    #[inline(always)]
    pub fn ch3f(&self) -> CH3F_R {
        CH3F_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - processor 2 transmit to process 1 Receive channel 4 status flag
    #[inline(always)]
    pub fn ch4f(&self) -> CH4F_R {
        CH4F_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - processor 2 transmit to process 1 Receive channel 5 status flag
    #[inline(always)]
    pub fn ch5f(&self) -> CH5F_R {
        CH5F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - processor 2 transmit to process 1 Receive channel 6 status flag
    #[inline(always)]
    pub fn ch6f(&self) -> CH6F_R {
        CH6F_R::new(((self.bits >> 5) & 1) != 0)
    }
}
///CPU2 to CPU1 status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2toc1sr](index.html) module
pub struct C2TOC1SR_SPEC;
impl crate::RegisterSpec for C2TOC1SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2toc1sr::R](R) reader structure
impl crate::Readable for C2TOC1SR_SPEC {
    type Reader = R;
}
///`reset()` method sets C2TOC1SR to value 0
impl crate::Resettable for C2TOC1SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
