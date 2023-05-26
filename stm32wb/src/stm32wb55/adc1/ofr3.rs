///Register `OFR3` reader
pub struct R(crate::R<OFR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OFR3` writer
pub struct W(crate::W<OFR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFR3_SPEC>;
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
impl From<crate::W<OFR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OFFSET3` reader - ADC offset number 3 offset level
pub type OFFSET3_R = crate::FieldReader<u16, u16>;
///Field `OFFSET3` writer - ADC offset number 3 offset level
pub type OFFSET3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR3_SPEC, u16, u16, 12, O>;
///Field `OFFSET3_CH` reader - ADC offset number 3 channel selection
pub type OFFSET3_CH_R = crate::FieldReader<u8, u8>;
///Field `OFFSET3_CH` writer - ADC offset number 3 channel selection
pub type OFFSET3_CH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFR3_SPEC, u8, u8, 5, O>;
///Field `OFFSET3_EN` reader - ADC offset number 3 enable
pub type OFFSET3_EN_R = crate::BitReader<bool>;
///Field `OFFSET3_EN` writer - ADC offset number 3 enable
pub type OFFSET3_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFR3_SPEC, bool, O>;
impl R {
    ///Bits 0:11 - ADC offset number 3 offset level
    #[inline(always)]
    pub fn offset3(&self) -> OFFSET3_R {
        OFFSET3_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 26:30 - ADC offset number 3 channel selection
    #[inline(always)]
    pub fn offset3_ch(&self) -> OFFSET3_CH_R {
        OFFSET3_CH_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
    ///Bit 31 - ADC offset number 3 enable
    #[inline(always)]
    pub fn offset3_en(&self) -> OFFSET3_EN_R {
        OFFSET3_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - ADC offset number 3 offset level
    #[inline(always)]
    #[must_use]
    pub fn offset3(&mut self) -> OFFSET3_W<0> {
        OFFSET3_W::new(self)
    }
    ///Bits 26:30 - ADC offset number 3 channel selection
    #[inline(always)]
    #[must_use]
    pub fn offset3_ch(&mut self) -> OFFSET3_CH_W<26> {
        OFFSET3_CH_W::new(self)
    }
    ///Bit 31 - ADC offset number 3 enable
    #[inline(always)]
    #[must_use]
    pub fn offset3_en(&mut self) -> OFFSET3_EN_W<31> {
        OFFSET3_EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC offset number 3 register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ofr3](index.html) module
pub struct OFR3_SPEC;
impl crate::RegisterSpec for OFR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [ofr3::R](R) reader structure
impl crate::Readable for OFR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ofr3::W](W) writer structure
impl crate::Writable for OFR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OFR3 to value 0
impl crate::Resettable for OFR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
