///Register `PDCRH` reader
pub struct R(crate::R<PDCRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PDCRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PDCRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PDCRH_SPEC>) -> Self {
        R(reader)
    }
}
///Register `PDCRH` writer
pub struct W(crate::W<PDCRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PDCRH_SPEC>;
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
impl From<crate::W<PDCRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PDCRH_SPEC>) -> Self {
        W(writer)
    }
}
///Field `PD0` reader - Port H pull-down bit y (y=0..1)
pub type PD0_R = crate::BitReader<bool>;
///Field `PD0` writer - Port H pull-down bit y (y=0..1)
pub type PD0_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRH_SPEC, bool, O>;
///Field `PD1` reader - Port H pull-down bit y (y=0..1)
pub type PD1_R = crate::BitReader<bool>;
///Field `PD1` writer - Port H pull-down bit y (y=0..1)
pub type PD1_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRH_SPEC, bool, O>;
///Field `PD3` reader - Port H pull-down bit y (y=0..1)
pub type PD3_R = crate::BitReader<bool>;
///Field `PD3` writer - Port H pull-down bit y (y=0..1)
pub type PD3_W<'a, const O: u8> = crate::BitWriter<'a, u32, PDCRH_SPEC, bool, O>;
impl R {
    ///Bit 0 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    #[must_use]
    pub fn pd0(&mut self) -> PD0_W<0> {
        PD0_W::new(self)
    }
    ///Bit 1 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    #[must_use]
    pub fn pd1(&mut self) -> PD1_W<1> {
        PD1_W::new(self)
    }
    ///Bit 3 - Port H pull-down bit y (y=0..1)
    #[inline(always)]
    #[must_use]
    pub fn pd3(&mut self) -> PD3_W<3> {
        PD3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power Port H pull-down control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [pdcrh](index.html) module
pub struct PDCRH_SPEC;
impl crate::RegisterSpec for PDCRH_SPEC {
    type Ux = u32;
}
///`read()` method returns [pdcrh::R](R) reader structure
impl crate::Readable for PDCRH_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [pdcrh::W](W) writer structure
impl crate::Writable for PDCRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets PDCRH to value 0
impl crate::Resettable for PDCRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
