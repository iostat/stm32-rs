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
///Field `ADRDY_MST` reader - master ADC ready
pub type ADRDY_MST_R = crate::BitReader<bool>;
///Field `EOSMP_MST` reader - End of Sampling phase flag of the master ADC
pub type EOSMP_MST_R = crate::BitReader<bool>;
///Field `EOC_MST` reader - End of regular conversion flag of the master ADC
pub type EOC_MST_R = crate::BitReader<bool>;
///Field `EOS_MST` reader - End of regular sequence flag of the master ADC
pub type EOS_MST_R = crate::BitReader<bool>;
///Field `OVR_MST` reader - Overrun flag of the master ADC
pub type OVR_MST_R = crate::BitReader<bool>;
///Field `JEOC_MST` reader - End of injected conversion flag of the master ADC
pub type JEOC_MST_R = crate::BitReader<bool>;
///Field `JEOS_MST` reader - End of injected sequence flag of the master ADC
pub type JEOS_MST_R = crate::BitReader<bool>;
///Field `AWD1_MST` reader - Analog watchdog 1 flag of the master ADC
pub type AWD1_MST_R = crate::BitReader<bool>;
///Field `AWD2_MST` reader - Analog watchdog 2 flag of the master ADC
pub type AWD2_MST_R = crate::BitReader<bool>;
///Field `AWD3_MST` reader - Analog watchdog 3 flag of the master ADC
pub type AWD3_MST_R = crate::BitReader<bool>;
///Field `JQOVF_MST` reader - Injected Context Queue Overflow flag of the master ADC
pub type JQOVF_MST_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - master ADC ready
    #[inline(always)]
    pub fn adrdy_mst(&self) -> ADRDY_MST_R {
        ADRDY_MST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of Sampling phase flag of the master ADC
    #[inline(always)]
    pub fn eosmp_mst(&self) -> EOSMP_MST_R {
        EOSMP_MST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of regular conversion flag of the master ADC
    #[inline(always)]
    pub fn eoc_mst(&self) -> EOC_MST_R {
        EOC_MST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of regular sequence flag of the master ADC
    #[inline(always)]
    pub fn eos_mst(&self) -> EOS_MST_R {
        EOS_MST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Overrun flag of the master ADC
    #[inline(always)]
    pub fn ovr_mst(&self) -> OVR_MST_R {
        OVR_MST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - End of injected conversion flag of the master ADC
    #[inline(always)]
    pub fn jeoc_mst(&self) -> JEOC_MST_R {
        JEOC_MST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - End of injected sequence flag of the master ADC
    #[inline(always)]
    pub fn jeos_mst(&self) -> JEOS_MST_R {
        JEOS_MST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog 1 flag of the master ADC
    #[inline(always)]
    pub fn awd1_mst(&self) -> AWD1_MST_R {
        AWD1_MST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag of the master ADC
    #[inline(always)]
    pub fn awd2_mst(&self) -> AWD2_MST_R {
        AWD2_MST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag of the master ADC
    #[inline(always)]
    pub fn awd3_mst(&self) -> AWD3_MST_R {
        AWD3_MST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Injected Context Queue Overflow flag of the master ADC
    #[inline(always)]
    pub fn jqovf_mst(&self) -> JQOVF_MST_R {
        JQOVF_MST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
///ADC common status register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
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
///`reset()` method sets CSR to value 0
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
