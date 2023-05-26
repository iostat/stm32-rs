///Register `TR3` reader
pub struct R(crate::R<TR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR3_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TR3` writer
pub struct W(crate::W<TR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR3_SPEC>;
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
impl From<crate::W<TR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR3_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LT3` reader - ADC analog watchdog 3 threshold low
pub type LT3_R = crate::FieldReader<u8, u8>;
///Field `LT3` writer - ADC analog watchdog 3 threshold low
pub type LT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR3_SPEC, u8, u8, 8, O>;
///Field `HT3` reader - ADC analog watchdog 3 threshold high
pub type HT3_R = crate::FieldReader<u8, u8>;
///Field `HT3` writer - ADC analog watchdog 3 threshold high
pub type HT3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR3_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - ADC analog watchdog 3 threshold low
    #[inline(always)]
    pub fn lt3(&self) -> LT3_R {
        LT3_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - ADC analog watchdog 3 threshold high
    #[inline(always)]
    pub fn ht3(&self) -> HT3_R {
        HT3_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - ADC analog watchdog 3 threshold low
    #[inline(always)]
    #[must_use]
    pub fn lt3(&mut self) -> LT3_W<0> {
        LT3_W::new(self)
    }
    ///Bits 16:23 - ADC analog watchdog 3 threshold high
    #[inline(always)]
    #[must_use]
    pub fn ht3(&mut self) -> HT3_W<16> {
        HT3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC analog watchdog 3 threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tr3](index.html) module
pub struct TR3_SPEC;
impl crate::RegisterSpec for TR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [tr3::R](R) reader structure
impl crate::Readable for TR3_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tr3::W](W) writer structure
impl crate::Writable for TR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TR3 to value 0x0fff_0000
impl crate::Resettable for TR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
