///Register `TIMEOUTR` reader
pub struct R(crate::R<TIMEOUTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEOUTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEOUTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEOUTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TIMEOUTR` writer
pub struct W(crate::W<TIMEOUTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMEOUTR_SPEC>;
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
impl From<crate::W<TIMEOUTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMEOUTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `TIMEOUTA` reader - Bus timeout A
pub type TIMEOUTA_R = crate::FieldReader<u16, u16>;
///Field `TIMEOUTA` writer - Bus timeout A
pub type TIMEOUTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMEOUTR_SPEC, u16, u16, 12, O>;
///Field `TIDLE` reader - Idle clock timeout detection
pub type TIDLE_R = crate::BitReader<bool>;
///Field `TIDLE` writer - Idle clock timeout detection
pub type TIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMEOUTR_SPEC, bool, O>;
///Field `TIMOUTEN` reader - Clock timeout enable
pub type TIMOUTEN_R = crate::BitReader<bool>;
///Field `TIMOUTEN` writer - Clock timeout enable
pub type TIMOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMEOUTR_SPEC, bool, O>;
///Field `TIMEOUTB` reader - Bus timeout B
pub type TIMEOUTB_R = crate::FieldReader<u16, u16>;
///Field `TIMEOUTB` writer - Bus timeout B
pub type TIMEOUTB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMEOUTR_SPEC, u16, u16, 12, O>;
///Field `TEXTEN` reader - Extended clock timeout enable
pub type TEXTEN_R = crate::BitReader<bool>;
///Field `TEXTEN` writer - Extended clock timeout enable
pub type TEXTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMEOUTR_SPEC, bool, O>;
impl R {
    ///Bits 0:11 - Bus timeout A
    #[inline(always)]
    pub fn timeouta(&self) -> TIMEOUTA_R {
        TIMEOUTA_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bit 12 - Idle clock timeout detection
    #[inline(always)]
    pub fn tidle(&self) -> TIDLE_R {
        TIDLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    pub fn timouten(&self) -> TIMOUTEN_R {
        TIMOUTEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:27 - Bus timeout B
    #[inline(always)]
    pub fn timeoutb(&self) -> TIMEOUTB_R {
        TIMEOUTB_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    pub fn texten(&self) -> TEXTEN_R {
        TEXTEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:11 - Bus timeout A
    #[inline(always)]
    #[must_use]
    pub fn timeouta(&mut self) -> TIMEOUTA_W<0> {
        TIMEOUTA_W::new(self)
    }
    ///Bit 12 - Idle clock timeout detection
    #[inline(always)]
    #[must_use]
    pub fn tidle(&mut self) -> TIDLE_W<12> {
        TIDLE_W::new(self)
    }
    ///Bit 15 - Clock timeout enable
    #[inline(always)]
    #[must_use]
    pub fn timouten(&mut self) -> TIMOUTEN_W<15> {
        TIMOUTEN_W::new(self)
    }
    ///Bits 16:27 - Bus timeout B
    #[inline(always)]
    #[must_use]
    pub fn timeoutb(&mut self) -> TIMEOUTB_W<16> {
        TIMEOUTB_W::new(self)
    }
    ///Bit 31 - Extended clock timeout enable
    #[inline(always)]
    #[must_use]
    pub fn texten(&mut self) -> TEXTEN_W<31> {
        TEXTEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Status register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [timeoutr](index.html) module
pub struct TIMEOUTR_SPEC;
impl crate::RegisterSpec for TIMEOUTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [timeoutr::R](R) reader structure
impl crate::Readable for TIMEOUTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [timeoutr::W](W) writer structure
impl crate::Writable for TIMEOUTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TIMEOUTR to value 0
impl crate::Resettable for TIMEOUTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
