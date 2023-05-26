///Register `CPACR` reader
pub struct R(crate::R<CPACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CPACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CPACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CPACR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CPACR` writer
pub struct W(crate::W<CPACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CPACR_SPEC>;
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
impl From<crate::W<CPACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CPACR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CP` reader - CP
pub type CP_R = crate::FieldReader<u8, u8>;
///Field `CP` writer - CP
pub type CP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CPACR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 20:23 - CP
    #[inline(always)]
    pub fn cp(&self) -> CP_R {
        CP_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 20:23 - CP
    #[inline(always)]
    #[must_use]
    pub fn cp(&mut self) -> CP_W<20> {
        CP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Coprocessor access control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cpacr](index.html) module
pub struct CPACR_SPEC;
impl crate::RegisterSpec for CPACR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cpacr::R](R) reader structure
impl crate::Readable for CPACR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cpacr::W](W) writer structure
impl crate::Writable for CPACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CPACR to value 0
impl crate::Resettable for CPACR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
