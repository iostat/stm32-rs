///Register `DIER` reader
pub struct R(crate::R<DIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIER` writer
pub struct W(crate::W<DIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIER_SPEC>;
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
impl From<crate::W<DIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UIE` reader - Update interrupt enable.
pub type UIE_R = crate::BitReader<bool>;
///Field `UIE` writer - Update interrupt enable.
pub type UIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
///Field `CC1IE` reader - Capture/Compare 1 interrupt enable.
pub type CC1IE_R = crate::BitReader<bool>;
///Field `CC1IE` writer - Capture/Compare 1 interrupt enable.
pub type CC1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
///Field `COMIE` reader - COM interrupt enable.
pub type COMIE_R = crate::BitReader<bool>;
///Field `COMIE` writer - COM interrupt enable.
pub type COMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
///Field `BIE` reader - Break interrupt enable.
pub type BIE_R = crate::BitReader<bool>;
///Field `BIE` writer - Break interrupt enable.
pub type BIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
///Field `UDE` reader - Update DMA request enable.
pub type UDE_R = crate::BitReader<bool>;
///Field `UDE` writer - Update DMA request enable.
pub type UDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
///Field `CC1DE` reader - Capture/Compare 1 DMA request enable.
pub type CC1DE_R = crate::BitReader<bool>;
///Field `CC1DE` writer - Capture/Compare 1 DMA request enable.
pub type CC1DE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DIER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Update interrupt enable.
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable.
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - COM interrupt enable.
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Break interrupt enable.
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Update DMA request enable.
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable.
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Update interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<0> {
        UIE_W::new(self)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CC1IE_W<1> {
        CC1IE_W::new(self)
    }
    ///Bit 5 - COM interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn comie(&mut self) -> COMIE_W<5> {
        COMIE_W::new(self)
    }
    ///Bit 7 - Break interrupt enable.
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<7> {
        BIE_W::new(self)
    }
    ///Bit 8 - Update DMA request enable.
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UDE_W<8> {
        UDE_W::new(self)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable.
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CC1DE_W<9> {
        CC1DE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA/Interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dier](index.html) module
pub struct DIER_SPEC;
impl crate::RegisterSpec for DIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [dier::R](R) reader structure
impl crate::Readable for DIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dier::W](W) writer structure
impl crate::Writable for DIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}