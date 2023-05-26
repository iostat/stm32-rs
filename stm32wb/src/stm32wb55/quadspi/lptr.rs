///Register `LPTR` reader
pub struct R(crate::R<LPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LPTR` writer
pub struct W(crate::W<LPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTR_SPEC>;
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
impl From<crate::W<LPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIMEOUT` reader - Timeout period
pub type TIMEOUT_R = crate::FieldReader<u16, u16>;
///Field `TIMEOUT` writer - Timeout period
pub type TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - Timeout period
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - Timeout period
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TIMEOUT_W<0> {
        TIMEOUT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///low-power timeout register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lptr](index.html) module
pub struct LPTR_SPEC;
impl crate::RegisterSpec for LPTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lptr::R](R) reader structure
impl crate::Readable for LPTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lptr::W](W) writer structure
impl crate::Writable for LPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LPTR to value 0
impl crate::Resettable for LPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
