///Register `C2ICR` reader
pub struct R(crate::R<C2ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2ICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2ICR` writer
pub struct W(crate::W<C2ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2ICR_SPEC>;
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
impl From<crate::W<C2ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ISCm` reader - CPU(2) semaphore m clear bit
pub type ISCM_R = crate::FieldReader<u32, u32>;
///Field `ISCm` writer - CPU(2) semaphore m clear bit
pub type ISCM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C2ICR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CPU(2) semaphore m clear bit
    #[inline(always)]
    pub fn iscm(&self) -> ISCM_R {
        ISCM_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CPU(2) semaphore m clear bit
    #[inline(always)]
    #[must_use]
    pub fn iscm(&mut self) -> ISCM_W<0> {
        ISCM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HSEM Interrupt clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2icr](index.html) module
pub struct C2ICR_SPEC;
impl crate::RegisterSpec for C2ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2icr::R](R) reader structure
impl crate::Readable for C2ICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2icr::W](W) writer structure
impl crate::Writable for C2ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2ICR to value 0
impl crate::Resettable for C2ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
