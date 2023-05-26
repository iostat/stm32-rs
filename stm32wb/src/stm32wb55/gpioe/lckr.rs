///Register `LCKR` reader
pub struct R(crate::R<LCKR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCKR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCKR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCKR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `LCKR` writer
pub struct W(crate::W<LCKR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCKR_SPEC>;
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
impl From<crate::W<LCKR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCKR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LCK0` reader - Port x lock bit y (y= 0..15)
pub type LCK0_R = crate::BitReader<bool>;
///Field `LCK0` writer - Port x lock bit y (y= 0..15)
pub type LCK0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, O>;
///Field `LCK1` reader - Port x lock bit y (y= 0..15)
pub type LCK1_R = crate::BitReader<bool>;
///Field `LCK1` writer - Port x lock bit y (y= 0..15)
pub type LCK1_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, O>;
///Field `LCK2` reader - Port x lock bit y (y= 0..15)
pub type LCK2_R = crate::BitReader<bool>;
///Field `LCK2` writer - Port x lock bit y (y= 0..15)
pub type LCK2_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, O>;
///Field `LCK3` reader - Port x lock bit y (y= 0..15)
pub type LCK3_R = crate::BitReader<bool>;
///Field `LCK3` writer - Port x lock bit y (y= 0..15)
pub type LCK3_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, O>;
///Field `LCK4` reader - Port x lock bit y (y= 0..15)
pub type LCK4_R = crate::BitReader<bool>;
///Field `LCK4` writer - Port x lock bit y (y= 0..15)
pub type LCK4_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, O>;
///Field `LCKK` reader - Port x lock bit y (y= 0..15)
pub type LCKK_R = crate::BitReader<bool>;
///Field `LCKK` writer - Port x lock bit y (y= 0..15)
pub type LCKK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LCKR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck0(&self) -> LCK0_R {
        LCK0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck1(&self) -> LCK1_R {
        LCK1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck2(&self) -> LCK2_R {
        LCK2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck3(&self) -> LCK3_R {
        LCK3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lck4(&self) -> LCK4_R {
        LCK4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    pub fn lckk(&self) -> LCKK_R {
        LCKK_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    #[must_use]
    pub fn lck0(&mut self) -> LCK0_W<0> {
        LCK0_W::new(self)
    }
    ///Bit 1 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    #[must_use]
    pub fn lck1(&mut self) -> LCK1_W<1> {
        LCK1_W::new(self)
    }
    ///Bit 2 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    #[must_use]
    pub fn lck2(&mut self) -> LCK2_W<2> {
        LCK2_W::new(self)
    }
    ///Bit 3 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    #[must_use]
    pub fn lck3(&mut self) -> LCK3_W<3> {
        LCK3_W::new(self)
    }
    ///Bit 4 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    #[must_use]
    pub fn lck4(&mut self) -> LCK4_W<4> {
        LCK4_W::new(self)
    }
    ///Bit 16 - Port x lock bit y (y= 0..15)
    #[inline(always)]
    #[must_use]
    pub fn lckk(&mut self) -> LCKK_W<16> {
        LCKK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port configuration lock register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [lckr](index.html) module
pub struct LCKR_SPEC;
impl crate::RegisterSpec for LCKR_SPEC {
    type Ux = u32;
}
///`read()` method returns [lckr::R](R) reader structure
impl crate::Readable for LCKR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [lckr::W](W) writer structure
impl crate::Writable for LCKR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets LCKR to value 0
impl crate::Resettable for LCKR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
