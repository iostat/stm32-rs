///Register `AR` reader
pub struct R(crate::R<AR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AR` writer
pub struct W(crate::W<AR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AR_SPEC>;
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
impl From<crate::W<AR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADDRESS` reader - Address
pub type ADDRESS_R = crate::FieldReader<u32, u32>;
///Field `ADDRESS` writer - Address
pub type ADDRESS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Address
    #[inline(always)]
    pub fn address(&self) -> ADDRESS_R {
        ADDRESS_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Address
    #[inline(always)]
    #[must_use]
    pub fn address(&mut self) -> ADDRESS_W<0> {
        ADDRESS_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///address register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ar](index.html) module
pub struct AR_SPEC;
impl crate::RegisterSpec for AR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ar::R](R) reader structure
impl crate::Readable for AR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ar::W](W) writer structure
impl crate::Writable for AR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AR to value 0
impl crate::Resettable for AR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
