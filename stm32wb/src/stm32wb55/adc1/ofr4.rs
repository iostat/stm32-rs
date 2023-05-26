///Register `OFR4` reader
pub struct R(crate::R<OFR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OFR4` writer
pub struct W(crate::W<OFR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR4_SPEC>;
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
impl From<crate::W<OFR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OFFSET4` reader - ADC offset number 4 offset level
pub type OFFSET4_R = crate::FieldReader<u16, u16>;
///Field `OFFSET4` writer - ADC offset number 4 offset level
pub type OFFSET4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR4_SPEC, u16, u16, 12, O>;
///Field `OFFSET4_CH` reader - ADC offset number 4 channel selection
pub type OFFSET4_CH_R = crate::FieldReader<u8, u8>;
///Field `OFFSET4_CH` writer - ADC offset number 4 channel selection
pub type OFFSET4_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR4_SPEC, u8, u8, 5, O>;
///Field `OFFSET4_EN` reader - ADC offset number 4 enable
pub type OFFSET4_EN_R = crate::BitReader<bool>;
///Field `OFFSET4_EN` writer - ADC offset number 4 enable
pub type OFFSET4_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR4_SPEC, bool, O>;
impl R {
    ///Bits 0:11 - ADC offset number 4 offset level
    #[inline(always)]
    pub fn offset4(&self) -> OFFSET4_R {
        OFFSET4_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 26:30 - ADC offset number 4 channel selection
    #[inline(always)]
    pub fn offset4_ch(&self) -> OFFSET4_CH_R {
        OFFSET4_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - ADC offset number 4 enable
    #[inline(always)]
    pub fn offset4_en(&self) -> OFFSET4_EN_R {
        OFFSET4_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - ADC offset number 4 offset level
    #[inline(always)]
    #[must_use]
    pub fn offset4(&mut self) -> OFFSET4_W<0> {
        OFFSET4_W::new(self)
    }
    ///Bits 26:30 - ADC offset number 4 channel selection
    #[inline(always)]
    #[must_use]
    pub fn offset4_ch(&mut self) -> OFFSET4_CH_W<26> {
        OFFSET4_CH_W::new(self)
    }
    ///Bit 31 - ADC offset number 4 enable
    #[inline(always)]
    #[must_use]
    pub fn offset4_en(&mut self) -> OFFSET4_EN_W<31> {
        OFFSET4_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC offset number 4 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ofr4](index.html) module
pub struct OFR4_SPEC;
impl crate::RegisterSpec for OFR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [ofr4::R](R) reader structure
impl crate::Readable for OFR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ofr4::W](W) writer structure
impl crate::Writable for OFR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OFR4 to value 0
impl crate::Resettable for OFR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
