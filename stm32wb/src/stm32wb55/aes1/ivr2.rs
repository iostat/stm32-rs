///Register `IVR2` reader
pub struct R(crate::R<IVR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IVR2` writer
pub struct W(crate::W<IVR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR2_SPEC>;
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
impl From<crate::W<IVR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AES_IVR2` reader - Initialization Vector Register (IVR \[95:64\])
pub type AES_IVR2_R = crate::FieldReader<u32, u32>;
///Field `AES_IVR2` writer - Initialization Vector Register (IVR \[95:64\])
pub type AES_IVR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IVR2_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Initialization Vector Register (IVR \[95:64\])
    #[inline(always)]
    pub fn aes_ivr2(&self) -> AES_IVR2_R {
        AES_IVR2_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Initialization Vector Register (IVR \[95:64\])
    #[inline(always)]
    #[must_use]
    pub fn aes_ivr2(&mut self) -> AES_IVR2_W<0> {
        AES_IVR2_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///initialization vector register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ivr2](index.html) module
pub struct IVR2_SPEC;
impl crate::RegisterSpec for IVR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [ivr2::R](R) reader structure
impl crate::Readable for IVR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ivr2::W](W) writer structure
impl crate::Writable for IVR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IVR2 to value 0
impl crate::Resettable for IVR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
