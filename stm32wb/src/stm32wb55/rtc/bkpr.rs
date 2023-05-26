///Register `BKP%sR` reader
pub struct R(crate::R<BKPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BKPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BKPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BKPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BKP%sR` writer
pub struct W(crate::W<BKPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BKPR_SPEC>;
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
impl From<crate::W<BKPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BKPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BKP` reader - BKP
pub type BKP_R = crate::FieldReader<u32, u32>;
///Field `BKP` writer - BKP
pub type BKP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BKPR_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - BKP
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - BKP
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<0> {
        BKP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///backup register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bkpr](index.html) module
pub struct BKPR_SPEC;
impl crate::RegisterSpec for BKPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bkpr::R](R) reader structure
impl crate::Readable for BKPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bkpr::W](W) writer structure
impl crate::Writable for BKPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BKP%sR to value 0
impl crate::Resettable for BKPR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
