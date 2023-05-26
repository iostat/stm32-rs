///Register `CR3` reader
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR3` writer
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EIE` reader - Error interrupt enable
pub type EIE_R = crate::BitReader<bool>;
///Field `EIE` writer - Error interrupt enable
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `IREN` reader - Ir mode enable
pub type IREN_R = crate::BitReader<bool>;
///Field `IREN` writer - Ir mode enable
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `IRLP` reader - Ir low-power
pub type IRLP_R = crate::BitReader<bool>;
///Field `IRLP` writer - Ir low-power
pub type IRLP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `HDSEL` reader - Half-duplex selection
pub type HDSEL_R = crate::BitReader<bool>;
///Field `HDSEL` writer - Half-duplex selection
pub type HDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `NACK` reader - Smartcard NACK enable
pub type NACK_R = crate::BitReader<bool>;
///Field `NACK` writer - Smartcard NACK enable
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `SCEN` reader - Smartcard mode enable
pub type SCEN_R = crate::BitReader<bool>;
///Field `SCEN` writer - Smartcard mode enable
pub type SCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `DMAR` reader - DMA enable receiver
pub type DMAR_R = crate::BitReader<bool>;
///Field `DMAR` writer - DMA enable receiver
pub type DMAR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `DMAT` reader - DMA enable transmitter
pub type DMAT_R = crate::BitReader<bool>;
///Field `DMAT` writer - DMA enable transmitter
pub type DMAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `RTSE` reader - RTS enable
pub type RTSE_R = crate::BitReader<bool>;
///Field `RTSE` writer - RTS enable
pub type RTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `CTSE` reader - CTS enable
pub type CTSE_R = crate::BitReader<bool>;
///Field `CTSE` writer - CTS enable
pub type CTSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `CTSIE` reader - CTS interrupt enable
pub type CTSIE_R = crate::BitReader<bool>;
///Field `CTSIE` writer - CTS interrupt enable
pub type CTSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `ONEBIT` reader - One sample bit method enable
pub type ONEBIT_R = crate::BitReader<bool>;
///Field `ONEBIT` writer - One sample bit method enable
pub type ONEBIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `OVRDIS` reader - Overrun Disable
pub type OVRDIS_R = crate::BitReader<bool>;
///Field `OVRDIS` writer - Overrun Disable
pub type OVRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `DDRE` reader - DMA Disable on Reception Error
pub type DDRE_R = crate::BitReader<bool>;
///Field `DDRE` writer - DMA Disable on Reception Error
pub type DDRE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `DEM` reader - Driver enable mode
pub type DEM_R = crate::BitReader<bool>;
///Field `DEM` writer - Driver enable mode
pub type DEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `DEP` reader - Driver enable polarity selection
pub type DEP_R = crate::BitReader<bool>;
///Field `DEP` writer - Driver enable polarity selection
pub type DEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `SCARCNT` reader - Smartcard auto-retry count
pub type SCARCNT_R = crate::FieldReader<u8, u8>;
///Field `SCARCNT` writer - Smartcard auto-retry count
pub type SCARCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 3, O>;
///Field `WUS` reader - Wakeup from Stop mode interrupt flag selection
pub type WUS_R = crate::FieldReader<u8, u8>;
///Field `WUS` writer - Wakeup from Stop mode interrupt flag selection
pub type WUS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 2, O>;
///Field `WUFIE` reader - Wakeup from Stop mode interrupt enable
pub type WUFIE_R = crate::BitReader<bool>;
///Field `WUFIE` writer - Wakeup from Stop mode interrupt enable
pub type WUFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `TXFTIE` reader - threshold interrupt enable
pub type TXFTIE_R = crate::BitReader<bool>;
///Field `TXFTIE` writer - threshold interrupt enable
pub type TXFTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `TCBGTIE` reader - Tr Complete before guard time, interrupt enable
pub type TCBGTIE_R = crate::BitReader<bool>;
///Field `TCBGTIE` writer - Tr Complete before guard time, interrupt enable
pub type TCBGTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `RXFTCFG` reader - Receive FIFO threshold configuration
pub type RXFTCFG_R = crate::FieldReader<u8, u8>;
///Field `RXFTCFG` writer - Receive FIFO threshold configuration
pub type RXFTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 3, O>;
///Field `RXFTIE` reader - RXFIFO threshold interrupt enable
pub type RXFTIE_R = crate::BitReader<bool>;
///Field `RXFTIE` writer - RXFIFO threshold interrupt enable
pub type RXFTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR3_SPEC, bool, O>;
///Field `TXFTCFG` reader - TXFIFO threshold configuration
pub type TXFTCFG_R = crate::FieldReader<u8, u8>;
///Field `TXFTCFG` writer - TXFIFO threshold configuration
pub type TXFTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR3_SPEC, u8, u8, 3, O>;
impl R {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Ir mode enable
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Ir low-power
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 7) as u8)
    }
    ///Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - Wakeup from Stop mode interrupt enable
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - threshold interrupt enable
    #[inline(always)]
    pub fn txftie(&self) -> TXFTIE_R {
        TXFTIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Tr Complete before guard time, interrupt enable
    #[inline(always)]
    pub fn tcbgtie(&self) -> TCBGTIE_R {
        TCBGTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration
    #[inline(always)]
    pub fn rxftcfg(&self) -> RXFTCFG_R {
        RXFTCFG_R::new(((self.bits >> 25) & 7) as u8)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable
    #[inline(always)]
    pub fn rxftie(&self) -> RXFTIE_R {
        RXFTIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bits 29:31 - TXFIFO threshold configuration
    #[inline(always)]
    pub fn txftcfg(&self) -> TXFTCFG_R {
        TXFTCFG_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bit 0 - Error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<0> {
        EIE_W::new(self)
    }
    ///Bit 1 - Ir mode enable
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<1> {
        IREN_W::new(self)
    }
    ///Bit 2 - Ir low-power
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IRLP_W<2> {
        IRLP_W::new(self)
    }
    ///Bit 3 - Half-duplex selection
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<3> {
        HDSEL_W::new(self)
    }
    ///Bit 4 - Smartcard NACK enable
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<4> {
        NACK_W::new(self)
    }
    ///Bit 5 - Smartcard mode enable
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<5> {
        SCEN_W::new(self)
    }
    ///Bit 6 - DMA enable receiver
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DMAR_W<6> {
        DMAR_W::new(self)
    }
    ///Bit 7 - DMA enable transmitter
    #[inline(always)]
    #[must_use]
    pub fn dmat(&mut self) -> DMAT_W<7> {
        DMAT_W::new(self)
    }
    ///Bit 8 - RTS enable
    #[inline(always)]
    #[must_use]
    pub fn rtse(&mut self) -> RTSE_W<8> {
        RTSE_W::new(self)
    }
    ///Bit 9 - CTS enable
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CTSE_W<9> {
        CTSE_W::new(self)
    }
    ///Bit 10 - CTS interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CTSIE_W<10> {
        CTSIE_W::new(self)
    }
    ///Bit 11 - One sample bit method enable
    #[inline(always)]
    #[must_use]
    pub fn onebit(&mut self) -> ONEBIT_W<11> {
        ONEBIT_W::new(self)
    }
    ///Bit 12 - Overrun Disable
    #[inline(always)]
    #[must_use]
    pub fn ovrdis(&mut self) -> OVRDIS_W<12> {
        OVRDIS_W::new(self)
    }
    ///Bit 13 - DMA Disable on Reception Error
    #[inline(always)]
    #[must_use]
    pub fn ddre(&mut self) -> DDRE_W<13> {
        DDRE_W::new(self)
    }
    ///Bit 14 - Driver enable mode
    #[inline(always)]
    #[must_use]
    pub fn dem(&mut self) -> DEM_W<14> {
        DEM_W::new(self)
    }
    ///Bit 15 - Driver enable polarity selection
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DEP_W<15> {
        DEP_W::new(self)
    }
    ///Bits 17:19 - Smartcard auto-retry count
    #[inline(always)]
    #[must_use]
    pub fn scarcnt(&mut self) -> SCARCNT_W<17> {
        SCARCNT_W::new(self)
    }
    ///Bits 20:21 - Wakeup from Stop mode interrupt flag selection
    #[inline(always)]
    #[must_use]
    pub fn wus(&mut self) -> WUS_W<20> {
        WUS_W::new(self)
    }
    ///Bit 22 - Wakeup from Stop mode interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn wufie(&mut self) -> WUFIE_W<22> {
        WUFIE_W::new(self)
    }
    ///Bit 23 - threshold interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn txftie(&mut self) -> TXFTIE_W<23> {
        TXFTIE_W::new(self)
    }
    ///Bit 24 - Tr Complete before guard time, interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tcbgtie(&mut self) -> TCBGTIE_W<24> {
        TCBGTIE_W::new(self)
    }
    ///Bits 25:27 - Receive FIFO threshold configuration
    #[inline(always)]
    #[must_use]
    pub fn rxftcfg(&mut self) -> RXFTCFG_W<25> {
        RXFTCFG_W::new(self)
    }
    ///Bit 28 - RXFIFO threshold interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn rxftie(&mut self) -> RXFTIE_W<28> {
        RXFTIE_W::new(self)
    }
    ///Bits 29:31 - TXFIFO threshold configuration
    #[inline(always)]
    #[must_use]
    pub fn txftcfg(&mut self) -> TXFTCFG_W<29> {
        TXFTCFG_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 3
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr3](index.html) module
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr3::R](R) reader structure
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr3::W](W) writer structure
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR3 to value 0
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
