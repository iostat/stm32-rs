///Register `DIFSEL` reader
pub struct R(crate::R<DIFSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIFSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIFSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIFSEL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DIFSEL` writer
pub struct W(crate::W<DIFSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIFSEL_SPEC>;
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
impl From<crate::W<DIFSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIFSEL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DIFSEL_0` reader - ADC channel differential or single-ended mode for channel 0
pub type DIFSEL_0_R = crate::BitReader<bool>;
///Field `DIFSEL_1_15` reader - ADC channel differential or single-ended mode for channels 1 to 15
pub type DIFSEL_1_15_R = crate::FieldReader<u16, u16>;
///Field `DIFSEL_1_15` writer - ADC channel differential or single-ended mode for channels 1 to 15
pub type DIFSEL_1_15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DIFSEL_SPEC, u16, u16, 15, O>;
///Field `DIFSEL_16_18` reader - ADC channel differential or single-ended mode for channels 18 to 16
pub type DIFSEL_16_18_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bit 0 - ADC channel differential or single-ended mode for channel 0
    #[inline(always)]
    pub fn difsel_0(&self) -> DIFSEL_0_R {
        DIFSEL_0_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:15 - ADC channel differential or single-ended mode for channels 1 to 15
    #[inline(always)]
    pub fn difsel_1_15(&self) -> DIFSEL_1_15_R {
        DIFSEL_1_15_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    ///Bits 16:18 - ADC channel differential or single-ended mode for channels 18 to 16
    #[inline(always)]
    pub fn difsel_16_18(&self) -> DIFSEL_16_18_R {
        DIFSEL_16_18_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    ///Bits 1:15 - ADC channel differential or single-ended mode for channels 1 to 15
    #[inline(always)]
    #[must_use]
    pub fn difsel_1_15(&mut self) -> DIFSEL_1_15_W<1> {
        DIFSEL_1_15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC channel differential or single-ended mode selection register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [difsel](index.html) module
pub struct DIFSEL_SPEC;
impl crate::RegisterSpec for DIFSEL_SPEC {
    type Ux = u32;
}
///`read()` method returns [difsel::R](R) reader structure
impl crate::Readable for DIFSEL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [difsel::W](W) writer structure
impl crate::Writable for DIFSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DIFSEL to value 0
impl crate::Resettable for DIFSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
