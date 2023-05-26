///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EN` reader - Enable
pub type EN_R = crate::BitReader<bool>;
///Field `EN` writer - Enable
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ABORT` reader - Abort request
pub type ABORT_R = crate::BitReader<bool>;
///Field `ABORT` writer - Abort request
pub type ABORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DMAEN` reader - DMA enable
pub type DMAEN_R = crate::BitReader<bool>;
///Field `DMAEN` writer - DMA enable
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TCEN` reader - Timeout counter enable
pub type TCEN_R = crate::BitReader<bool>;
///Field `TCEN` writer - Timeout counter enable
pub type TCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `SSHIFT` reader - Sample shift
pub type SSHIFT_R = crate::BitReader<bool>;
///Field `SSHIFT` writer - Sample shift
pub type SSHIFT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `FTHRES` reader - FIFO threshold level
pub type FTHRES_R = crate::FieldReader<u8, u8>;
///Field `FTHRES` writer - FIFO threshold level
pub type FTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 4, O>;
///Field `TEIE` reader - Transfer error interrupt enable
pub type TEIE_R = crate::BitReader<bool>;
///Field `TEIE` writer - Transfer error interrupt enable
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TCIE` reader - Transfer complete interrupt enable
pub type TCIE_R = crate::BitReader<bool>;
///Field `TCIE` writer - Transfer complete interrupt enable
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `FTIE` reader - FIFO threshold interrupt enable
pub type FTIE_R = crate::BitReader<bool>;
///Field `FTIE` writer - FIFO threshold interrupt enable
pub type FTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `SMIE` reader - Status match interrupt enable
pub type SMIE_R = crate::BitReader<bool>;
///Field `SMIE` writer - Status match interrupt enable
pub type SMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TOIE` reader - TimeOut interrupt enable
pub type TOIE_R = crate::BitReader<bool>;
///Field `TOIE` writer - TimeOut interrupt enable
pub type TOIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `APMS` reader - Automatic poll mode stop
pub type APMS_R = crate::BitReader<bool>;
///Field `APMS` writer - Automatic poll mode stop
pub type APMS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `PMM` reader - Polling match mode
pub type PMM_R = crate::BitReader<bool>;
///Field `PMM` writer - Polling match mode
pub type PMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `PRESCALER` reader - Clock prescaler
pub type PRESCALER_R = crate::FieldReader<u8, u8>;
///Field `PRESCALER` writer - Clock prescaler
pub type PRESCALER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 8, O>;
impl R {
    ///Bit 0 - Enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Abort request
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMA enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timeout counter enable
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Sample shift
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - FIFO threshold level
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 16 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - FIFO threshold interrupt enable
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Status match interrupt enable
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - TimeOut interrupt enable
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 22 - Automatic poll mode stop
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Polling match mode
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:31 - Clock prescaler
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    ///Bit 0 - Enable
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    ///Bit 1 - Abort request
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<1> {
        ABORT_W::new(self)
    }
    ///Bit 2 - DMA enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<2> {
        DMAEN_W::new(self)
    }
    ///Bit 3 - Timeout counter enable
    #[inline(always)]
    #[must_use]
    pub fn tcen(&mut self) -> TCEN_W<3> {
        TCEN_W::new(self)
    }
    ///Bit 4 - Sample shift
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<4> {
        SSHIFT_W::new(self)
    }
    ///Bits 8:11 - FIFO threshold level
    #[inline(always)]
    #[must_use]
    pub fn fthres(&mut self) -> FTHRES_W<8> {
        FTHRES_W::new(self)
    }
    ///Bit 16 - Transfer error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<16> {
        TEIE_W::new(self)
    }
    ///Bit 17 - Transfer complete interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<17> {
        TCIE_W::new(self)
    }
    ///Bit 18 - FIFO threshold interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ftie(&mut self) -> FTIE_W<18> {
        FTIE_W::new(self)
    }
    ///Bit 19 - Status match interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn smie(&mut self) -> SMIE_W<19> {
        SMIE_W::new(self)
    }
    ///Bit 20 - TimeOut interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<20> {
        TOIE_W::new(self)
    }
    ///Bit 22 - Automatic poll mode stop
    #[inline(always)]
    #[must_use]
    pub fn apms(&mut self) -> APMS_W<22> {
        APMS_W::new(self)
    }
    ///Bit 23 - Polling match mode
    #[inline(always)]
    #[must_use]
    pub fn pmm(&mut self) -> PMM_W<23> {
        PMM_W::new(self)
    }
    ///Bits 24:31 - Clock prescaler
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<24> {
        PRESCALER_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
