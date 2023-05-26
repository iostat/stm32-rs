///Register `CLRFR` writer
pub struct W(crate::W<CLRFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLRFR_SPEC>;
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
impl From<crate::W<CLRFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLRFR_SPEC>) -> Self {
        W(writer)
    }
}
///Clear overrun / underrun
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COVRUDRW_AW {
    ///1: Clears the OVRUDR flag
    Clear = 1,
}
impl From<COVRUDRW_AW> for bool {
    #[inline(always)]
    fn from(variant: COVRUDRW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `COVRUDR` writer - Clear overrun / underrun
pub type COVRUDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, COVRUDRW_AW, O>;
impl<'a, const O: u8> COVRUDR_W<'a, O> {
    ///Clears the OVRUDR flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(COVRUDRW_AW::Clear)
    }
}
///Mute detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMUTEDETW_AW {
    ///1: Clears the MUTEDET flag
    Clear = 1,
}
impl From<CMUTEDETW_AW> for bool {
    #[inline(always)]
    fn from(variant: CMUTEDETW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMUTEDET` writer - Mute detection flag
pub type CMUTEDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CMUTEDETW_AW, O>;
impl<'a, const O: u8> CMUTEDET_W<'a, O> {
    ///Clears the MUTEDET flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CMUTEDETW_AW::Clear)
    }
}
///Clear wrong clock configuration flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWCKCFGW_AW {
    ///1: Clears the WCKCFG flag
    Clear = 1,
}
impl From<CWCKCFGW_AW> for bool {
    #[inline(always)]
    fn from(variant: CWCKCFGW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CWCKCFG` writer - Clear wrong clock configuration flag
pub type CWCKCFG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CWCKCFGW_AW, O>;
impl<'a, const O: u8> CWCKCFG_W<'a, O> {
    ///Clears the WCKCFG flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CWCKCFGW_AW::Clear)
    }
}
///Clear codec not ready flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCNRDYW_AW {
    ///1: Clears the CNRDY flag
    Clear = 1,
}
impl From<CCNRDYW_AW> for bool {
    #[inline(always)]
    fn from(variant: CCNRDYW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CCNRDY` writer - Clear codec not ready flag
pub type CCNRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CCNRDYW_AW, O>;
impl<'a, const O: u8> CCNRDY_W<'a, O> {
    ///Clears the CNRDY flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CCNRDYW_AW::Clear)
    }
}
///Clear anticipated frame synchronization detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAFSDETW_AW {
    ///1: Clears the AFSDET flag
    Clear = 1,
}
impl From<CAFSDETW_AW> for bool {
    #[inline(always)]
    fn from(variant: CAFSDETW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag
pub type CAFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CAFSDETW_AW, O>;
impl<'a, const O: u8> CAFSDET_W<'a, O> {
    ///Clears the AFSDET flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CAFSDETW_AW::Clear)
    }
}
///Clear late frame synchronization detection flag
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLFSDETW_AW {
    ///1: Clears the LFSDET flag
    Clear = 1,
}
impl From<CLFSDETW_AW> for bool {
    #[inline(always)]
    fn from(variant: CLFSDETW_AW) -> Self {
        variant as u8 != 0
    }
}
///Field `CLFSDET` writer - Clear late frame synchronization detection flag
pub type CLFSDET_W<'a, const O: u8> = crate::BitWriter<'a, u32, CLRFR_SPEC, CLFSDETW_AW, O>;
impl<'a, const O: u8> CLFSDET_W<'a, O> {
    ///Clears the LFSDET flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(CLFSDETW_AW::Clear)
    }
}
impl W {
    ///Bit 0 - Clear overrun / underrun
    #[inline(always)]
    #[must_use]
    pub fn covrudr(&mut self) -> COVRUDR_W<0> {
        COVRUDR_W::new(self)
    }
    ///Bit 1 - Mute detection flag
    #[inline(always)]
    #[must_use]
    pub fn cmutedet(&mut self) -> CMUTEDET_W<1> {
        CMUTEDET_W::new(self)
    }
    ///Bit 2 - Clear wrong clock configuration flag
    #[inline(always)]
    #[must_use]
    pub fn cwckcfg(&mut self) -> CWCKCFG_W<2> {
        CWCKCFG_W::new(self)
    }
    ///Bit 4 - Clear codec not ready flag
    #[inline(always)]
    #[must_use]
    pub fn ccnrdy(&mut self) -> CCNRDY_W<4> {
        CCNRDY_W::new(self)
    }
    ///Bit 5 - Clear anticipated frame synchronization detection flag
    #[inline(always)]
    #[must_use]
    pub fn cafsdet(&mut self) -> CAFSDET_W<5> {
        CAFSDET_W::new(self)
    }
    ///Bit 6 - Clear late frame synchronization detection flag
    #[inline(always)]
    #[must_use]
    pub fn clfsdet(&mut self) -> CLFSDET_W<6> {
        CLFSDET_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AClear flag register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [clrfr](index.html) module
pub struct CLRFR_SPEC;
impl crate::RegisterSpec for CLRFR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [clrfr::W](W) writer structure
impl crate::Writable for CLRFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CLRFR to value 0
impl crate::Resettable for CLRFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
