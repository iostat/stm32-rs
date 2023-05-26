///Register `SUSP7R` reader
pub struct R(crate::R<SUSP7R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUSP7R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUSP7R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUSP7R_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SUSP7R` writer
pub struct W(crate::W<SUSP7R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUSP7R_SPEC>;
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
impl From<crate::W<SUSP7R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUSP7R_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AES_SUSP7R` reader - AES suspend register 7
pub type AES_SUSP7R_R = crate::FieldReader<u32, u32>;
///Field `AES_SUSP7R` writer - AES suspend register 7
pub type AES_SUSP7R_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SUSP7R_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - AES suspend register 7
    #[inline(always)]
    pub fn aes_susp7r(&self) -> AES_SUSP7R_R {
        AES_SUSP7R_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - AES suspend register 7
    #[inline(always)]
    #[must_use]
    pub fn aes_susp7r(&mut self) -> AES_SUSP7R_W<0> {
        AES_SUSP7R_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AES suspend register 7
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [susp7r](index.html) module
pub struct SUSP7R_SPEC;
impl crate::RegisterSpec for SUSP7R_SPEC {
    type Ux = u32;
}
///`read()` method returns [susp7r::R](R) reader structure
impl crate::Readable for SUSP7R_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [susp7r::W](W) writer structure
impl crate::Writable for SUSP7R_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SUSP7R to value 0
impl crate::Resettable for SUSP7R_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
