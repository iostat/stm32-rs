///Register `CIER` reader
pub struct R(crate::R<CIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CIER` writer
pub struct W(crate::W<CIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIER_SPEC>;
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
impl From<crate::W<CIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSI1RDYIE` reader - LSI1 ready interrupt enable
pub type LSI1RDYIE_R = crate::BitReader<bool>;
///Field `LSI1RDYIE` writer - LSI1 ready interrupt enable
pub type LSI1RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `LSERDYIE` reader - LSE ready interrupt enable
pub type LSERDYIE_R = crate::BitReader<bool>;
///Field `LSERDYIE` writer - LSE ready interrupt enable
pub type LSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `MSIRDYIE` reader - MSI ready interrupt enable
pub type MSIRDYIE_R = crate::BitReader<bool>;
///Field `MSIRDYIE` writer - MSI ready interrupt enable
pub type MSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `HSIRDYIE` reader - HSI ready interrupt enable
pub type HSIRDYIE_R = crate::BitReader<bool>;
///Field `HSIRDYIE` writer - HSI ready interrupt enable
pub type HSIRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `HSERDYIE` reader - HSE ready interrupt enable
pub type HSERDYIE_R = crate::BitReader<bool>;
///Field `HSERDYIE` writer - HSE ready interrupt enable
pub type HSERDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `PLLRDYIE` reader - PLLSYS ready interrupt enable
pub type PLLRDYIE_R = crate::BitReader<bool>;
///Field `PLLRDYIE` writer - PLLSYS ready interrupt enable
pub type PLLRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `PLLSAI1RDYIE` reader - PLLSAI1 ready interrupt enable
pub type PLLSAI1RDYIE_R = crate::BitReader<bool>;
///Field `PLLSAI1RDYIE` writer - PLLSAI1 ready interrupt enable
pub type PLLSAI1RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `LSECSSIE` reader - LSE clock security system interrupt enable
pub type LSECSSIE_R = crate::BitReader<bool>;
///Field `LSECSSIE` writer - LSE clock security system interrupt enable
pub type LSECSSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `HSI48RDYIE` reader - HSI48 ready interrupt enable
pub type HSI48RDYIE_R = crate::BitReader<bool>;
///Field `HSI48RDYIE` writer - HSI48 ready interrupt enable
pub type HSI48RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
///Field `LSI2RDYIE` reader - LSI2 ready interrupt enable
pub type LSI2RDYIE_R = crate::BitReader<bool>;
///Field `LSI2RDYIE` writer - LSI2 ready interrupt enable
pub type LSI2RDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CIER_SPEC, bool, O>;
impl R {
    ///Bit 0 - LSI1 ready interrupt enable
    #[inline(always)]
    pub fn lsi1rdyie(&self) -> LSI1RDYIE_R {
        LSI1RDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE ready interrupt enable
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - MSI ready interrupt enable
    #[inline(always)]
    pub fn msirdyie(&self) -> MSIRDYIE_R {
        MSIRDYIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSI ready interrupt enable
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE ready interrupt enable
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PLLSYS ready interrupt enable
    #[inline(always)]
    pub fn pllrdyie(&self) -> PLLRDYIE_R {
        PLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - PLLSAI1 ready interrupt enable
    #[inline(always)]
    pub fn pllsai1rdyie(&self) -> PLLSAI1RDYIE_R {
        PLLSAI1RDYIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - LSE clock security system interrupt enable
    #[inline(always)]
    pub fn lsecssie(&self) -> LSECSSIE_R {
        LSECSSIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - HSI48 ready interrupt enable
    #[inline(always)]
    pub fn hsi48rdyie(&self) -> HSI48RDYIE_R {
        HSI48RDYIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LSI2 ready interrupt enable
    #[inline(always)]
    pub fn lsi2rdyie(&self) -> LSI2RDYIE_R {
        LSI2RDYIE_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSI1 ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn lsi1rdyie(&mut self) -> LSI1RDYIE_W<0> {
        LSI1RDYIE_W::new(self)
    }
    ///Bit 1 - LSE ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<1> {
        LSERDYIE_W::new(self)
    }
    ///Bit 2 - MSI ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn msirdyie(&mut self) -> MSIRDYIE_W<2> {
        MSIRDYIE_W::new(self)
    }
    ///Bit 3 - HSI ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<3> {
        HSIRDYIE_W::new(self)
    }
    ///Bit 4 - HSE ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<4> {
        HSERDYIE_W::new(self)
    }
    ///Bit 5 - PLLSYS ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn pllrdyie(&mut self) -> PLLRDYIE_W<5> {
        PLLRDYIE_W::new(self)
    }
    ///Bit 6 - PLLSAI1 ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn pllsai1rdyie(&mut self) -> PLLSAI1RDYIE_W<6> {
        PLLSAI1RDYIE_W::new(self)
    }
    ///Bit 9 - LSE clock security system interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn lsecssie(&mut self) -> LSECSSIE_W<9> {
        LSECSSIE_W::new(self)
    }
    ///Bit 10 - HSI48 ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn hsi48rdyie(&mut self) -> HSI48RDYIE_W<10> {
        HSI48RDYIE_W::new(self)
    }
    ///Bit 11 - LSI2 ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn lsi2rdyie(&mut self) -> LSI2RDYIE_W<11> {
        LSI2RDYIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Clock interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cier](index.html) module
pub struct CIER_SPEC;
impl crate::RegisterSpec for CIER_SPEC {
    type Ux = u32;
}
///`read()` method returns [cier::R](R) reader structure
impl crate::Readable for CIER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cier::W](W) writer structure
impl crate::Writable for CIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
