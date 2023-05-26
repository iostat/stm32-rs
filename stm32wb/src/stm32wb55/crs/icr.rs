///Register `ICR` reader
pub struct R(crate::R<ICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ICR` writer
pub struct W(crate::W<ICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICR_SPEC>;
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
impl From<crate::W<ICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SYNCOKC` reader - SYNC event OK clear flag
pub type SYNCOKC_R = crate::BitReader<bool>;
///Field `SYNCOKC` writer - SYNC event OK clear flag
pub type SYNCOKC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `SYNCWARNC` reader - warning clear flag
pub type SYNCWARNC_R = crate::BitReader<bool>;
///Field `SYNCWARNC` writer - warning clear flag
pub type SYNCWARNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `ERRC` reader - Error clear flag
pub type ERRC_R = crate::BitReader<bool>;
///Field `ERRC` writer - Error clear flag
pub type ERRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
///Field `ESYNCC` reader - Expected SYNC clear flag
pub type ESYNCC_R = crate::BitReader<bool>;
///Field `ESYNCC` writer - Expected SYNC clear flag
pub type ESYNCC_W<'a, const O: u8> = crate::BitWriter<'a, u32, ICR_SPEC, bool, O>;
impl R {
    ///Bit 0 - SYNC event OK clear flag
    #[inline(always)]
    pub fn syncokc(&self) -> SYNCOKC_R {
        SYNCOKC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - warning clear flag
    #[inline(always)]
    pub fn syncwarnc(&self) -> SYNCWARNC_R {
        SYNCWARNC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Error clear flag
    #[inline(always)]
    pub fn errc(&self) -> ERRC_R {
        ERRC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Expected SYNC clear flag
    #[inline(always)]
    pub fn esyncc(&self) -> ESYNCC_R {
        ESYNCC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - SYNC event OK clear flag
    #[inline(always)]
    #[must_use]
    pub fn syncokc(&mut self) -> SYNCOKC_W<0> {
        SYNCOKC_W::new(self)
    }
    ///Bit 1 - warning clear flag
    #[inline(always)]
    #[must_use]
    pub fn syncwarnc(&mut self) -> SYNCWARNC_W<1> {
        SYNCWARNC_W::new(self)
    }
    ///Bit 2 - Error clear flag
    #[inline(always)]
    #[must_use]
    pub fn errc(&mut self) -> ERRC_W<2> {
        ERRC_W::new(self)
    }
    ///Bit 3 - Expected SYNC clear flag
    #[inline(always)]
    #[must_use]
    pub fn esyncc(&mut self) -> ESYNCC_W<3> {
        ESYNCC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CRS interrupt flag clear register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [icr](index.html) module
pub struct ICR_SPEC;
impl crate::RegisterSpec for ICR_SPEC {
    type Ux = u32;
}
///`read()` method returns [icr::R](R) reader structure
impl crate::Readable for ICR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [icr::W](W) writer structure
impl crate::Writable for ICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
