///Register `BDCR` reader
pub struct R(crate::R<BDCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDCR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BDCR` writer
pub struct W(crate::W<BDCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDCR_SPEC>;
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
impl From<crate::W<BDCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSEON` reader - LSE oscillator enable
pub type LSEON_R = crate::BitReader<bool>;
///Field `LSEON` writer - LSE oscillator enable
pub type LSEON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
///Field `LSERDY` reader - LSE oscillator ready
pub type LSERDY_R = crate::BitReader<bool>;
///Field `LSEBYP` reader - LSE oscillator bypass
pub type LSEBYP_R = crate::BitReader<bool>;
///Field `LSEBYP` writer - LSE oscillator bypass
pub type LSEBYP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
///Field `LSEDRV` reader - SE oscillator drive capability
pub type LSEDRV_R = crate::FieldReader<u8, u8>;
///Field `LSEDRV` writer - SE oscillator drive capability
pub type LSEDRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDCR_SPEC, u8, u8, 2, O>;
///Field `LSECSSON` reader - LSECSSON
pub type LSECSSON_R = crate::BitReader<bool>;
///Field `LSECSSON` writer - LSECSSON
pub type LSECSSON_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
///Field `LSECSSD_` reader - CSS on LSE failure detection
pub type LSECSSD__R = crate::BitReader<bool>;
///Field `RTCSEL` reader - RTC clock source selection
pub type RTCSEL_R = crate::FieldReader<u8, u8>;
///Field `RTCSEL` writer - RTC clock source selection
pub type RTCSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDCR_SPEC, u8, u8, 2, O>;
///Field `RTCEN` reader - RTC clock enable
pub type RTCEN_R = crate::BitReader<bool>;
///Field `RTCEN` writer - RTC clock enable
pub type RTCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
///Field `BDRST` reader - Backup domain software reset
pub type BDRST_R = crate::BitReader<bool>;
///Field `BDRST` writer - Backup domain software reset
pub type BDRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
///Field `LSCOEN` reader - Low speed clock output enable
pub type LSCOEN_R = crate::BitReader<bool>;
///Field `LSCOEN` writer - Low speed clock output enable
pub type LSCOEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDCR_SPEC, bool, O>;
///Field `LSCOSEL` reader - Low speed clock output selection
pub type LSCOSEL_R = crate::FieldReader<u8, u8>;
///Field `LSCOSEL` writer - Low speed clock output selection
pub type LSCOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDCR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 0 - LSE oscillator enable
    #[inline(always)]
    pub fn lseon(&self) -> LSEON_R {
        LSEON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE oscillator ready
    #[inline(always)]
    pub fn lserdy(&self) -> LSERDY_R {
        LSERDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSE oscillator bypass
    #[inline(always)]
    pub fn lsebyp(&self) -> LSEBYP_R {
        LSEBYP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:4 - SE oscillator drive capability
    #[inline(always)]
    pub fn lsedrv(&self) -> LSEDRV_R {
        LSEDRV_R::new(((self.bits >> 3) & 3) as u8)
    }
    ///Bit 5 - LSECSSON
    #[inline(always)]
    pub fn lsecsson(&self) -> LSECSSON_R {
        LSECSSON_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CSS on LSE failure detection
    #[inline(always)]
    pub fn lsecssd_(&self) -> LSECSSD__R {
        LSECSSD__R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    pub fn rtcsel(&self) -> RTCSEL_R {
        RTCSEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    pub fn rtcen(&self) -> RTCEN_R {
        RTCEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    pub fn bdrst(&self) -> BDRST_R {
        BDRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 24 - Low speed clock output enable
    #[inline(always)]
    pub fn lscoen(&self) -> LSCOEN_R {
        LSCOEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Low speed clock output selection
    #[inline(always)]
    pub fn lscosel(&self) -> LSCOSEL_R {
        LSCOSEL_R::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    ///Bit 0 - LSE oscillator enable
    #[inline(always)]
    #[must_use]
    pub fn lseon(&mut self) -> LSEON_W<0> {
        LSEON_W::new(self)
    }
    ///Bit 2 - LSE oscillator bypass
    #[inline(always)]
    #[must_use]
    pub fn lsebyp(&mut self) -> LSEBYP_W<2> {
        LSEBYP_W::new(self)
    }
    ///Bits 3:4 - SE oscillator drive capability
    #[inline(always)]
    #[must_use]
    pub fn lsedrv(&mut self) -> LSEDRV_W<3> {
        LSEDRV_W::new(self)
    }
    ///Bit 5 - LSECSSON
    #[inline(always)]
    #[must_use]
    pub fn lsecsson(&mut self) -> LSECSSON_W<5> {
        LSECSSON_W::new(self)
    }
    ///Bits 8:9 - RTC clock source selection
    #[inline(always)]
    #[must_use]
    pub fn rtcsel(&mut self) -> RTCSEL_W<8> {
        RTCSEL_W::new(self)
    }
    ///Bit 15 - RTC clock enable
    #[inline(always)]
    #[must_use]
    pub fn rtcen(&mut self) -> RTCEN_W<15> {
        RTCEN_W::new(self)
    }
    ///Bit 16 - Backup domain software reset
    #[inline(always)]
    #[must_use]
    pub fn bdrst(&mut self) -> BDRST_W<16> {
        BDRST_W::new(self)
    }
    ///Bit 24 - Low speed clock output enable
    #[inline(always)]
    #[must_use]
    pub fn lscoen(&mut self) -> LSCOEN_W<24> {
        LSCOEN_W::new(self)
    }
    ///Bits 25:26 - Low speed clock output selection
    #[inline(always)]
    #[must_use]
    pub fn lscosel(&mut self) -> LSCOSEL_W<25> {
        LSCOSEL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///BDCR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdcr](index.html) module
pub struct BDCR_SPEC;
impl crate::RegisterSpec for BDCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bdcr::R](R) reader structure
impl crate::Readable for BDCR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bdcr::W](W) writer structure
impl crate::Writable for BDCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BDCR to value 0
impl crate::Resettable for BDCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
