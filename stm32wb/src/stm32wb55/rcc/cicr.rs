///Register `CICR` writer
pub struct W(crate::W<CICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CICR_SPEC>;
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
impl From<crate::W<CICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CICR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSI1RDYC` writer - LSI1 ready interrupt clear
pub type LSI1RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
///Field `LSERDYC` writer - LSE ready interrupt clear
pub type LSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
///Field `MSIRDYC` writer - MSI ready interrupt clear
pub type MSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
///Field `HSIRDYC` writer - HSI ready interrupt clear
pub type HSIRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
///Field `HSERDYC` writer - HSE ready interrupt clear
pub type HSERDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
///Field `PLLRDYC` writer - PLL ready interrupt clear
pub type PLLRDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
///Field `PLLSAI1RDYC` writer - PLLSAI1 ready interrupt clear
pub type PLLSAI1RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
///Field `HSECSSC` writer - HSE Clock security system interrupt clear
pub type HSECSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
///Field `LSECSSC` writer - LSE Clock security system interrupt clear
pub type LSECSSC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
///Field `HSI48RDYC` writer - HSI48 ready interrupt clear
pub type HSI48RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
///Field `LSI2RDYC` writer - LSI2 ready interrupt clear
pub type LSI2RDYC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CICR_SPEC, bool, O>;
impl W {
    ///Bit 0 - LSI1 ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn lsi1rdyc(&mut self) -> LSI1RDYC_W<0> {
        LSI1RDYC_W::new(self)
    }
    ///Bit 1 - LSE ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn lserdyc(&mut self) -> LSERDYC_W<1> {
        LSERDYC_W::new(self)
    }
    ///Bit 2 - MSI ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn msirdyc(&mut self) -> MSIRDYC_W<2> {
        MSIRDYC_W::new(self)
    }
    ///Bit 3 - HSI ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn hsirdyc(&mut self) -> HSIRDYC_W<3> {
        HSIRDYC_W::new(self)
    }
    ///Bit 4 - HSE ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn hserdyc(&mut self) -> HSERDYC_W<4> {
        HSERDYC_W::new(self)
    }
    ///Bit 5 - PLL ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn pllrdyc(&mut self) -> PLLRDYC_W<5> {
        PLLRDYC_W::new(self)
    }
    ///Bit 6 - PLLSAI1 ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn pllsai1rdyc(&mut self) -> PLLSAI1RDYC_W<6> {
        PLLSAI1RDYC_W::new(self)
    }
    ///Bit 8 - HSE Clock security system interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn hsecssc(&mut self) -> HSECSSC_W<8> {
        HSECSSC_W::new(self)
    }
    ///Bit 9 - LSE Clock security system interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn lsecssc(&mut self) -> LSECSSC_W<9> {
        LSECSSC_W::new(self)
    }
    ///Bit 10 - HSI48 ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyc(&mut self) -> HSI48RDYC_W<10> {
        HSI48RDYC_W::new(self)
    }
    ///Bit 11 - LSI2 ready interrupt clear
    #[inline(always)]
    #[must_use]
    pub fn lsi2rdyc(&mut self) -> LSI2RDYC_W<11> {
        LSI2RDYC_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock interrupt clear register
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cicr](index.html) module
pub struct CICR_SPEC;
impl crate::RegisterSpec for CICR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [cicr::W](W) writer structure
impl crate::Writable for CICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CICR to value 0
impl crate::Resettable for CICR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
