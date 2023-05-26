///Register `PUCRE` reader
pub struct R(crate::R<PUCRE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PUCRE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PUCRE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PUCRE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PUCRE` writer
pub struct W(crate::W<PUCRE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PUCRE_SPEC>;
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
impl From<crate::W<PUCRE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PUCRE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PU0` reader - Port E pull-up bit y (y=0..15)
pub type PU0_R = crate::BitReader<bool>;
///Field `PU0` writer - Port E pull-up bit y (y=0..15)
pub type PU0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
///Field `PU1` reader - Port E pull-up bit y (y=0..15)
pub type PU1_R = crate::BitReader<bool>;
///Field `PU1` writer - Port E pull-up bit y (y=0..15)
pub type PU1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
///Field `PU2` reader - Port E pull-up bit y (y=0..15)
pub type PU2_R = crate::BitReader<bool>;
///Field `PU2` writer - Port E pull-up bit y (y=0..15)
pub type PU2_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
///Field `PU3` reader - Port E pull-up bit y (y=0..15)
pub type PU3_R = crate::BitReader<bool>;
///Field `PU3` writer - Port E pull-up bit y (y=0..15)
pub type PU3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
///Field `PU4` reader - Port E pull-up bit y (y=0..15)
pub type PU4_R = crate::BitReader<bool>;
///Field `PU4` writer - Port E pull-up bit y (y=0..15)
pub type PU4_W<'a, const O: u8> = crate::BitWriter<'a, u32, PUCRE_SPEC, bool, O>;
impl R {
    ///Bit 0 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<0> {
        PU0_W::new(self)
    }
    ///Bit 1 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<1> {
        PU1_W::new(self)
    }
    ///Bit 2 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<2> {
        PU2_W::new(self)
    }
    ///Bit 3 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<3> {
        PU3_W::new(self)
    }
    ///Bit 4 - Port E pull-up bit y (y=0..15)
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<4> {
        PU4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power Port E pull-up control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pucre](index.html) module
pub struct PUCRE_SPEC;
impl crate::RegisterSpec for PUCRE_SPEC {
    type Ux = u32;
}
///`read()` method returns [pucre::R](R) reader structure
impl crate::Readable for PUCRE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pucre::W](W) writer structure
impl crate::Writable for PUCRE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PUCRE to value 0
impl crate::Resettable for PUCRE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
