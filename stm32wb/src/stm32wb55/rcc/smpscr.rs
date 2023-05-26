///Register `SMPSCR` reader
pub struct R(crate::R<SMPSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPSCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SMPSCR` writer
pub struct W(crate::W<SMPSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPSCR_SPEC>;
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
impl From<crate::W<SMPSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPSCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SMPSSEL` reader - Step Down converter clock selection
pub type SMPSSEL_R = crate::FieldReader<u8, u8>;
///Field `SMPSSEL` writer - Step Down converter clock selection
pub type SMPSSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPSCR_SPEC, u8, u8, 2, O>;
///Field `SMPSDIV` reader - Step Down converter clock prescaler
pub type SMPSDIV_R = crate::FieldReader<u8, u8>;
///Field `SMPSDIV` writer - Step Down converter clock prescaler
pub type SMPSDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SMPSCR_SPEC, u8, u8, 2, O>;
///Field `SMPSSWS` reader - Step Down converter clock switch status
pub type SMPSSWS_R = crate::FieldReader<u8, u8>;
impl R {
    ///Bits 0:1 - Step Down converter clock selection
    #[inline(always)]
    pub fn smpssel(&self) -> SMPSSEL_R {
        SMPSSEL_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:5 - Step Down converter clock prescaler
    #[inline(always)]
    pub fn smpsdiv(&self) -> SMPSDIV_R {
        SMPSDIV_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:9 - Step Down converter clock switch status
    #[inline(always)]
    pub fn smpssws(&self) -> SMPSSWS_R {
        SMPSSWS_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Step Down converter clock selection
    #[inline(always)]
    #[must_use]
    pub fn smpssel(&mut self) -> SMPSSEL_W<0> {
        SMPSSEL_W::new(self)
    }
    ///Bits 4:5 - Step Down converter clock prescaler
    #[inline(always)]
    #[must_use]
    pub fn smpsdiv(&mut self) -> SMPSDIV_W<4> {
        SMPSDIV_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Step Down converter control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [smpscr](index.html) module
pub struct SMPSCR_SPEC;
impl crate::RegisterSpec for SMPSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [smpscr::R](R) reader structure
impl crate::Readable for SMPSCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [smpscr::W](W) writer structure
impl crate::Writable for SMPSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SMPSCR to value 0x0301
impl crate::Resettable for SMPSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0301;
}
