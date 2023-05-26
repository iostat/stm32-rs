///Register `IER` reader
pub struct R(crate::R<IER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IER_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IER` writer
pub struct W(crate::W<IER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IER_SPEC>;
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
impl From<crate::W<IER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IER_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADRDYIE` reader - ADC ready interrupt
pub type ADRDYIE_R = crate::BitReader<bool>;
///Field `ADRDYIE` writer - ADC ready interrupt
pub type ADRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `EOSMPIE` reader - ADC group regular end of sampling interrupt
pub type EOSMPIE_R = crate::BitReader<bool>;
///Field `EOSMPIE` writer - ADC group regular end of sampling interrupt
pub type EOSMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `EOCIE` reader - ADC group regular end of unitary conversion interrupt
pub type EOCIE_R = crate::BitReader<bool>;
///Field `EOCIE` writer - ADC group regular end of unitary conversion interrupt
pub type EOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `EOSIE` reader - ADC group regular end of sequence conversions interrupt
pub type EOSIE_R = crate::BitReader<bool>;
///Field `EOSIE` writer - ADC group regular end of sequence conversions interrupt
pub type EOSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `OVRIE` reader - ADC group regular overrun interrupt
pub type OVRIE_R = crate::BitReader<bool>;
///Field `OVRIE` writer - ADC group regular overrun interrupt
pub type OVRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `JEOCIE` reader - ADC group injected end of unitary conversion interrupt
pub type JEOCIE_R = crate::BitReader<bool>;
///Field `JEOCIE` writer - ADC group injected end of unitary conversion interrupt
pub type JEOCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `JEOSIE` reader - ADC group injected end of sequence conversions interrupt
pub type JEOSIE_R = crate::BitReader<bool>;
///Field `JEOSIE` writer - ADC group injected end of sequence conversions interrupt
pub type JEOSIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `AWD1IE` reader - ADC analog watchdog 1 interrupt
pub type AWD1IE_R = crate::BitReader<bool>;
///Field `AWD1IE` writer - ADC analog watchdog 1 interrupt
pub type AWD1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `AWD2IE` reader - ADC analog watchdog 2 interrupt
pub type AWD2IE_R = crate::BitReader<bool>;
///Field `AWD2IE` writer - ADC analog watchdog 2 interrupt
pub type AWD2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `AWD3IE` reader - ADC analog watchdog 3 interrupt
pub type AWD3IE_R = crate::BitReader<bool>;
///Field `AWD3IE` writer - ADC analog watchdog 3 interrupt
pub type AWD3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
///Field `JQOVFIE` reader - ADC group injected contexts queue overflow interrupt
pub type JQOVFIE_R = crate::BitReader<bool>;
///Field `JQOVFIE` writer - ADC group injected contexts queue overflow interrupt
pub type JQOVFIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IER_SPEC, bool, O>;
impl R {
    ///Bit 0 - ADC ready interrupt
    #[inline(always)]
    pub fn adrdyie(&self) -> ADRDYIE_R {
        ADRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ADC group regular end of sampling interrupt
    #[inline(always)]
    pub fn eosmpie(&self) -> EOSMPIE_R {
        EOSMPIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ADC group regular end of unitary conversion interrupt
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ADC group regular end of sequence conversions interrupt
    #[inline(always)]
    pub fn eosie(&self) -> EOSIE_R {
        EOSIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC group regular overrun interrupt
    #[inline(always)]
    pub fn ovrie(&self) -> OVRIE_R {
        OVRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ADC group injected end of unitary conversion interrupt
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ADC group injected end of sequence conversions interrupt
    #[inline(always)]
    pub fn jeosie(&self) -> JEOSIE_R {
        JEOSIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ADC analog watchdog 1 interrupt
    #[inline(always)]
    pub fn awd1ie(&self) -> AWD1IE_R {
        AWD1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ADC analog watchdog 2 interrupt
    #[inline(always)]
    pub fn awd2ie(&self) -> AWD2IE_R {
        AWD2IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - ADC analog watchdog 3 interrupt
    #[inline(always)]
    pub fn awd3ie(&self) -> AWD3IE_R {
        AWD3IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADC group injected contexts queue overflow interrupt
    #[inline(always)]
    pub fn jqovfie(&self) -> JQOVFIE_R {
        JQOVFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - ADC ready interrupt
    #[inline(always)]
    #[must_use]
    pub fn adrdyie(&mut self) -> ADRDYIE_W<0> {
        ADRDYIE_W::new(self)
    }
    ///Bit 1 - ADC group regular end of sampling interrupt
    #[inline(always)]
    #[must_use]
    pub fn eosmpie(&mut self) -> EOSMPIE_W<1> {
        EOSMPIE_W::new(self)
    }
    ///Bit 2 - ADC group regular end of unitary conversion interrupt
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<2> {
        EOCIE_W::new(self)
    }
    ///Bit 3 - ADC group regular end of sequence conversions interrupt
    #[inline(always)]
    #[must_use]
    pub fn eosie(&mut self) -> EOSIE_W<3> {
        EOSIE_W::new(self)
    }
    ///Bit 4 - ADC group regular overrun interrupt
    #[inline(always)]
    #[must_use]
    pub fn ovrie(&mut self) -> OVRIE_W<4> {
        OVRIE_W::new(self)
    }
    ///Bit 5 - ADC group injected end of unitary conversion interrupt
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<5> {
        JEOCIE_W::new(self)
    }
    ///Bit 6 - ADC group injected end of sequence conversions interrupt
    #[inline(always)]
    #[must_use]
    pub fn jeosie(&mut self) -> JEOSIE_W<6> {
        JEOSIE_W::new(self)
    }
    ///Bit 7 - ADC analog watchdog 1 interrupt
    #[inline(always)]
    #[must_use]
    pub fn awd1ie(&mut self) -> AWD1IE_W<7> {
        AWD1IE_W::new(self)
    }
    ///Bit 8 - ADC analog watchdog 2 interrupt
    #[inline(always)]
    #[must_use]
    pub fn awd2ie(&mut self) -> AWD2IE_W<8> {
        AWD2IE_W::new(self)
    }
    ///Bit 9 - ADC analog watchdog 3 interrupt
    #[inline(always)]
    #[must_use]
    pub fn awd3ie(&mut self) -> AWD3IE_W<9> {
        AWD3IE_W::new(self)
    }
    ///Bit 10 - ADC group injected contexts queue overflow interrupt
    #[inline(always)]
    #[must_use]
    pub fn jqovfie(&mut self) -> JQOVFIE_W<10> {
        JQOVFIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///ADC interrupt enable register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ier](index.html) module
pub struct IER_SPEC;
impl crate::RegisterSpec for IER_SPEC {
    type Ux = u32;
}
///`read()` method returns [ier::R](R) reader structure
impl crate::Readable for IER_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ier::W](W) writer structure
impl crate::Writable for IER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
