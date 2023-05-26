///Register `ACR` reader
pub struct R(crate::R<ACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ACR` writer
pub struct W(crate::W<ACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ACR_SPEC>;
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
impl From<crate::W<ACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::FieldReader<u8, u8>;
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, ACR_SPEC, u8, u8, 3, O>;
///Field `PRFTEN` reader - Prefetch enable
pub type PRFTEN_R = crate::BitReader<bool>;
///Field `PRFTEN` writer - Prefetch enable
pub type PRFTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
///Field `ICEN` reader - Instruction cache enable
pub type ICEN_R = crate::BitReader<bool>;
///Field `ICEN` writer - Instruction cache enable
pub type ICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
///Field `DCEN` reader - Data cache enable
pub type DCEN_R = crate::BitReader<bool>;
///Field `DCEN` writer - Data cache enable
pub type DCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
///Field `ICRST` reader - Instruction cache reset
pub type ICRST_R = crate::BitReader<bool>;
///Field `ICRST` writer - Instruction cache reset
pub type ICRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
///Field `DCRST` reader - Data cache reset
pub type DCRST_R = crate::BitReader<bool>;
///Field `DCRST` writer - Data cache reset
pub type DCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
///Field `PES` reader - CPU1 CortexM4 program erase suspend request
pub type PES_R = crate::BitReader<bool>;
///Field `PES` writer - CPU1 CortexM4 program erase suspend request
pub type PES_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
///Field `EMPTY` reader - Flash User area empty
pub type EMPTY_R = crate::BitReader<bool>;
///Field `EMPTY` writer - Flash User area empty
pub type EMPTY_W<'a, const O: u8> = crate::BitWriter<'a, u32, ACR_SPEC, bool, O>;
impl R {
    ///Bits 0:2 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 7) as u8)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    pub fn dcen(&self) -> DCEN_R {
        DCEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    pub fn icrst(&self) -> ICRST_R {
        ICRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    pub fn dcrst(&self) -> DCRST_R {
        DCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - CPU1 CortexM4 program erase suspend request
    #[inline(always)]
    pub fn pes(&self) -> PES_R {
        PES_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Flash User area empty
    #[inline(always)]
    pub fn empty(&self) -> EMPTY_R {
        EMPTY_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Latency
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<0> {
        LATENCY_W::new(self)
    }
    ///Bit 8 - Prefetch enable
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<8> {
        PRFTEN_W::new(self)
    }
    ///Bit 9 - Instruction cache enable
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<9> {
        ICEN_W::new(self)
    }
    ///Bit 10 - Data cache enable
    #[inline(always)]
    #[must_use]
    pub fn dcen(&mut self) -> DCEN_W<10> {
        DCEN_W::new(self)
    }
    ///Bit 11 - Instruction cache reset
    #[inline(always)]
    #[must_use]
    pub fn icrst(&mut self) -> ICRST_W<11> {
        ICRST_W::new(self)
    }
    ///Bit 12 - Data cache reset
    #[inline(always)]
    #[must_use]
    pub fn dcrst(&mut self) -> DCRST_W<12> {
        DCRST_W::new(self)
    }
    ///Bit 15 - CPU1 CortexM4 program erase suspend request
    #[inline(always)]
    #[must_use]
    pub fn pes(&mut self) -> PES_W<15> {
        PES_W::new(self)
    }
    ///Bit 16 - Flash User area empty
    #[inline(always)]
    #[must_use]
    pub fn empty(&mut self) -> EMPTY_W<16> {
        EMPTY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Access control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [acr](index.html) module
pub struct ACR_SPEC;
impl crate::RegisterSpec for ACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [acr::R](R) reader structure
impl crate::Readable for ACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [acr::W](W) writer structure
impl crate::Writable for ACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ACR to value 0x0600
impl crate::Resettable for ACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
