///Register `BRR` writer
pub struct W(crate::W<BRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BRR_SPEC>;
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
impl From<crate::W<BRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BRR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BR0` writer - Port Reset bit
pub type BR0_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
///Field `BR1` writer - Port Reset bit
pub type BR1_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
///Field `BR3` writer - Port Reset bit
pub type BR3_W<'a, const O: u8> = crate::BitWriter<'a, u32, BRR_SPEC, bool, O>;
impl W {
    ///Bit 0 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br0(&mut self) -> BR0_W<0> {
        BR0_W::new(self)
    }
    ///Bit 1 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br1(&mut self) -> BR1_W<1> {
        BR1_W::new(self)
    }
    ///Bit 3 - Port Reset bit
    #[inline(always)]
    #[must_use]
    pub fn br3(&mut self) -> BR3_W<3> {
        BR3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///port bit reset register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [brr](index.html) module
pub struct BRR_SPEC;
impl crate::RegisterSpec for BRR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [brr::W](W) writer structure
impl crate::Writable for BRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BRR to value 0
impl crate::Resettable for BRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
