///Register `ISR` reader
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ISR` writer
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EOAF` reader - End of acquisition flag
pub type EOAF_R = crate::BitReader<bool>;
///Field `EOAF` writer - End of acquisition flag
pub type EOAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
///Field `MCEF` reader - Max count error flag
pub type MCEF_R = crate::BitReader<bool>;
///Field `MCEF` writer - Max count error flag
pub type MCEF_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISR_SPEC, bool, O>;
impl R {
    ///Bit 0 - End of acquisition flag
    #[inline(always)]
    pub fn eoaf(&self) -> EOAF_R {
        EOAF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Max count error flag
    #[inline(always)]
    pub fn mcef(&self) -> MCEF_R {
        MCEF_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - End of acquisition flag
    #[inline(always)]
    #[must_use]
    pub fn eoaf(&mut self) -> EOAF_W<0> {
        EOAF_W::new(self)
    }
    ///Bit 1 - Max count error flag
    #[inline(always)]
    #[must_use]
    pub fn mcef(&mut self) -> MCEF_W<1> {
        MCEF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [isr](index.html) module
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
///`read()` method returns [isr::R](R) reader structure
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [isr::W](W) writer structure
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
