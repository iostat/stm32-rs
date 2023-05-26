///Register `C2SCR` writer
pub struct W(crate::W<C2SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<C2SCR_SPEC>;
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
impl From<crate::W<C2SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<C2SCR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CH1C` writer - processor 2 Receive channel 1 status clear
pub type CH1C_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH2C` writer - processor 2 Receive channel 2 status clear
pub type CH2C_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH3C` writer - processor 2 Receive channel 3 status clear
pub type CH3C_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH4C` writer - processor 2 Receive channel 4 status clear
pub type CH4C_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH5C` writer - processor 2 Receive channel 5 status clear
pub type CH5C_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH6C` writer - processor 2 Receive channel 6 status clear
pub type CH6C_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH1S` writer - processor 2 Transmit channel 1 status set
pub type CH1S_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH2S` writer - processor 2 Transmit channel 2 status set
pub type CH2S_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH3S` writer - processor 2 Transmit channel 3 status set
pub type CH3S_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH4S` writer - processor 2 Transmit channel 4 status set
pub type CH4S_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH5S` writer - processor 2 Transmit channel 5 status set
pub type CH5S_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
///Field `CH6S` writer - processor 2 Transmit channel 6 status set
pub type CH6S_W<'a, const O: u8> = crate::BitWriter<'a, u32, C2SCR_SPEC, bool, O>;
impl W {
    ///Bit 0 - processor 2 Receive channel 1 status clear
    #[inline(always)]
    #[must_use]
    pub fn ch1c(&mut self) -> CH1C_W<0> {
        CH1C_W::new(self)
    }
    ///Bit 1 - processor 2 Receive channel 2 status clear
    #[inline(always)]
    #[must_use]
    pub fn ch2c(&mut self) -> CH2C_W<1> {
        CH2C_W::new(self)
    }
    ///Bit 2 - processor 2 Receive channel 3 status clear
    #[inline(always)]
    #[must_use]
    pub fn ch3c(&mut self) -> CH3C_W<2> {
        CH3C_W::new(self)
    }
    ///Bit 3 - processor 2 Receive channel 4 status clear
    #[inline(always)]
    #[must_use]
    pub fn ch4c(&mut self) -> CH4C_W<3> {
        CH4C_W::new(self)
    }
    ///Bit 4 - processor 2 Receive channel 5 status clear
    #[inline(always)]
    #[must_use]
    pub fn ch5c(&mut self) -> CH5C_W<4> {
        CH5C_W::new(self)
    }
    ///Bit 5 - processor 2 Receive channel 6 status clear
    #[inline(always)]
    #[must_use]
    pub fn ch6c(&mut self) -> CH6C_W<5> {
        CH6C_W::new(self)
    }
    ///Bit 16 - processor 2 Transmit channel 1 status set
    #[inline(always)]
    #[must_use]
    pub fn ch1s(&mut self) -> CH1S_W<16> {
        CH1S_W::new(self)
    }
    ///Bit 17 - processor 2 Transmit channel 2 status set
    #[inline(always)]
    #[must_use]
    pub fn ch2s(&mut self) -> CH2S_W<17> {
        CH2S_W::new(self)
    }
    ///Bit 18 - processor 2 Transmit channel 3 status set
    #[inline(always)]
    #[must_use]
    pub fn ch3s(&mut self) -> CH3S_W<18> {
        CH3S_W::new(self)
    }
    ///Bit 19 - processor 2 Transmit channel 4 status set
    #[inline(always)]
    #[must_use]
    pub fn ch4s(&mut self) -> CH4S_W<19> {
        CH4S_W::new(self)
    }
    ///Bit 20 - processor 2 Transmit channel 5 status set
    #[inline(always)]
    #[must_use]
    pub fn ch5s(&mut self) -> CH5S_W<20> {
        CH5S_W::new(self)
    }
    ///Bit 21 - processor 2 Transmit channel 6 status set
    #[inline(always)]
    #[must_use]
    pub fn ch6s(&mut self) -> CH6S_W<21> {
        CH6S_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Status Set or Clear register CPU2
///
///This register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [c2scr](index.html) module
pub struct C2SCR_SPEC;
impl crate::RegisterSpec for C2SCR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [c2scr::W](W) writer structure
impl crate::Writable for C2SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets C2SCR to value 0
impl crate::Resettable for C2SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
