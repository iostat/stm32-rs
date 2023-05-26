///Register `IDR` reader
pub struct R(crate::R<IDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IDR` writer
pub struct W(crate::W<IDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IDR_SPEC>;
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
impl From<crate::W<IDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `IDR` reader - General-purpose 32-bit data register bits
pub type IDR_R = crate::FieldReader<u32, u32>;
///Field `IDR` writer - General-purpose 32-bit data register bits
pub type IDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IDR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - General-purpose 32-bit data register bits
    #[inline(always)]
    pub fn idr(&self) -> IDR_R {
        IDR_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - General-purpose 32-bit data register bits
    #[inline(always)]
    #[must_use]
    pub fn idr(&mut self) -> IDR_W<0> {
        IDR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Independent data register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [idr](index.html) module
pub struct IDR_SPEC;
impl crate::RegisterSpec for IDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [idr::R](R) reader structure
impl crate::Readable for IDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [idr::W](W) writer structure
impl crate::Writable for IDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IDR to value 0
impl crate::Resettable for IDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
