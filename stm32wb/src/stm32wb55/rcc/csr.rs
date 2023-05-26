///Register `CSR` reader
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CSR` writer
pub struct W(crate::W<CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR_SPEC>;
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
impl From<crate::W<CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LSI1ON` reader - LSI1 oscillator enabled
pub type LSI1ON_R = crate::BitReader<bool>;
///Field `LSI1ON` writer - LSI1 oscillator enabled
pub type LSI1ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `LSI1RDY` reader - LSI1 oscillator ready
pub type LSI1RDY_R = crate::BitReader<bool>;
///Field `LSI2ON` reader - LSI2 oscillator enabled
pub type LSI2ON_R = crate::BitReader<bool>;
///Field `LSI2ON` writer - LSI2 oscillator enabled
pub type LSI2ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `LSI2RDY` reader - LSI2 oscillator ready
pub type LSI2RDY_R = crate::BitReader<bool>;
///Field `LSI2TRIMEN` reader - LSI2 oscillator trimming enable
pub type LSI2TRIMEN_R = crate::BitReader<bool>;
///Field `LSI2TRIMEN` writer - LSI2 oscillator trimming enable
pub type LSI2TRIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `LSI2TRIMOK` reader - LSI2 oscillator trim OK
pub type LSI2TRIMOK_R = crate::BitReader<bool>;
///Field `LSI2BW` reader - LSI2 oscillator bias configuration
pub type LSI2BW_R = crate::FieldReader<u8, u8>;
///Field `LSI2BW` writer - LSI2 oscillator bias configuration
pub type LSI2BW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 4, O>;
///Field `RFWKPSEL` reader - RF system wakeup clock source selection
pub type RFWKPSEL_R = crate::FieldReader<u8, u8>;
///Field `RFWKPSEL` writer - RF system wakeup clock source selection
pub type RFWKPSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CSR_SPEC, u8, u8, 2, O>;
///Field `RFRSTS` reader - Radio system BLE and 802.15.4 reset status
pub type RFRSTS_R = crate::BitReader<bool>;
///Field `RMVF` reader - Remove reset flag
pub type RMVF_R = crate::BitReader<bool>;
///Field `RMVF` writer - Remove reset flag
pub type RMVF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CSR_SPEC, bool, O>;
///Field `OBLRSTF` reader - Option byte loader reset flag
pub type OBLRSTF_R = crate::BitReader<bool>;
///Field `PINRSTF` reader - Pin reset flag
pub type PINRSTF_R = crate::BitReader<bool>;
///Field `BORRSTF` reader - BOR flag
pub type BORRSTF_R = crate::BitReader<bool>;
///Field `SFTRSTF` reader - Software reset flag
pub type SFTRSTF_R = crate::BitReader<bool>;
///Field `IWDGRSTF` reader - Independent window watchdog reset flag
pub type IWDGRSTF_R = crate::BitReader<bool>;
///Field `WWDGRSTF` reader - Window watchdog reset flag
pub type WWDGRSTF_R = crate::BitReader<bool>;
///Field `LPWRRSTF` reader - Low-power reset flag
pub type LPWRRSTF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - LSI1 oscillator enabled
    #[inline(always)]
    pub fn lsi1on(&self) -> LSI1ON_R {
        LSI1ON_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSI1 oscillator ready
    #[inline(always)]
    pub fn lsi1rdy(&self) -> LSI1RDY_R {
        LSI1RDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LSI2 oscillator enabled
    #[inline(always)]
    pub fn lsi2on(&self) -> LSI2ON_R {
        LSI2ON_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LSI2 oscillator ready
    #[inline(always)]
    pub fn lsi2rdy(&self) -> LSI2RDY_R {
        LSI2RDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LSI2 oscillator trimming enable
    #[inline(always)]
    pub fn lsi2trimen(&self) -> LSI2TRIMEN_R {
        LSI2TRIMEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LSI2 oscillator trim OK
    #[inline(always)]
    pub fn lsi2trimok(&self) -> LSI2TRIMOK_R {
        LSI2TRIMOK_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 8:11 - LSI2 oscillator bias configuration
    #[inline(always)]
    pub fn lsi2bw(&self) -> LSI2BW_R {
        LSI2BW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 14:15 - RF system wakeup clock source selection
    #[inline(always)]
    pub fn rfwkpsel(&self) -> RFWKPSEL_R {
        RFWKPSEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 16 - Radio system BLE and 802.15.4 reset status
    #[inline(always)]
    pub fn rfrsts(&self) -> RFRSTS_R {
        RFRSTS_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    pub fn rmvf(&self) -> RMVF_R {
        RMVF_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - Option byte loader reset flag
    #[inline(always)]
    pub fn oblrstf(&self) -> OBLRSTF_R {
        OBLRSTF_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Pin reset flag
    #[inline(always)]
    pub fn pinrstf(&self) -> PINRSTF_R {
        PINRSTF_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - BOR flag
    #[inline(always)]
    pub fn borrstf(&self) -> BORRSTF_R {
        BORRSTF_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Software reset flag
    #[inline(always)]
    pub fn sftrstf(&self) -> SFTRSTF_R {
        SFTRSTF_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - Independent window watchdog reset flag
    #[inline(always)]
    pub fn iwdgrstf(&self) -> IWDGRSTF_R {
        IWDGRSTF_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Window watchdog reset flag
    #[inline(always)]
    pub fn wwdgrstf(&self) -> WWDGRSTF_R {
        WWDGRSTF_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low-power reset flag
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LPWRRSTF_R {
        LPWRRSTF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LSI1 oscillator enabled
    #[inline(always)]
    #[must_use]
    pub fn lsi1on(&mut self) -> LSI1ON_W<0> {
        LSI1ON_W::new(self)
    }
    ///Bit 2 - LSI2 oscillator enabled
    #[inline(always)]
    #[must_use]
    pub fn lsi2on(&mut self) -> LSI2ON_W<2> {
        LSI2ON_W::new(self)
    }
    ///Bit 4 - LSI2 oscillator trimming enable
    #[inline(always)]
    #[must_use]
    pub fn lsi2trimen(&mut self) -> LSI2TRIMEN_W<4> {
        LSI2TRIMEN_W::new(self)
    }
    ///Bits 8:11 - LSI2 oscillator bias configuration
    #[inline(always)]
    #[must_use]
    pub fn lsi2bw(&mut self) -> LSI2BW_W<8> {
        LSI2BW_W::new(self)
    }
    ///Bits 14:15 - RF system wakeup clock source selection
    #[inline(always)]
    #[must_use]
    pub fn rfwkpsel(&mut self) -> RFWKPSEL_W<14> {
        RFWKPSEL_W::new(self)
    }
    ///Bit 23 - Remove reset flag
    #[inline(always)]
    #[must_use]
    pub fn rmvf(&mut self) -> RMVF_W<23> {
        RMVF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CSR
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [csr](index.html) module
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [csr::R](R) reader structure
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [csr::W](W) writer structure
impl crate::Writable for CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CSR to value 0x0c00_0000
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c00_0000;
}
