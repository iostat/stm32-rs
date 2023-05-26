///Register `CLR` writer
pub struct W(crate::W<CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLR_SPEC>;
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
impl From<crate::W<CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SOFC` writer - Start of frame flag clear
pub type SOFC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, bool, O>;
///Field `UDDC` writer - Update display done clear
pub type UDDC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLR_SPEC, bool, O>;
impl W {
    ///Bit 1 - Start of frame flag clear
    #[inline(always)]
    #[must_use]
    pub fn sofc(&mut self) -> SOFC_W<1> {
        SOFC_W::new(self)
    }
    ///Bit 3 - Update display done clear
    #[inline(always)]
    #[must_use]
    pub fn uddc(&mut self) -> UDDC_W<3> {
        UDDC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clr](index.html) module
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [clr::W](W) writer structure
impl crate::Writable for CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CLR to value 0
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
