///Register `OTYPER` reader
pub struct R(crate::R<OTYPER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTYPER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTYPER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTYPER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OTYPER` writer
pub struct W(crate::W<OTYPER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTYPER_SPEC>;
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
impl From<crate::W<OTYPER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTYPER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OT0` reader - Port x configuration bits (y = 0..15)
pub type OT0_R = crate::BitReader<bool>;
///Field `OT0` writer - Port x configuration bits (y = 0..15)
pub type OT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
///Field `OT1` reader - Port x configuration bits (y = 0..15)
pub type OT1_R = crate::BitReader<bool>;
///Field `OT1` writer - Port x configuration bits (y = 0..15)
pub type OT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
///Field `OT3` reader - Port x configuration bits (y = 0..15)
pub type OT3_R = crate::BitReader<bool>;
///Field `OT3` writer - Port x configuration bits (y = 0..15)
pub type OT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTYPER_SPEC, bool, O>;
impl R {
    ///Bit 0 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot0(&self) -> OT0_R {
        OT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot1(&self) -> OT1_R {
        OT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot0(&mut self) -> OT0_W<0> {
        OT0_W::new(self)
    }
    ///Bit 1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot1(&mut self) -> OT1_W<1> {
        OT1_W::new(self)
    }
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ot3(&mut self) -> OT3_W<3> {
        OT3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port output type register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [otyper](index.html) module
pub struct OTYPER_SPEC;
impl crate::RegisterSpec for OTYPER_SPEC {
    type Ux = u32;
}
///`read()` method returns [otyper::R](R) reader structure
impl crate::Readable for OTYPER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [otyper::W](W) writer structure
impl crate::Writable for OTYPER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OTYPER to value 0
impl crate::Resettable for OTYPER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
