///Register `RG0CR` reader
pub struct R(crate::R<RG0CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RG0CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RG0CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RG0CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `RG0CR` writer
pub struct W(crate::W<RG0CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RG0CR_SPEC>;
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
impl From<crate::W<RG0CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RG0CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SIG_ID` reader - Signal ID
pub type SIG_ID_R = crate::FieldReader<u8, u8>;
///Field `SIG_ID` writer - Signal ID
pub type SIG_ID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG0CR_SPEC, u8, u8, 5, O>;
///Field `OIE` reader - Overrun Interrupt Enable
pub type OIE_R = crate::BitReader<bool>;
///Field `OIE` writer - Overrun Interrupt Enable
pub type OIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RG0CR_SPEC, bool, O>;
///Field `GE` reader - Generation Enable
pub type GE_R = crate::BitReader<bool>;
///Field `GE` writer - Generation Enable
pub type GE_W<'a, const O: u8> = crate::BitWriter<'a, u32, RG0CR_SPEC, bool, O>;
///Field `GPOL` reader - Generation Polarity
pub type GPOL_R = crate::FieldReader<u8, u8>;
///Field `GPOL` writer - Generation Polarity
pub type GPOL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG0CR_SPEC, u8, u8, 2, O>;
///Field `GNBREQ` reader - Number of Request
pub type GNBREQ_R = crate::FieldReader<u8, u8>;
///Field `GNBREQ` writer - Number of Request
pub type GNBREQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RG0CR_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - Signal ID
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 8 - Overrun Interrupt Enable
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 16 - Generation Enable
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:18 - Generation Polarity
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    ///Bits 19:23 - Number of Request
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - Signal ID
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<0> {
        SIG_ID_W::new(self)
    }
    ///Bit 8 - Overrun Interrupt Enable
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<8> {
        OIE_W::new(self)
    }
    ///Bit 16 - Generation Enable
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<16> {
        GE_W::new(self)
    }
    ///Bits 17:18 - Generation Polarity
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<17> {
        GPOL_W::new(self)
    }
    ///Bits 19:23 - Number of Request
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<19> {
        GNBREQ_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///DMA Request Generator 0 Control Register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [rg0cr](index.html) module
pub struct RG0CR_SPEC;
impl crate::RegisterSpec for RG0CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [rg0cr::R](R) reader structure
impl crate::Readable for RG0CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [rg0cr::W](W) writer structure
impl crate::Writable for RG0CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets RG0CR to value 0
impl crate::Resettable for RG0CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
