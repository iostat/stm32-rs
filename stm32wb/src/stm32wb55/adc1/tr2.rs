///Register `TR2` reader
pub struct R(crate::R<TR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TR2` writer
pub struct W(crate::W<TR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR2_SPEC>;
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
impl From<crate::W<TR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LT2` reader - ADC analog watchdog 2 threshold low
pub type LT2_R = crate::FieldReader<u8, u8>;
///Field `LT2` writer - ADC analog watchdog 2 threshold low
pub type LT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR2_SPEC, u8, u8, 8, O>;
///Field `HT2` reader - ADC analog watchdog 2 threshold high
pub type HT2_R = crate::FieldReader<u8, u8>;
///Field `HT2` writer - ADC analog watchdog 2 threshold high
pub type HT2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR2_SPEC, u8, u8, 8, O>;
impl R {
    ///Bits 0:7 - ADC analog watchdog 2 threshold low
    #[inline(always)]
    pub fn lt2(&self) -> LT2_R {
        LT2_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - ADC analog watchdog 2 threshold high
    #[inline(always)]
    pub fn ht2(&self) -> HT2_R {
        HT2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    ///Bits 0:7 - ADC analog watchdog 2 threshold low
    #[inline(always)]
    #[must_use]
    pub fn lt2(&mut self) -> LT2_W<0> {
        LT2_W::new(self)
    }
    ///Bits 16:23 - ADC analog watchdog 2 threshold high
    #[inline(always)]
    #[must_use]
    pub fn ht2(&mut self) -> HT2_W<16> {
        HT2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC analog watchdog 2 threshold register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tr2](index.html) module
pub struct TR2_SPEC;
impl crate::RegisterSpec for TR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [tr2::R](R) reader structure
impl crate::Readable for TR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tr2::W](W) writer structure
impl crate::Writable for TR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TR2 to value 0x0fff_0000
impl crate::Resettable for TR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0fff_0000;
}
