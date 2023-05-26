///Register `KEYR` writer
pub struct W(crate::W<KEYR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYR_SPEC>;
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
impl From<crate::W<KEYR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `KEYR` writer - KEYR
pub type KEYR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEYR_SPEC, u32, u32, 32, O>;
impl W {
    ///Bits 0:31 - KEYR
    #[inline(always)]
    #[must_use]
    pub fn keyr(&mut self) -> KEYR_W<0> {
        KEYR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash key register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [keyr](index.html) module
pub struct KEYR_SPEC;
impl crate::RegisterSpec for KEYR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [keyr::W](W) writer structure
impl crate::Writable for KEYR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets KEYR to value 0
impl crate::Resettable for KEYR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
