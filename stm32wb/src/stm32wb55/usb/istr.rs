///Register `ISTR` reader
pub struct R(crate::R<ISTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `ISTR` writer
pub struct W(crate::W<ISTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISTR_SPEC>;
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
impl From<crate::W<ISTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EP_ID` reader - Endpoint Identifier
pub type EP_ID_R = crate::FieldReader<u8, u8>;
///Field `DIR` reader - Direction of transaction
pub type DIR_R = crate::BitReader<bool>;
///Field `L1REQ` reader - LPM L1 state request
pub type L1REQ_R = crate::BitReader<bool>;
///Field `L1REQ` writer - LPM L1 state request
pub type L1REQ_W<'a, const O: u8> = crate::BitWriter<'a, u16, ISTR_SPEC, bool, O>;
///Field `ESOF` reader - Expected start frame
pub type ESOF_R = crate::BitReader<bool>;
///Field `ESOF` writer - Expected start frame
pub type ESOF_W<'a, const O: u8> = crate::BitWriter<'a, u16, ISTR_SPEC, bool, O>;
///Field `SOF` reader - start of frame
pub type SOF_R = crate::BitReader<bool>;
///Field `SOF` writer - start of frame
pub type SOF_W<'a, const O: u8> = crate::BitWriter<'a, u16, ISTR_SPEC, bool, O>;
///Field `RESET` reader - reset request
pub type RESET_R = crate::BitReader<bool>;
///Field `RESET` writer - reset request
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, u16, ISTR_SPEC, bool, O>;
///Field `SUSP` reader - Suspend mode request
pub type SUSP_R = crate::BitReader<bool>;
///Field `SUSP` writer - Suspend mode request
pub type SUSP_W<'a, const O: u8> = crate::BitWriter<'a, u16, ISTR_SPEC, bool, O>;
///Field `WKUP` reader - Wakeup
pub type WKUP_R = crate::BitReader<bool>;
///Field `WKUP` writer - Wakeup
pub type WKUP_W<'a, const O: u8> = crate::BitWriter<'a, u16, ISTR_SPEC, bool, O>;
///Field `ERR` reader - Error
pub type ERR_R = crate::BitReader<bool>;
///Field `ERR` writer - Error
pub type ERR_W<'a, const O: u8> = crate::BitWriter<'a, u16, ISTR_SPEC, bool, O>;
///Field `PMAOVR` reader - Packet memory area over / underrun
pub type PMAOVR_R = crate::BitReader<bool>;
///Field `PMAOVR` writer - Packet memory area over / underrun
pub type PMAOVR_W<'a, const O: u8> = crate::BitWriter<'a, u16, ISTR_SPEC, bool, O>;
///Field `CTR` reader - Correct transfer
pub type CTR_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:3 - Endpoint Identifier
    #[inline(always)]
    pub fn ep_id(&self) -> EP_ID_R {
        EP_ID_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - Direction of transaction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - LPM L1 state request
    #[inline(always)]
    pub fn l1req(&self) -> L1REQ_R {
        L1REQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Expected start frame
    #[inline(always)]
    pub fn esof(&self) -> ESOF_R {
        ESOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - start of frame
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - reset request
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Suspend mode request
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Wakeup
    #[inline(always)]
    pub fn wkup(&self) -> WKUP_R {
        WKUP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Error
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Packet memory area over / underrun
    #[inline(always)]
    pub fn pmaovr(&self) -> PMAOVR_R {
        PMAOVR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Correct transfer
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 7 - LPM L1 state request
    #[inline(always)]
    #[must_use]
    pub fn l1req(&mut self) -> L1REQ_W<7> {
        L1REQ_W::new(self)
    }
    ///Bit 8 - Expected start frame
    #[inline(always)]
    #[must_use]
    pub fn esof(&mut self) -> ESOF_W<8> {
        ESOF_W::new(self)
    }
    ///Bit 9 - start of frame
    #[inline(always)]
    #[must_use]
    pub fn sof(&mut self) -> SOF_W<9> {
        SOF_W::new(self)
    }
    ///Bit 10 - reset request
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<10> {
        RESET_W::new(self)
    }
    ///Bit 11 - Suspend mode request
    #[inline(always)]
    #[must_use]
    pub fn susp(&mut self) -> SUSP_W<11> {
        SUSP_W::new(self)
    }
    ///Bit 12 - Wakeup
    #[inline(always)]
    #[must_use]
    pub fn wkup(&mut self) -> WKUP_W<12> {
        WKUP_W::new(self)
    }
    ///Bit 13 - Error
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ERR_W<13> {
        ERR_W::new(self)
    }
    ///Bit 14 - Packet memory area over / underrun
    #[inline(always)]
    #[must_use]
    pub fn pmaovr(&mut self) -> PMAOVR_W<14> {
        PMAOVR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///interrupt status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [istr](index.html) module
pub struct ISTR_SPEC;
impl crate::RegisterSpec for ISTR_SPEC {
    type Ux = u16;
}
///`read()` method returns [istr::R](R) reader structure
impl crate::Readable for ISTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [istr::W](W) writer structure
impl crate::Writable for ISTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets ISTR to value 0
impl crate::Resettable for ISTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
