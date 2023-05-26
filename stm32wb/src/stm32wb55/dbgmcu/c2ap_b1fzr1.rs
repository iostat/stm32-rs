///Register `C2AP_B1FZR1` reader
pub struct R(crate::R<C2AP_B1FZR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<C2AP_B1FZR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<C2AP_B1FZR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<C2AP_B1FZR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `C2AP_B1FZR1` writer
pub struct W(crate::W<C2AP_B1FZR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2AP_B1FZR1_SPEC>;
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
impl From<crate::W<C2AP_B1FZR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2AP_B1FZR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DBG_LPTIM2_STOP` reader - LPTIM2 counter stopped when core is halted
pub type DBG_LPTIM2_STOP_R = crate::BitReader<bool>;
///Field `DBG_LPTIM2_STOP` writer - LPTIM2 counter stopped when core is halted
pub type DBG_LPTIM2_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AP_B1FZR1_SPEC, bool, O>;
///Field `DBG_RTC_STOP` reader - RTC counter stopped when core is halted
pub type DBG_RTC_STOP_R = crate::BitReader<bool>;
///Field `DBG_RTC_STOP` writer - RTC counter stopped when core is halted
pub type DBG_RTC_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AP_B1FZR1_SPEC, bool, O>;
///Field `DBG_IWDG_STOP` reader - IWDG stopped when core is halted
pub type DBG_IWDG_STOP_R = crate::BitReader<bool>;
///Field `DBG_IWDG_STOP` writer - IWDG stopped when core is halted
pub type DBG_IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AP_B1FZR1_SPEC, bool, O>;
///Field `DBG_I2C1_STOP` reader - I2C1 SMBUS timeout stopped when core is halted
pub type DBG_I2C1_STOP_R = crate::BitReader<bool>;
///Field `DBG_I2C1_STOP` writer - I2C1 SMBUS timeout stopped when core is halted
pub type DBG_I2C1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AP_B1FZR1_SPEC, bool, O>;
///Field `DBG_I2C3_STOP` reader - I2C3 SMBUS timeout stopped when core is halted
pub type DBG_I2C3_STOP_R = crate::BitReader<bool>;
///Field `DBG_I2C3_STOP` writer - I2C3 SMBUS timeout stopped when core is halted
pub type DBG_I2C3_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AP_B1FZR1_SPEC, bool, O>;
///Field `DBG_LPTIM1_STOP` reader - LPTIM1 counter stopped when core is halted
pub type DBG_LPTIM1_STOP_R = crate::BitReader<bool>;
///Field `DBG_LPTIM1_STOP` writer - LPTIM1 counter stopped when core is halted
pub type DBG_LPTIM1_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2AP_B1FZR1_SPEC, bool, O>;
impl R {
    ///Bit 0 - LPTIM2 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptim2_stop(&self) -> DBG_LPTIM2_STOP_R {
        DBG_LPTIM2_STOP_R::new((self.bits & 1) != 0)
    }
    ///Bit 10 - RTC counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_rtc_stop(&self) -> DBG_RTC_STOP_R {
        DBG_RTC_STOP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - IWDG stopped when core is halted
    #[inline(always)]
    pub fn dbg_iwdg_stop(&self) -> DBG_IWDG_STOP_R {
        DBG_IWDG_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 21 - I2C1 SMBUS timeout stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c1_stop(&self) -> DBG_I2C1_STOP_R {
        DBG_I2C1_STOP_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 23 - I2C3 SMBUS timeout stopped when core is halted
    #[inline(always)]
    pub fn dbg_i2c3_stop(&self) -> DBG_I2C3_STOP_R {
        DBG_I2C3_STOP_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    pub fn dbg_lptim1_stop(&self) -> DBG_LPTIM1_STOP_R {
        DBG_LPTIM1_STOP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LPTIM2 counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim2_stop(&mut self) -> DBG_LPTIM2_STOP_W<0> {
        DBG_LPTIM2_STOP_W::new(self)
    }
    ///Bit 10 - RTC counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_rtc_stop(&mut self) -> DBG_RTC_STOP_W<10> {
        DBG_RTC_STOP_W::new(self)
    }
    ///Bit 12 - IWDG stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_iwdg_stop(&mut self) -> DBG_IWDG_STOP_W<12> {
        DBG_IWDG_STOP_W::new(self)
    }
    ///Bit 21 - I2C1 SMBUS timeout stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c1_stop(&mut self) -> DBG_I2C1_STOP_W<21> {
        DBG_I2C1_STOP_W::new(self)
    }
    ///Bit 23 - I2C3 SMBUS timeout stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_i2c3_stop(&mut self) -> DBG_I2C3_STOP_W<23> {
        DBG_I2C3_STOP_W::new(self)
    }
    ///Bit 31 - LPTIM1 counter stopped when core is halted
    #[inline(always)]
    #[must_use]
    pub fn dbg_lptim1_stop(&mut self) -> DBG_LPTIM1_STOP_W<31> {
        DBG_LPTIM1_STOP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///APB1 Low Freeze Register CPU2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2ap_b1fzr1](index.html) module
pub struct C2AP_B1FZR1_SPEC;
impl crate::RegisterSpec for C2AP_B1FZR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [c2ap_b1fzr1::R](R) reader structure
impl crate::Readable for C2AP_B1FZR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [c2ap_b1fzr1::W](W) writer structure
impl crate::Writable for C2AP_B1FZR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2AP_B1FZR1 to value 0
impl crate::Resettable for C2AP_B1FZR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
