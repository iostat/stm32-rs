///Register `CNTR` reader
pub struct R(crate::R<CNTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CNTR` writer
pub struct W(crate::W<CNTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNTR_SPEC>;
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
impl From<crate::W<CNTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `FRES` reader - Force USB Reset
pub type FRES_R = crate::BitReader<bool>;
///Field `FRES` writer - Force USB Reset
pub type FRES_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `PDWN` reader - Power down
pub type PDWN_R = crate::BitReader<bool>;
///Field `PDWN` writer - Power down
pub type PDWN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `LPMODE` reader - Low-power mode
pub type LPMODE_R = crate::BitReader<bool>;
///Field `LPMODE` writer - Low-power mode
pub type LPMODE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `FSUSP` reader - Force suspend
pub type FSUSP_R = crate::BitReader<bool>;
///Field `FSUSP` writer - Force suspend
pub type FSUSP_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `RESUME` reader - Resume request
pub type RESUME_R = crate::BitReader<bool>;
///Field `RESUME` writer - Resume request
pub type RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `L1RESUME` reader - LPM L1 Resume request
pub type L1RESUME_R = crate::BitReader<bool>;
///Field `L1RESUME` writer - LPM L1 Resume request
pub type L1RESUME_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `L1REQM` reader - LPM L1 state request interrupt mask
pub type L1REQM_R = crate::BitReader<bool>;
///Field `L1REQM` writer - LPM L1 state request interrupt mask
pub type L1REQM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `ESOFM` reader - Expected start of frame interrupt mask
pub type ESOFM_R = crate::BitReader<bool>;
///Field `ESOFM` writer - Expected start of frame interrupt mask
pub type ESOFM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `SOFM` reader - Start of frame interrupt mask
pub type SOFM_R = crate::BitReader<bool>;
///Field `SOFM` writer - Start of frame interrupt mask
pub type SOFM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `RESETM` reader - USB reset interrupt mask
pub type RESETM_R = crate::BitReader<bool>;
///Field `RESETM` writer - USB reset interrupt mask
pub type RESETM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `SUSPM` reader - Suspend mode interrupt mask
pub type SUSPM_R = crate::BitReader<bool>;
///Field `SUSPM` writer - Suspend mode interrupt mask
pub type SUSPM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `WKUPM` reader - Wakeup interrupt mask
pub type WKUPM_R = crate::BitReader<bool>;
///Field `WKUPM` writer - Wakeup interrupt mask
pub type WKUPM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `ERRM` reader - Error interrupt mask
pub type ERRM_R = crate::BitReader<bool>;
///Field `ERRM` writer - Error interrupt mask
pub type ERRM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `PMAOVRM` reader - Packet memory area over / underrun interrupt mask
pub type PMAOVRM_R = crate::BitReader<bool>;
///Field `PMAOVRM` writer - Packet memory area over / underrun interrupt mask
pub type PMAOVRM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
///Field `CTRM` reader - Correct transfer interrupt mask
pub type CTRM_R = crate::BitReader<bool>;
///Field `CTRM` writer - Correct transfer interrupt mask
pub type CTRM_W<'a, const O: u8> = crate::BitWriter<'a, u16, CNTR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Force USB Reset
    #[inline(always)]
    pub fn fres(&self) -> FRES_R {
        FRES_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Power down
    #[inline(always)]
    pub fn pdwn(&self) -> PDWN_R {
        PDWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Low-power mode
    #[inline(always)]
    pub fn lpmode(&self) -> LPMODE_R {
        LPMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Force suspend
    #[inline(always)]
    pub fn fsusp(&self) -> FSUSP_R {
        FSUSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Resume request
    #[inline(always)]
    pub fn resume(&self) -> RESUME_R {
        RESUME_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LPM L1 Resume request
    #[inline(always)]
    pub fn l1resume(&self) -> L1RESUME_R {
        L1RESUME_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - LPM L1 state request interrupt mask
    #[inline(always)]
    pub fn l1reqm(&self) -> L1REQM_R {
        L1REQM_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Expected start of frame interrupt mask
    #[inline(always)]
    pub fn esofm(&self) -> ESOFM_R {
        ESOFM_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Start of frame interrupt mask
    #[inline(always)]
    pub fn sofm(&self) -> SOFM_R {
        SOFM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - USB reset interrupt mask
    #[inline(always)]
    pub fn resetm(&self) -> RESETM_R {
        RESETM_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Suspend mode interrupt mask
    #[inline(always)]
    pub fn suspm(&self) -> SUSPM_R {
        SUSPM_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wakeup interrupt mask
    #[inline(always)]
    pub fn wkupm(&self) -> WKUPM_R {
        WKUPM_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Error interrupt mask
    #[inline(always)]
    pub fn errm(&self) -> ERRM_R {
        ERRM_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Packet memory area over / underrun interrupt mask
    #[inline(always)]
    pub fn pmaovrm(&self) -> PMAOVRM_R {
        PMAOVRM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Correct transfer interrupt mask
    #[inline(always)]
    pub fn ctrm(&self) -> CTRM_R {
        CTRM_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Force USB Reset
    #[inline(always)]
    #[must_use]
    pub fn fres(&mut self) -> FRES_W<0> {
        FRES_W::new(self)
    }
    ///Bit 1 - Power down
    #[inline(always)]
    #[must_use]
    pub fn pdwn(&mut self) -> PDWN_W<1> {
        PDWN_W::new(self)
    }
    ///Bit 2 - Low-power mode
    #[inline(always)]
    #[must_use]
    pub fn lpmode(&mut self) -> LPMODE_W<2> {
        LPMODE_W::new(self)
    }
    ///Bit 3 - Force suspend
    #[inline(always)]
    #[must_use]
    pub fn fsusp(&mut self) -> FSUSP_W<3> {
        FSUSP_W::new(self)
    }
    ///Bit 4 - Resume request
    #[inline(always)]
    #[must_use]
    pub fn resume(&mut self) -> RESUME_W<4> {
        RESUME_W::new(self)
    }
    ///Bit 5 - LPM L1 Resume request
    #[inline(always)]
    #[must_use]
    pub fn l1resume(&mut self) -> L1RESUME_W<5> {
        L1RESUME_W::new(self)
    }
    ///Bit 7 - LPM L1 state request interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn l1reqm(&mut self) -> L1REQM_W<7> {
        L1REQM_W::new(self)
    }
    ///Bit 8 - Expected start of frame interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn esofm(&mut self) -> ESOFM_W<8> {
        ESOFM_W::new(self)
    }
    ///Bit 9 - Start of frame interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn sofm(&mut self) -> SOFM_W<9> {
        SOFM_W::new(self)
    }
    ///Bit 10 - USB reset interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn resetm(&mut self) -> RESETM_W<10> {
        RESETM_W::new(self)
    }
    ///Bit 11 - Suspend mode interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn suspm(&mut self) -> SUSPM_W<11> {
        SUSPM_W::new(self)
    }
    ///Bit 12 - Wakeup interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn wkupm(&mut self) -> WKUPM_W<12> {
        WKUPM_W::new(self)
    }
    ///Bit 13 - Error interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn errm(&mut self) -> ERRM_W<13> {
        ERRM_W::new(self)
    }
    ///Bit 14 - Packet memory area over / underrun interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn pmaovrm(&mut self) -> PMAOVRM_W<14> {
        PMAOVRM_W::new(self)
    }
    ///Bit 15 - Correct transfer interrupt mask
    #[inline(always)]
    #[must_use]
    pub fn ctrm(&mut self) -> CTRM_W<15> {
        CTRM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cntr](index.html) module
pub struct CNTR_SPEC;
impl crate::RegisterSpec for CNTR_SPEC {
    type Ux = u16;
}
///`read()` method returns [cntr::R](R) reader structure
impl crate::Readable for CNTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cntr::W](W) writer structure
impl crate::Writable for CNTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CNTR to value 0x03
impl crate::Resettable for CNTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
