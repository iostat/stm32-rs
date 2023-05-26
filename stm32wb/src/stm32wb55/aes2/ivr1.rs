///Register `IVR1` reader
pub struct R(crate::R<IVR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IVR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IVR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IVR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IVR1` writer
pub struct W(crate::W<IVR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IVR1_SPEC>;
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
impl From<crate::W<IVR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IVR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AES_IVR1` reader - Initialization Vector Register (IVR \[63:32\])
pub type AES_IVR1_R = crate::FieldReader<u32, u32>;
///Field `AES_IVR1` writer - Initialization Vector Register (IVR \[63:32\])
pub type AES_IVR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IVR1_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Initialization Vector Register (IVR \[63:32\])
    #[inline(always)]
    pub fn aes_ivr1(&self) -> AES_IVR1_R {
        AES_IVR1_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Initialization Vector Register (IVR \[63:32\])
    #[inline(always)]
    #[must_use]
    pub fn aes_ivr1(&mut self) -> AES_IVR1_W<0> {
        AES_IVR1_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///initialization vector register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ivr1](index.html) module
pub struct IVR1_SPEC;
impl crate::RegisterSpec for IVR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [ivr1::R](R) reader structure
impl crate::Readable for IVR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ivr1::W](W) writer structure
impl crate::Writable for IVR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IVR1 to value 0
impl crate::Resettable for IVR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
