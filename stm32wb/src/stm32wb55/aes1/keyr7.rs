///Register `KEYR7` reader
pub struct R(crate::R<KEYR7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYR7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYR7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYR7_SPEC>) -> Self {
        R(reader)
    }
}
///Register `KEYR7` writer
pub struct W(crate::W<KEYR7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR7_SPEC>;
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
impl From<crate::W<KEYR7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR7_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AES_KEYR7` reader - AES key register (MSB key \[255:224\])
pub type AES_KEYR7_R = crate::FieldReader<u32, u32>;
///Field `AES_KEYR7` writer - AES key register (MSB key \[255:224\])
pub type AES_KEYR7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYR7_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - AES key register (MSB key \[255:224\])
    #[inline(always)]
    pub fn aes_keyr7(&self) -> AES_KEYR7_R {
        AES_KEYR7_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - AES key register (MSB key \[255:224\])
    #[inline(always)]
    #[must_use]
    pub fn aes_keyr7(&mut self) -> AES_KEYR7_W<0> {
        AES_KEYR7_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///key register 7
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [keyr7](index.html) module
pub struct KEYR7_SPEC;
impl crate::RegisterSpec for KEYR7_SPEC {
    type Ux = u32;
}
///`read()` method returns [keyr7::R](R) reader structure
impl crate::Readable for KEYR7_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [keyr7::W](W) writer structure
impl crate::Writable for KEYR7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets KEYR7 to value 0
impl crate::Resettable for KEYR7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
