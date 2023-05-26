///Register `OFR1` reader
pub struct R(crate::R<OFR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OFR1` writer
pub struct W(crate::W<OFR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR1_SPEC>;
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
impl From<crate::W<OFR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OFFSET1` reader - ADC offset number 1 offset level
pub type OFFSET1_R = crate::FieldReader<u16, u16>;
///Field `OFFSET1` writer - ADC offset number 1 offset level
pub type OFFSET1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR1_SPEC, u16, u16, 12, O>;
///Field `OFFSET1_CH` reader - ADC offset number 1 channel selection
pub type OFFSET1_CH_R = crate::FieldReader<u8, u8>;
///Field `OFFSET1_CH` writer - ADC offset number 1 channel selection
pub type OFFSET1_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR1_SPEC, u8, u8, 5, O>;
///Field `OFFSET1_EN` reader - ADC offset number 1 enable
pub type OFFSET1_EN_R = crate::BitReader<bool>;
///Field `OFFSET1_EN` writer - ADC offset number 1 enable
pub type OFFSET1_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR1_SPEC, bool, O>;
impl R {
    ///Bits 0:11 - ADC offset number 1 offset level
    #[inline(always)]
    pub fn offset1(&self) -> OFFSET1_R {
        OFFSET1_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 26:30 - ADC offset number 1 channel selection
    #[inline(always)]
    pub fn offset1_ch(&self) -> OFFSET1_CH_R {
        OFFSET1_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - ADC offset number 1 enable
    #[inline(always)]
    pub fn offset1_en(&self) -> OFFSET1_EN_R {
        OFFSET1_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - ADC offset number 1 offset level
    #[inline(always)]
    #[must_use]
    pub fn offset1(&mut self) -> OFFSET1_W<0> {
        OFFSET1_W::new(self)
    }
    ///Bits 26:30 - ADC offset number 1 channel selection
    #[inline(always)]
    #[must_use]
    pub fn offset1_ch(&mut self) -> OFFSET1_CH_W<26> {
        OFFSET1_CH_W::new(self)
    }
    ///Bit 31 - ADC offset number 1 enable
    #[inline(always)]
    #[must_use]
    pub fn offset1_en(&mut self) -> OFFSET1_EN_W<31> {
        OFFSET1_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC offset number 1 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ofr1](index.html) module
pub struct OFR1_SPEC;
impl crate::RegisterSpec for OFR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ofr1::R](R) reader structure
impl crate::Readable for OFR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ofr1::W](W) writer structure
impl crate::Writable for OFR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OFR1 to value 0
impl crate::Resettable for OFR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
