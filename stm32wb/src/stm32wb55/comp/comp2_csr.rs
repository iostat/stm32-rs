///Register `COMP2_CSR` reader
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COMP2_CSR` writer
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `COMP2_EN` reader - Comparator 2 enable bit
pub type COMP2_EN_R = crate::BitReader<bool>;
///Field `COMP2_EN` writer - Comparator 2 enable bit
pub type COMP2_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `COMP2_PWRMODE` reader - Power Mode of the comparator 2
pub type COMP2_PWRMODE_R = crate::FieldReader<u8, u8>;
///Field `COMP2_PWRMODE` writer - Power Mode of the comparator 2
pub type COMP2_PWRMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP2_INMSEL` reader - Comparator 2 input minus selection bits
pub type COMP2_INMSEL_R = crate::FieldReader<u8, u8>;
///Field `COMP2_INMSEL` writer - Comparator 2 input minus selection bits
pub type COMP2_INMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP2_INPSEL` reader - Comparator 1 input plus selection bit
pub type COMP2_INPSEL_R = crate::FieldReader<u8, u8>;
///Field `COMP2_INPSEL` writer - Comparator 1 input plus selection bit
pub type COMP2_INPSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP2_WINMODE` reader - Windows mode selection bit
pub type COMP2_WINMODE_R = crate::BitReader<bool>;
///Field `COMP2_WINMODE` writer - Windows mode selection bit
pub type COMP2_WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `COMP2_POLARITY` reader - Comparator 2 polarity selection bit
pub type COMP2_POLARITY_R = crate::BitReader<bool>;
///Field `COMP2_POLARITY` writer - Comparator 2 polarity selection bit
pub type COMP2_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `COMP2_HYST` reader - Comparator 2 hysteresis selection bits
pub type COMP2_HYST_R = crate::FieldReader<u8, u8>;
///Field `COMP2_HYST` writer - Comparator 2 hysteresis selection bits
pub type COMP2_HYST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP2_BLANKING` reader - Comparator 2 blanking source selection bits
pub type COMP2_BLANKING_R = crate::FieldReader<u8, u8>;
///Field `COMP2_BLANKING` writer - Comparator 2 blanking source selection bits
pub type COMP2_BLANKING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 3, O>;
///Field `COMP2_BRGEN` reader - Scaler bridge enable
pub type COMP2_BRGEN_R = crate::BitReader<bool>;
///Field `COMP2_BRGEN` writer - Scaler bridge enable
pub type COMP2_BRGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `COMP2_SCALEN` reader - Voltage scaler enable bit
pub type COMP2_SCALEN_R = crate::BitReader<bool>;
///Field `COMP2_SCALEN` writer - Voltage scaler enable bit
pub type COMP2_SCALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
///Field `COMP2_INMESEL` reader - comparator 2 input minus extended selection bits.
pub type COMP2_INMESEL_R = crate::FieldReader<u8, u8>;
///Field `COMP2_INMESEL` writer - comparator 2 input minus extended selection bits.
pub type COMP2_INMESEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, COMP2_CSR_SPEC, u8, u8, 2, O>;
///Field `COMP2_VALUE` reader - Comparator 2 output status bit
pub type COMP2_VALUE_R = crate::BitReader<bool>;
///Field `COMP2_LOCK` reader - CSR register lock bit
pub type COMP2_LOCK_R = crate::BitReader<bool>;
///Field `COMP2_LOCK` writer - CSR register lock bit
pub type COMP2_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, COMP2_CSR_SPEC, bool, O>;
impl R {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    pub fn comp2_en(&self) -> COMP2_EN_R {
        COMP2_EN_R::new((self.bits & 1) != 0)
    }
    ///Bits 2:3 - Power Mode of the comparator 2
    #[inline(always)]
    pub fn comp2_pwrmode(&self) -> COMP2_PWRMODE_R {
        COMP2_PWRMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Comparator 2 input minus selection bits
    #[inline(always)]
    pub fn comp2_inmsel(&self) -> COMP2_INMSEL_R {
        COMP2_INMSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 7:8 - Comparator 1 input plus selection bit
    #[inline(always)]
    pub fn comp2_inpsel(&self) -> COMP2_INPSEL_R {
        COMP2_INPSEL_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///Bit 9 - Windows mode selection bit
    #[inline(always)]
    pub fn comp2_winmode(&self) -> COMP2_WINMODE_R {
        COMP2_WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    pub fn comp2_polarity(&self) -> COMP2_POLARITY_R {
        COMP2_POLARITY_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    pub fn comp2_hyst(&self) -> COMP2_HYST_R {
        COMP2_HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Comparator 2 blanking source selection bits
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 22 - Scaler bridge enable
    #[inline(always)]
    pub fn comp2_brgen(&self) -> COMP2_BRGEN_R {
        COMP2_BRGEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Voltage scaler enable bit
    #[inline(always)]
    pub fn comp2_scalen(&self) -> COMP2_SCALEN_R {
        COMP2_SCALEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 25:26 - comparator 2 input minus extended selection bits.
    #[inline(always)]
    pub fn comp2_inmesel(&self) -> COMP2_INMESEL_R {
        COMP2_INMESEL_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 30 - Comparator 2 output status bit
    #[inline(always)]
    pub fn comp2_value(&self) -> COMP2_VALUE_R {
        COMP2_VALUE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CSR register lock bit
    #[inline(always)]
    pub fn comp2_lock(&self) -> COMP2_LOCK_R {
        COMP2_LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Comparator 2 enable bit
    #[inline(always)]
    #[must_use]
    pub fn comp2_en(&mut self) -> COMP2_EN_W<0> {
        COMP2_EN_W::new(self)
    }
    ///Bits 2:3 - Power Mode of the comparator 2
    #[inline(always)]
    #[must_use]
    pub fn comp2_pwrmode(&mut self) -> COMP2_PWRMODE_W<2> {
        COMP2_PWRMODE_W::new(self)
    }
    ///Bits 4:5 - Comparator 2 input minus selection bits
    #[inline(always)]
    #[must_use]
    pub fn comp2_inmsel(&mut self) -> COMP2_INMSEL_W<4> {
        COMP2_INMSEL_W::new(self)
    }
    ///Bits 7:8 - Comparator 1 input plus selection bit
    #[inline(always)]
    #[must_use]
    pub fn comp2_inpsel(&mut self) -> COMP2_INPSEL_W<7> {
        COMP2_INPSEL_W::new(self)
    }
    ///Bit 9 - Windows mode selection bit
    #[inline(always)]
    #[must_use]
    pub fn comp2_winmode(&mut self) -> COMP2_WINMODE_W<9> {
        COMP2_WINMODE_W::new(self)
    }
    ///Bit 15 - Comparator 2 polarity selection bit
    #[inline(always)]
    #[must_use]
    pub fn comp2_polarity(&mut self) -> COMP2_POLARITY_W<15> {
        COMP2_POLARITY_W::new(self)
    }
    ///Bits 16:17 - Comparator 2 hysteresis selection bits
    #[inline(always)]
    #[must_use]
    pub fn comp2_hyst(&mut self) -> COMP2_HYST_W<16> {
        COMP2_HYST_W::new(self)
    }
    ///Bits 18:20 - Comparator 2 blanking source selection bits
    #[inline(always)]
    #[must_use]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W<18> {
        COMP2_BLANKING_W::new(self)
    }
    ///Bit 22 - Scaler bridge enable
    #[inline(always)]
    #[must_use]
    pub fn comp2_brgen(&mut self) -> COMP2_BRGEN_W<22> {
        COMP2_BRGEN_W::new(self)
    }
    ///Bit 23 - Voltage scaler enable bit
    #[inline(always)]
    #[must_use]
    pub fn comp2_scalen(&mut self) -> COMP2_SCALEN_W<23> {
        COMP2_SCALEN_W::new(self)
    }
    ///Bits 25:26 - comparator 2 input minus extended selection bits.
    #[inline(always)]
    #[must_use]
    pub fn comp2_inmesel(&mut self) -> COMP2_INMESEL_W<25> {
        COMP2_INMESEL_W::new(self)
    }
    ///Bit 31 - CSR register lock bit
    #[inline(always)]
    #[must_use]
    pub fn comp2_lock(&mut self) -> COMP2_LOCK_W<31> {
        COMP2_LOCK_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Comparator 2 control and status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [comp2_csr](index.html) module
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [comp2_csr::R](R) reader structure
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [comp2_csr::W](W) writer structure
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets COMP2_CSR to value 0
impl crate::Resettable for COMP2_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
