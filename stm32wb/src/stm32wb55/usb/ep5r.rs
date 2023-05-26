///Register `EP5R` reader
pub struct R(crate::R<EP5R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP5R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP5R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP5R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `EP5R` writer
pub struct W(crate::W<EP5R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP5R_SPEC>;
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
impl From<crate::W<EP5R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP5R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EA` reader - Endpoint address
pub type EA_R = crate::FieldReader<u8, u8>;
///Field `EA` writer - Endpoint address
pub type EA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, EP5R_SPEC, u8, u8, 4, O>;
///Field `STAT_TX` reader - Status bits, for transmission transfers
pub type STAT_TX_R = crate::FieldReader<u8, u8>;
///Field `STAT_TX` writer - Status bits, for transmission transfers
pub type STAT_TX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, EP5R_SPEC, u8, u8, 2, O>;
///Field `DTOG_TX` reader - Data Toggle, for transmission transfers
pub type DTOG_TX_R = crate::BitReader<bool>;
///Field `DTOG_TX` writer - Data Toggle, for transmission transfers
pub type DTOG_TX_W<'a, const O: u8> = crate::BitWriter<'a, u16, EP5R_SPEC, bool, O>;
///Field `CTR_TX` reader - Correct Transfer for transmission
pub type CTR_TX_R = crate::BitReader<bool>;
///Field `CTR_TX` writer - Correct Transfer for transmission
pub type CTR_TX_W<'a, const O: u8> = crate::BitWriter<'a, u16, EP5R_SPEC, bool, O>;
///Field `EP_KIND` reader - Endpoint kind
pub type EP_KIND_R = crate::BitReader<bool>;
///Field `EP_KIND` writer - Endpoint kind
pub type EP_KIND_W<'a, const O: u8> = crate::BitWriter<'a, u16, EP5R_SPEC, bool, O>;
///Field `EP_TYPE` reader - Endpoint type
pub type EP_TYPE_R = crate::FieldReader<u8, u8>;
///Field `EP_TYPE` writer - Endpoint type
pub type EP_TYPE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, EP5R_SPEC, u8, u8, 2, O>;
///Field `SETUP` reader - Setup transaction completed
pub type SETUP_R = crate::BitReader<bool>;
///Field `SETUP` writer - Setup transaction completed
pub type SETUP_W<'a, const O: u8> = crate::BitWriter<'a, u16, EP5R_SPEC, bool, O>;
///Field `STAT_RX` reader - Status bits, for reception transfers
pub type STAT_RX_R = crate::FieldReader<u8, u8>;
///Field `STAT_RX` writer - Status bits, for reception transfers
pub type STAT_RX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, EP5R_SPEC, u8, u8, 2, O>;
///Field `DTOG_RX` reader - Data Toggle, for reception transfers
pub type DTOG_RX_R = crate::BitReader<bool>;
///Field `DTOG_RX` writer - Data Toggle, for reception transfers
pub type DTOG_RX_W<'a, const O: u8> = crate::BitWriter<'a, u16, EP5R_SPEC, bool, O>;
///Field `CTR_RX` reader - Correct transfer for reception
pub type CTR_RX_R = crate::BitReader<bool>;
///Field `CTR_RX` writer - Correct transfer for reception
pub type CTR_RX_W<'a, const O: u8> = crate::BitWriter<'a, u16, EP5R_SPEC, bool, O>;
impl R {
    ///Bits 0:3 - Endpoint address
    #[inline(always)]
    pub fn ea(&self) -> EA_R {
        EA_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    pub fn stat_tx(&self) -> STAT_TX_R {
        STAT_TX_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bit 6 - Data Toggle, for transmission transfers
    #[inline(always)]
    pub fn dtog_tx(&self) -> DTOG_TX_R {
        DTOG_TX_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Correct Transfer for transmission
    #[inline(always)]
    pub fn ctr_tx(&self) -> CTR_TX_R {
        CTR_TX_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Endpoint kind
    #[inline(always)]
    pub fn ep_kind(&self) -> EP_KIND_R {
        EP_KIND_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:10 - Endpoint type
    #[inline(always)]
    pub fn ep_type(&self) -> EP_TYPE_R {
        EP_TYPE_R::new(((self.bits >> 9) & 3) as u8)
    }
    ///Bit 11 - Setup transaction completed
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    pub fn stat_rx(&self) -> STAT_RX_R {
        STAT_RX_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Data Toggle, for reception transfers
    #[inline(always)]
    pub fn dtog_rx(&self) -> DTOG_RX_R {
        DTOG_RX_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Correct transfer for reception
    #[inline(always)]
    pub fn ctr_rx(&self) -> CTR_RX_R {
        CTR_RX_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Endpoint address
    #[inline(always)]
    #[must_use]
    pub fn ea(&mut self) -> EA_W<0> {
        EA_W::new(self)
    }
    ///Bits 4:5 - Status bits, for transmission transfers
    #[inline(always)]
    #[must_use]
    pub fn stat_tx(&mut self) -> STAT_TX_W<4> {
        STAT_TX_W::new(self)
    }
    ///Bit 6 - Data Toggle, for transmission transfers
    #[inline(always)]
    #[must_use]
    pub fn dtog_tx(&mut self) -> DTOG_TX_W<6> {
        DTOG_TX_W::new(self)
    }
    ///Bit 7 - Correct Transfer for transmission
    #[inline(always)]
    #[must_use]
    pub fn ctr_tx(&mut self) -> CTR_TX_W<7> {
        CTR_TX_W::new(self)
    }
    ///Bit 8 - Endpoint kind
    #[inline(always)]
    #[must_use]
    pub fn ep_kind(&mut self) -> EP_KIND_W<8> {
        EP_KIND_W::new(self)
    }
    ///Bits 9:10 - Endpoint type
    #[inline(always)]
    #[must_use]
    pub fn ep_type(&mut self) -> EP_TYPE_W<9> {
        EP_TYPE_W::new(self)
    }
    ///Bit 11 - Setup transaction completed
    #[inline(always)]
    #[must_use]
    pub fn setup(&mut self) -> SETUP_W<11> {
        SETUP_W::new(self)
    }
    ///Bits 12:13 - Status bits, for reception transfers
    #[inline(always)]
    #[must_use]
    pub fn stat_rx(&mut self) -> STAT_RX_W<12> {
        STAT_RX_W::new(self)
    }
    ///Bit 14 - Data Toggle, for reception transfers
    #[inline(always)]
    #[must_use]
    pub fn dtog_rx(&mut self) -> DTOG_RX_W<14> {
        DTOG_RX_W::new(self)
    }
    ///Bit 15 - Correct transfer for reception
    #[inline(always)]
    #[must_use]
    pub fn ctr_rx(&mut self) -> CTR_RX_W<15> {
        CTR_RX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///endpoint 5 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ep5r](index.html) module
pub struct EP5R_SPEC;
impl crate::RegisterSpec for EP5R_SPEC {
    type Ux = u16;
}
///`read()` method returns [ep5r::R](R) reader structure
impl crate::Readable for EP5R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ep5r::W](W) writer structure
impl crate::Writable for EP5R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets EP5R to value 0
impl crate::Resettable for EP5R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
