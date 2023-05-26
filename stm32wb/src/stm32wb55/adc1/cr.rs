///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADEN` reader - ADC enable
pub type ADEN_R = crate::BitReader<bool>;
///Field `ADEN` writer - ADC enable
pub type ADEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ADDIS` reader - ADC disable
pub type ADDIS_R = crate::BitReader<bool>;
///Field `ADDIS` writer - ADC disable
pub type ADDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ADSTART` reader - ADC group regular conversion start
pub type ADSTART_R = crate::BitReader<bool>;
///Field `ADSTART` writer - ADC group regular conversion start
pub type ADSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `JADSTART` reader - ADC group injected conversion start
pub type JADSTART_R = crate::BitReader<bool>;
///Field `JADSTART` writer - ADC group injected conversion start
pub type JADSTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ADSTP` reader - ADC group regular conversion stop
pub type ADSTP_R = crate::BitReader<bool>;
///Field `ADSTP` writer - ADC group regular conversion stop
pub type ADSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `JADSTP` reader - ADC group injected conversion stop
pub type JADSTP_R = crate::BitReader<bool>;
///Field `JADSTP` writer - ADC group injected conversion stop
pub type JADSTP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ADVREGEN` reader - ADC voltage regulator enable
pub type ADVREGEN_R = crate::BitReader<bool>;
///Field `ADVREGEN` writer - ADC voltage regulator enable
pub type ADVREGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DEEPPWD` reader - ADC deep power down enable
pub type DEEPPWD_R = crate::BitReader<bool>;
///Field `DEEPPWD` writer - ADC deep power down enable
pub type DEEPPWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ADCALDIF` reader - ADC differential mode for calibration
pub type ADCALDIF_R = crate::BitReader<bool>;
///Field `ADCALDIF` writer - ADC differential mode for calibration
pub type ADCALDIF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ADCAL` reader - ADC calibration
pub type ADCAL_R = crate::BitReader<bool>;
///Field `ADCAL` writer - ADC calibration
pub type ADCAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - ADC enable
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC disable
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC group regular conversion start
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC group injected conversion start
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC group regular conversion stop
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC group injected conversion stop
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 28 - ADC voltage regulator enable
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ADC deep power down enable
    #[inline(always)]
    pub fn deeppwd(&self) -> DEEPPWD_R {
        DEEPPWD_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - ADC differential mode for calibration
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - ADC calibration
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ADC enable
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<0> {
        ADEN_W::new(self)
    }
    ///Bit 1 - ADC disable
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<1> {
        ADDIS_W::new(self)
    }
    ///Bit 2 - ADC group regular conversion start
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<2> {
        ADSTART_W::new(self)
    }
    ///Bit 3 - ADC group injected conversion start
    #[inline(always)]
    #[must_use]
    pub fn jadstart(&mut self) -> JADSTART_W<3> {
        JADSTART_W::new(self)
    }
    ///Bit 4 - ADC group regular conversion stop
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<4> {
        ADSTP_W::new(self)
    }
    ///Bit 5 - ADC group injected conversion stop
    #[inline(always)]
    #[must_use]
    pub fn jadstp(&mut self) -> JADSTP_W<5> {
        JADSTP_W::new(self)
    }
    ///Bit 28 - ADC voltage regulator enable
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<28> {
        ADVREGEN_W::new(self)
    }
    ///Bit 29 - ADC deep power down enable
    #[inline(always)]
    #[must_use]
    pub fn deeppwd(&mut self) -> DEEPPWD_W<29> {
        DEEPPWD_W::new(self)
    }
    ///Bit 30 - ADC differential mode for calibration
    #[inline(always)]
    #[must_use]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<30> {
        ADCALDIF_W::new(self)
    }
    ///Bit 31 - ADC calibration
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<31> {
        ADCAL_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
