///Register `OPTR` reader
pub struct R(crate::R<OPTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OPTR` writer
pub struct W(crate::W<OPTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPTR_SPEC>;
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
impl From<crate::W<OPTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `RDP` reader - Read protection level
pub type RDP_R = crate::FieldReader<u8, u8>;
///Field `RDP` writer - Read protection level
pub type RDP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 8, O>;
///Field `ESE` reader - Security enabled
pub type ESE_R = crate::BitReader<bool>;
///Field `ESE` writer - Security enabled
pub type ESE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `BOR_LEV` reader - BOR reset Level
pub type BOR_LEV_R = crate::FieldReader<u8, u8>;
///Field `BOR_LEV` writer - BOR reset Level
pub type BOR_LEV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 3, O>;
///Field `nRST_STOP` reader - nRST_STOP
pub type N_RST_STOP_R = crate::BitReader<bool>;
///Field `nRST_STOP` writer - nRST_STOP
pub type N_RST_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nRST_STDBY` reader - nRST_STDBY
pub type N_RST_STDBY_R = crate::BitReader<bool>;
///Field `nRST_STDBY` writer - nRST_STDBY
pub type N_RST_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nRST_SHDW` reader - nRST_SHDW
pub type N_RST_SHDW_R = crate::BitReader<bool>;
///Field `nRST_SHDW` writer - nRST_SHDW
pub type N_RST_SHDW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `IDWG_SW` reader - Independent watchdog selection
pub type IDWG_SW_R = crate::BitReader<bool>;
///Field `IDWG_SW` writer - Independent watchdog selection
pub type IDWG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `IWDG_STOP` reader - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_R = crate::BitReader<bool>;
///Field `IWDG_STOP` writer - Independent watchdog counter freeze in Stop mode
pub type IWDG_STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `IWDG_STDBY` reader - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_R = crate::BitReader<bool>;
///Field `IWDG_STDBY` writer - Independent watchdog counter freeze in Standby mode
pub type IWDG_STDBY_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `WWDG_SW` reader - Window watchdog selection
pub type WWDG_SW_R = crate::BitReader<bool>;
///Field `WWDG_SW` writer - Window watchdog selection
pub type WWDG_SW_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nBOOT1` reader - Boot configuration
pub type N_BOOT1_R = crate::BitReader<bool>;
///Field `nBOOT1` writer - Boot configuration
pub type N_BOOT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `SRAM2_PE` reader - SRAM2 parity check enable
pub type SRAM2_PE_R = crate::BitReader<bool>;
///Field `SRAM2_PE` writer - SRAM2 parity check enable
pub type SRAM2_PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `SRAM2_RST` reader - SRAM2 Erase when system reset
pub type SRAM2_RST_R = crate::BitReader<bool>;
///Field `SRAM2_RST` writer - SRAM2 Erase when system reset
pub type SRAM2_RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nSWBOOT0` reader - Software Boot0
pub type N_SWBOOT0_R = crate::BitReader<bool>;
///Field `nSWBOOT0` writer - Software Boot0
pub type N_SWBOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `nBOOT0` reader - nBoot0 option bit
pub type N_BOOT0_R = crate::BitReader<bool>;
///Field `nBOOT0` writer - nBoot0 option bit
pub type N_BOOT0_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPTR_SPEC, bool, O>;
///Field `AGC_TRIM` reader - Radio Automatic Gain Control Trimming
pub type AGC_TRIM_R = crate::FieldReader<u8, u8>;
///Field `AGC_TRIM` writer - Radio Automatic Gain Control Trimming
pub type AGC_TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPTR_SPEC, u8, u8, 3, O>;
impl R {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    pub fn rdp(&self) -> RDP_R {
        RDP_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 8 - Security enabled
    #[inline(always)]
    pub fn ese(&self) -> ESE_R {
        ESE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:11 - BOR reset Level
    #[inline(always)]
    pub fn bor_lev(&self) -> BOR_LEV_R {
        BOR_LEV_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    pub fn n_rst_stop(&self) -> N_RST_STOP_R {
        N_RST_STOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    pub fn n_rst_stdby(&self) -> N_RST_STDBY_R {
        N_RST_STDBY_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - nRST_SHDW
    #[inline(always)]
    pub fn n_rst_shdw(&self) -> N_RST_SHDW_R {
        N_RST_SHDW_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    pub fn idwg_sw(&self) -> IDWG_SW_R {
        IDWG_SW_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    pub fn iwdg_stop(&self) -> IWDG_STOP_R {
        IWDG_STOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    pub fn iwdg_stdby(&self) -> IWDG_STDBY_R {
        IWDG_STDBY_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    pub fn wwdg_sw(&self) -> WWDG_SW_R {
        WWDG_SW_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    pub fn n_boot1(&self) -> N_BOOT1_R {
        N_BOOT1_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    pub fn sram2_pe(&self) -> SRAM2_PE_R {
        SRAM2_PE_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    pub fn sram2_rst(&self) -> SRAM2_RST_R {
        SRAM2_RST_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Software Boot0
    #[inline(always)]
    pub fn n_swboot0(&self) -> N_SWBOOT0_R {
        N_SWBOOT0_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - nBoot0 option bit
    #[inline(always)]
    pub fn n_boot0(&self) -> N_BOOT0_R {
        N_BOOT0_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 29:31 - Radio Automatic Gain Control Trimming
    #[inline(always)]
    pub fn agc_trim(&self) -> AGC_TRIM_R {
        AGC_TRIM_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    ///Bits 0:7 - Read protection level
    #[inline(always)]
    #[must_use]
    pub fn rdp(&mut self) -> RDP_W<0> {
        RDP_W::new(self)
    }
    ///Bit 8 - Security enabled
    #[inline(always)]
    #[must_use]
    pub fn ese(&mut self) -> ESE_W<8> {
        ESE_W::new(self)
    }
    ///Bits 9:11 - BOR reset Level
    #[inline(always)]
    #[must_use]
    pub fn bor_lev(&mut self) -> BOR_LEV_W<9> {
        BOR_LEV_W::new(self)
    }
    ///Bit 12 - nRST_STOP
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stop(&mut self) -> N_RST_STOP_W<12> {
        N_RST_STOP_W::new(self)
    }
    ///Bit 13 - nRST_STDBY
    #[inline(always)]
    #[must_use]
    pub fn n_rst_stdby(&mut self) -> N_RST_STDBY_W<13> {
        N_RST_STDBY_W::new(self)
    }
    ///Bit 14 - nRST_SHDW
    #[inline(always)]
    #[must_use]
    pub fn n_rst_shdw(&mut self) -> N_RST_SHDW_W<14> {
        N_RST_SHDW_W::new(self)
    }
    ///Bit 16 - Independent watchdog selection
    #[inline(always)]
    #[must_use]
    pub fn idwg_sw(&mut self) -> IDWG_SW_W<16> {
        IDWG_SW_W::new(self)
    }
    ///Bit 17 - Independent watchdog counter freeze in Stop mode
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stop(&mut self) -> IWDG_STOP_W<17> {
        IWDG_STOP_W::new(self)
    }
    ///Bit 18 - Independent watchdog counter freeze in Standby mode
    #[inline(always)]
    #[must_use]
    pub fn iwdg_stdby(&mut self) -> IWDG_STDBY_W<18> {
        IWDG_STDBY_W::new(self)
    }
    ///Bit 19 - Window watchdog selection
    #[inline(always)]
    #[must_use]
    pub fn wwdg_sw(&mut self) -> WWDG_SW_W<19> {
        WWDG_SW_W::new(self)
    }
    ///Bit 23 - Boot configuration
    #[inline(always)]
    #[must_use]
    pub fn n_boot1(&mut self) -> N_BOOT1_W<23> {
        N_BOOT1_W::new(self)
    }
    ///Bit 24 - SRAM2 parity check enable
    #[inline(always)]
    #[must_use]
    pub fn sram2_pe(&mut self) -> SRAM2_PE_W<24> {
        SRAM2_PE_W::new(self)
    }
    ///Bit 25 - SRAM2 Erase when system reset
    #[inline(always)]
    #[must_use]
    pub fn sram2_rst(&mut self) -> SRAM2_RST_W<25> {
        SRAM2_RST_W::new(self)
    }
    ///Bit 26 - Software Boot0
    #[inline(always)]
    #[must_use]
    pub fn n_swboot0(&mut self) -> N_SWBOOT0_W<26> {
        N_SWBOOT0_W::new(self)
    }
    ///Bit 27 - nBoot0 option bit
    #[inline(always)]
    #[must_use]
    pub fn n_boot0(&mut self) -> N_BOOT0_W<27> {
        N_BOOT0_W::new(self)
    }
    ///Bits 29:31 - Radio Automatic Gain Control Trimming
    #[inline(always)]
    #[must_use]
    pub fn agc_trim(&mut self) -> AGC_TRIM_W<29> {
        AGC_TRIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Flash option register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [optr](index.html) module
pub struct OPTR_SPEC;
impl crate::RegisterSpec for OPTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [optr::R](R) reader structure
impl crate::Readable for OPTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [optr::W](W) writer structure
impl crate::Writable for OPTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OPTR to value 0x1070_8000
impl crate::Resettable for OPTR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1070_8000;
}
