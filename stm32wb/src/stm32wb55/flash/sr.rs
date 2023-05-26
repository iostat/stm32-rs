///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `EOP` reader - End of operation
pub type EOP_R = crate::BitReader<bool>;
///Field `EOP` writer - End of operation
pub type EOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `OPERR` reader - Operation error
pub type OPERR_R = crate::BitReader<bool>;
///Field `OPERR` writer - Operation error
pub type OPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `PROGERR` reader - Programming error
pub type PROGERR_R = crate::BitReader<bool>;
///Field `PROGERR` writer - Programming error
pub type PROGERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `WRPERR` reader - Write protected error
pub type WRPERR_R = crate::BitReader<bool>;
///Field `WRPERR` writer - Write protected error
pub type WRPERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `PGAERR` reader - Programming alignment error
pub type PGAERR_R = crate::BitReader<bool>;
///Field `PGAERR` writer - Programming alignment error
pub type PGAERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `SIZERR` reader - Size error
pub type SIZERR_R = crate::BitReader<bool>;
///Field `SIZERR` writer - Size error
pub type SIZERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `PGSERR` reader - Programming sequence error
pub type PGSERR_R = crate::BitReader<bool>;
///Field `PGSERR` writer - Programming sequence error
pub type PGSERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `MISERR` reader - Fast programming data miss error
pub type MISERR_R = crate::BitReader<bool>;
///Field `MISERR` writer - Fast programming data miss error
pub type MISERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `FASTERR` reader - Fast programming error
pub type FASTERR_R = crate::BitReader<bool>;
///Field `FASTERR` writer - Fast programming error
pub type FASTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `OPTNV` reader - User Option OPTVAL indication
pub type OPTNV_R = crate::BitReader<bool>;
///Field `RDERR` reader - PCROP read error
pub type RDERR_R = crate::BitReader<bool>;
///Field `RDERR` writer - PCROP read error
pub type RDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `OPTVERR` reader - Option validity error
pub type OPTVERR_R = crate::BitReader<bool>;
///Field `OPTVERR` writer - Option validity error
pub type OPTVERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `BSY` reader - Busy
pub type BSY_R = crate::BitReader<bool>;
///Field `CFGBSY` reader - Programming or erase configuration busy
pub type CFGBSY_R = crate::BitReader<bool>;
///Field `PESD` reader - Programming or erase operation suspended
pub type PESD_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - End of operation
    #[inline(always)]
    pub fn eop(&self) -> EOP_R {
        EOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    pub fn operr(&self) -> OPERR_R {
        OPERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Write protected error
    #[inline(always)]
    pub fn wrperr(&self) -> WRPERR_R {
        WRPERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    pub fn pgaerr(&self) -> PGAERR_R {
        PGAERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    pub fn sizerr(&self) -> SIZERR_R {
        SIZERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    pub fn pgserr(&self) -> PGSERR_R {
        PGSERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    pub fn miserr(&self) -> MISERR_R {
        MISERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    pub fn fasterr(&self) -> FASTERR_R {
        FASTERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 13 - User Option OPTVAL indication
    #[inline(always)]
    pub fn optnv(&self) -> OPTNV_R {
        OPTNV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    pub fn rderr(&self) -> RDERR_R {
        RDERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Option validity error
    #[inline(always)]
    pub fn optverr(&self) -> OPTVERR_R {
        OPTVERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Busy
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - Programming or erase configuration busy
    #[inline(always)]
    pub fn cfgbsy(&self) -> CFGBSY_R {
        CFGBSY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Programming or erase operation suspended
    #[inline(always)]
    pub fn pesd(&self) -> PESD_R {
        PESD_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - End of operation
    #[inline(always)]
    #[must_use]
    pub fn eop(&mut self) -> EOP_W<0> {
        EOP_W::new(self)
    }
    ///Bit 1 - Operation error
    #[inline(always)]
    #[must_use]
    pub fn operr(&mut self) -> OPERR_W<1> {
        OPERR_W::new(self)
    }
    ///Bit 3 - Programming error
    #[inline(always)]
    #[must_use]
    pub fn progerr(&mut self) -> PROGERR_W<3> {
        PROGERR_W::new(self)
    }
    ///Bit 4 - Write protected error
    #[inline(always)]
    #[must_use]
    pub fn wrperr(&mut self) -> WRPERR_W<4> {
        WRPERR_W::new(self)
    }
    ///Bit 5 - Programming alignment error
    #[inline(always)]
    #[must_use]
    pub fn pgaerr(&mut self) -> PGAERR_W<5> {
        PGAERR_W::new(self)
    }
    ///Bit 6 - Size error
    #[inline(always)]
    #[must_use]
    pub fn sizerr(&mut self) -> SIZERR_W<6> {
        SIZERR_W::new(self)
    }
    ///Bit 7 - Programming sequence error
    #[inline(always)]
    #[must_use]
    pub fn pgserr(&mut self) -> PGSERR_W<7> {
        PGSERR_W::new(self)
    }
    ///Bit 8 - Fast programming data miss error
    #[inline(always)]
    #[must_use]
    pub fn miserr(&mut self) -> MISERR_W<8> {
        MISERR_W::new(self)
    }
    ///Bit 9 - Fast programming error
    #[inline(always)]
    #[must_use]
    pub fn fasterr(&mut self) -> FASTERR_W<9> {
        FASTERR_W::new(self)
    }
    ///Bit 14 - PCROP read error
    #[inline(always)]
    #[must_use]
    pub fn rderr(&mut self) -> RDERR_W<14> {
        RDERR_W::new(self)
    }
    ///Bit 15 - Option validity error
    #[inline(always)]
    #[must_use]
    pub fn optverr(&mut self) -> OPTVERR_W<15> {
        OPTVERR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
