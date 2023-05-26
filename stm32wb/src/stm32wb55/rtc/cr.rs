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
///Field `WUCKSEL` reader - Wakeup clock selection
pub type WUCKSEL_R = crate::FieldReader<u8, u8>;
///Field `WUCKSEL` writer - Wakeup clock selection
pub type WUCKSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `TSEDGE` reader - Time-stamp event active edge
pub type TSEDGE_R = crate::BitReader<bool>;
///Field `TSEDGE` writer - Time-stamp event active edge
pub type TSEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `REFCKON` reader - Reference clock detection enable (50 or 60 Hz)
pub type REFCKON_R = crate::BitReader<bool>;
///Field `REFCKON` writer - Reference clock detection enable (50 or 60 Hz)
pub type REFCKON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `BYPSHAD` reader - Bypass the shadow registers
pub type BYPSHAD_R = crate::BitReader<bool>;
///Field `BYPSHAD` writer - Bypass the shadow registers
pub type BYPSHAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `FMT` reader - Hour format
pub type FMT_R = crate::BitReader<bool>;
///Field `FMT` writer - Hour format
pub type FMT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ALRAE` reader - Alarm A enable
pub type ALRAE_R = crate::BitReader<bool>;
///Field `ALRAE` writer - Alarm A enable
pub type ALRAE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ALRBE` reader - Alarm B enable
pub type ALRBE_R = crate::BitReader<bool>;
///Field `ALRBE` writer - Alarm B enable
pub type ALRBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `WUTE` reader - Wakeup timer enable
pub type WUTE_R = crate::BitReader<bool>;
///Field `WUTE` writer - Wakeup timer enable
pub type WUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TSE` reader - Time stamp enable
pub type TSE_R = crate::BitReader<bool>;
///Field `TSE` writer - Time stamp enable
pub type TSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ALRAIE` reader - Alarm A interrupt enable
pub type ALRAIE_R = crate::BitReader<bool>;
///Field `ALRAIE` writer - Alarm A interrupt enable
pub type ALRAIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ALRBIE` reader - Alarm B interrupt enable
pub type ALRBIE_R = crate::BitReader<bool>;
///Field `ALRBIE` writer - Alarm B interrupt enable
pub type ALRBIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `WUTIE` reader - Wakeup timer interrupt enable
pub type WUTIE_R = crate::BitReader<bool>;
///Field `WUTIE` writer - Wakeup timer interrupt enable
pub type WUTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TSIE` reader - Time-stamp interrupt enable
pub type TSIE_R = crate::BitReader<bool>;
///Field `TSIE` writer - Time-stamp interrupt enable
pub type TSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ADD1H` reader - Add 1 hour (summer time change)
pub type ADD1H_R = crate::BitReader<bool>;
///Field `ADD1H` writer - Add 1 hour (summer time change)
pub type ADD1H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `SUB1H` reader - Subtract 1 hour (winter time change)
pub type SUB1H_R = crate::BitReader<bool>;
///Field `SUB1H` writer - Subtract 1 hour (winter time change)
pub type SUB1H_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `BKP` reader - Backup
pub type BKP_R = crate::BitReader<bool>;
///Field `BKP` writer - Backup
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `COSEL` reader - Calibration output selection
pub type COSEL_R = crate::BitReader<bool>;
///Field `COSEL` writer - Calibration output selection
pub type COSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `POL` reader - Output polarity
pub type POL_R = crate::BitReader<bool>;
///Field `POL` writer - Output polarity
pub type POL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `OSEL` reader - Output selection
pub type OSEL_R = crate::FieldReader<u8, u8>;
///Field `OSEL` writer - Output selection
pub type OSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `COE` reader - Calibration output enable
pub type COE_R = crate::BitReader<bool>;
///Field `COE` writer - Calibration output enable
pub type COE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ITSE` reader - timestamp on internal event enable
pub type ITSE_R = crate::BitReader<bool>;
///Field `ITSE` writer - timestamp on internal event enable
pub type ITSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bits 0:2 - Wakeup clock selection
    #[inline(always)]
    pub fn wucksel(&self) -> WUCKSEL_R {
        WUCKSEL_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Time-stamp event active edge
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bypass the shadow registers
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    pub fn alrbe(&self) -> ALRBE_R {
        ALRBE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Wakeup timer enable
    #[inline(always)]
    pub fn wute(&self) -> WUTE_R {
        WUTE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Time stamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    pub fn alrbie(&self) -> ALRBIE_R {
        ALRBIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    pub fn wutie(&self) -> WUTIE_R {
        WUTIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Add 1 hour (summer time change)
    #[inline(always)]
    pub fn add1h(&self) -> ADD1H_R {
        ADD1H_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Subtract 1 hour (winter time change)
    #[inline(always)]
    pub fn sub1h(&self) -> SUB1H_R {
        SUB1H_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Calibration output selection
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - timestamp on internal event enable
    #[inline(always)]
    pub fn itse(&self) -> ITSE_R {
        ITSE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    ///Bits 0:2 - Wakeup clock selection
    #[inline(always)]
    #[must_use]
    pub fn wucksel(&mut self) -> WUCKSEL_W<0> {
        WUCKSEL_W::new(self)
    }
    ///Bit 3 - Time-stamp event active edge
    #[inline(always)]
    #[must_use]
    pub fn tsedge(&mut self) -> TSEDGE_W<3> {
        TSEDGE_W::new(self)
    }
    ///Bit 4 - Reference clock detection enable (50 or 60 Hz)
    #[inline(always)]
    #[must_use]
    pub fn refckon(&mut self) -> REFCKON_W<4> {
        REFCKON_W::new(self)
    }
    ///Bit 5 - Bypass the shadow registers
    #[inline(always)]
    #[must_use]
    pub fn bypshad(&mut self) -> BYPSHAD_W<5> {
        BYPSHAD_W::new(self)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    #[must_use]
    pub fn fmt(&mut self) -> FMT_W<6> {
        FMT_W::new(self)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    #[must_use]
    pub fn alrae(&mut self) -> ALRAE_W<8> {
        ALRAE_W::new(self)
    }
    ///Bit 9 - Alarm B enable
    #[inline(always)]
    #[must_use]
    pub fn alrbe(&mut self) -> ALRBE_W<9> {
        ALRBE_W::new(self)
    }
    ///Bit 10 - Wakeup timer enable
    #[inline(always)]
    #[must_use]
    pub fn wute(&mut self) -> WUTE_W<10> {
        WUTE_W::new(self)
    }
    ///Bit 11 - Time stamp enable
    #[inline(always)]
    #[must_use]
    pub fn tse(&mut self) -> TSE_W<11> {
        TSE_W::new(self)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn alraie(&mut self) -> ALRAIE_W<12> {
        ALRAIE_W::new(self)
    }
    ///Bit 13 - Alarm B interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn alrbie(&mut self) -> ALRBIE_W<13> {
        ALRBIE_W::new(self)
    }
    ///Bit 14 - Wakeup timer interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn wutie(&mut self) -> WUTIE_W<14> {
        WUTIE_W::new(self)
    }
    ///Bit 15 - Time-stamp interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn tsie(&mut self) -> TSIE_W<15> {
        TSIE_W::new(self)
    }
    ///Bit 16 - Add 1 hour (summer time change)
    #[inline(always)]
    #[must_use]
    pub fn add1h(&mut self) -> ADD1H_W<16> {
        ADD1H_W::new(self)
    }
    ///Bit 17 - Subtract 1 hour (winter time change)
    #[inline(always)]
    #[must_use]
    pub fn sub1h(&mut self) -> SUB1H_W<17> {
        SUB1H_W::new(self)
    }
    ///Bit 18 - Backup
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<18> {
        BKP_W::new(self)
    }
    ///Bit 19 - Calibration output selection
    #[inline(always)]
    #[must_use]
    pub fn cosel(&mut self) -> COSEL_W<19> {
        COSEL_W::new(self)
    }
    ///Bit 20 - Output polarity
    #[inline(always)]
    #[must_use]
    pub fn pol(&mut self) -> POL_W<20> {
        POL_W::new(self)
    }
    ///Bits 21:22 - Output selection
    #[inline(always)]
    #[must_use]
    pub fn osel(&mut self) -> OSEL_W<21> {
        OSEL_W::new(self)
    }
    ///Bit 23 - Calibration output enable
    #[inline(always)]
    #[must_use]
    pub fn coe(&mut self) -> COE_W<23> {
        COE_W::new(self)
    }
    ///Bit 24 - timestamp on internal event enable
    #[inline(always)]
    #[must_use]
    pub fn itse(&mut self) -> ITSE_W<24> {
        ITSE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
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
