///Register `SQR2` reader
pub struct R(crate::R<SQR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SQR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SQR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SQR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SQR2` writer
pub struct W(crate::W<SQR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SQR2_SPEC>;
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
impl From<crate::W<SQR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SQR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SQ5` reader - ADC group regular sequencer rank 5
pub type SQ5_R = crate::FieldReader<u8, u8>;
///Field `SQ5` writer - ADC group regular sequencer rank 5
pub type SQ5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
///Field `SQ6` reader - ADC group regular sequencer rank 6
pub type SQ6_R = crate::FieldReader<u8, u8>;
///Field `SQ6` writer - ADC group regular sequencer rank 6
pub type SQ6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
///Field `SQ7` reader - ADC group regular sequencer rank 7
pub type SQ7_R = crate::FieldReader<u8, u8>;
///Field `SQ7` writer - ADC group regular sequencer rank 7
pub type SQ7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
///Field `SQ8` reader - ADC group regular sequencer rank 8
pub type SQ8_R = crate::FieldReader<u8, u8>;
///Field `SQ8` writer - ADC group regular sequencer rank 8
pub type SQ8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
///Field `SQ9` reader - ADC group regular sequencer rank 9
pub type SQ9_R = crate::FieldReader<u8, u8>;
///Field `SQ9` writer - ADC group regular sequencer rank 9
pub type SQ9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SQR2_SPEC, u8, u8, 5, O>;
impl R {
    ///Bits 0:4 - ADC group regular sequencer rank 5
    #[inline(always)]
    pub fn sq5(&self) -> SQ5_R {
        SQ5_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:10 - ADC group regular sequencer rank 6
    #[inline(always)]
    pub fn sq6(&self) -> SQ6_R {
        SQ6_R::new(((self.bits >> 6) & 0x1f) as u8)
    }
    ///Bits 12:16 - ADC group regular sequencer rank 7
    #[inline(always)]
    pub fn sq7(&self) -> SQ7_R {
        SQ7_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    ///Bits 18:22 - ADC group regular sequencer rank 8
    #[inline(always)]
    pub fn sq8(&self) -> SQ8_R {
        SQ8_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
    ///Bits 24:28 - ADC group regular sequencer rank 9
    #[inline(always)]
    pub fn sq9(&self) -> SQ9_R {
        SQ9_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    ///Bits 0:4 - ADC group regular sequencer rank 5
    #[inline(always)]
    #[must_use]
    pub fn sq5(&mut self) -> SQ5_W<0> {
        SQ5_W::new(self)
    }
    ///Bits 6:10 - ADC group regular sequencer rank 6
    #[inline(always)]
    #[must_use]
    pub fn sq6(&mut self) -> SQ6_W<6> {
        SQ6_W::new(self)
    }
    ///Bits 12:16 - ADC group regular sequencer rank 7
    #[inline(always)]
    #[must_use]
    pub fn sq7(&mut self) -> SQ7_W<12> {
        SQ7_W::new(self)
    }
    ///Bits 18:22 - ADC group regular sequencer rank 8
    #[inline(always)]
    #[must_use]
    pub fn sq8(&mut self) -> SQ8_W<18> {
        SQ8_W::new(self)
    }
    ///Bits 24:28 - ADC group regular sequencer rank 9
    #[inline(always)]
    #[must_use]
    pub fn sq9(&mut self) -> SQ9_W<24> {
        SQ9_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC group regular sequencer ranks register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sqr2](index.html) module
pub struct SQR2_SPEC;
impl crate::RegisterSpec for SQR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [sqr2::R](R) reader structure
impl crate::Readable for SQR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sqr2::W](W) writer structure
impl crate::Writable for SQR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SQR2 to value 0
impl crate::Resettable for SQR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
