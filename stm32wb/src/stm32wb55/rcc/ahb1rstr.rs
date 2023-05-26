///Register `AHB1RSTR` reader
pub struct R(crate::R<AHB1RSTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB1RSTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB1RSTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB1RSTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AHB1RSTR` writer
pub struct W(crate::W<AHB1RSTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB1RSTR_SPEC>;
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
impl From<crate::W<AHB1RSTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB1RSTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DMA1RST` reader - DMA1 reset
pub type DMA1RST_R = crate::BitReader<bool>;
///Field `DMA1RST` writer - DMA1 reset
pub type DMA1RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
///Field `DMA2RST` reader - DMA2 reset
pub type DMA2RST_R = crate::BitReader<bool>;
///Field `DMA2RST` writer - DMA2 reset
pub type DMA2RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
///Field `DMAMUXRST` reader - DMAMUX reset
pub type DMAMUXRST_R = crate::BitReader<bool>;
///Field `DMAMUXRST` writer - DMAMUX reset
pub type DMAMUXRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
///Field `CRCRST` reader - CRC reset
pub type CRCRST_R = crate::BitReader<bool>;
///Field `CRCRST` writer - CRC reset
pub type CRCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
///Field `TSCRST` reader - Touch Sensing Controller reset
pub type TSCRST_R = crate::BitReader<bool>;
///Field `TSCRST` writer - Touch Sensing Controller reset
pub type TSCRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, AHB1RSTR_SPEC, bool, O>;
impl R {
    ///Bit 0 - DMA1 reset
    #[inline(always)]
    pub fn dma1rst(&self) -> DMA1RST_R {
        DMA1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA2 reset
    #[inline(always)]
    pub fn dma2rst(&self) -> DMA2RST_R {
        DMA2RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - DMAMUX reset
    #[inline(always)]
    pub fn dmamuxrst(&self) -> DMAMUXRST_R {
        DMAMUXRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - Touch Sensing Controller reset
    #[inline(always)]
    pub fn tscrst(&self) -> TSCRST_R {
        TSCRST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - DMA1 reset
    #[inline(always)]
    #[must_use]
    pub fn dma1rst(&mut self) -> DMA1RST_W<0> {
        DMA1RST_W::new(self)
    }
    ///Bit 1 - DMA2 reset
    #[inline(always)]
    #[must_use]
    pub fn dma2rst(&mut self) -> DMA2RST_W<1> {
        DMA2RST_W::new(self)
    }
    ///Bit 2 - DMAMUX reset
    #[inline(always)]
    #[must_use]
    pub fn dmamuxrst(&mut self) -> DMAMUXRST_W<2> {
        DMAMUXRST_W::new(self)
    }
    ///Bit 12 - CRC reset
    #[inline(always)]
    #[must_use]
    pub fn crcrst(&mut self) -> CRCRST_W<12> {
        CRCRST_W::new(self)
    }
    ///Bit 16 - Touch Sensing Controller reset
    #[inline(always)]
    #[must_use]
    pub fn tscrst(&mut self) -> TSCRST_W<16> {
        TSCRST_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AHB1 peripheral reset register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ahb1rstr](index.html) module
pub struct AHB1RSTR_SPEC;
impl crate::RegisterSpec for AHB1RSTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ahb1rstr::R](R) reader structure
impl crate::Readable for AHB1RSTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ahb1rstr::W](W) writer structure
impl crate::Writable for AHB1RSTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AHB1RSTR to value 0
impl crate::Resettable for AHB1RSTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
