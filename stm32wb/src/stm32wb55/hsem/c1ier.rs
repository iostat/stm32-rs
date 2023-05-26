///Register `C1IER` reader
pub struct R(crate::R<C1IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C1IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C1IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C1IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C1IER` writer
pub struct W(crate::W<C1IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C1IER_SPEC>;
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
impl From<crate::W<C1IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C1IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ISEm` reader - CPU(n) semaphore m enable bit
pub type ISEM_R = crate::FieldReader<u32, u32>;
///Field `ISEm` writer - CPU(n) semaphore m enable bit
pub type ISEM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, C1IER_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - CPU(n) semaphore m enable bit
    #[inline(always)]
    pub fn isem(&self) -> ISEM_R {
        ISEM_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - CPU(n) semaphore m enable bit
    #[inline(always)]
    #[must_use]
    pub fn isem(&mut self) -> ISEM_W<0> {
        ISEM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///HSEM Interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c1ier](index.html) module
pub struct C1IER_SPEC;
impl crate::RegisterSpec for C1IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [c1ier::R](R) reader structure
impl crate::Readable for C1IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c1ier::W](W) writer structure
impl crate::Writable for C1IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C1IER to value 0
impl crate::Resettable for C1IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
