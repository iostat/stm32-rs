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
///Field `SYNCOKIE` reader - SYNC event OK interrupt enable
pub type SYNCOKIE_R = crate::BitReader<bool>;
///Field `SYNCOKIE` writer - SYNC event OK interrupt enable
pub type SYNCOKIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `SYNCWARNIE` reader - SYNC warning interrupt enable
pub type SYNCWARNIE_R = crate::BitReader<bool>;
///Field `SYNCWARNIE` writer - SYNC warning interrupt enable
pub type SYNCWARNIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ERRIE` reader - Synchronization or trimming error interrupt enable
pub type ERRIE_R = crate::BitReader<bool>;
///Field `ERRIE` writer - Synchronization or trimming error interrupt enable
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `ESYNCIE` reader - Expected SYNC interrupt enable
pub type ESYNCIE_R = crate::BitReader<bool>;
///Field `ESYNCIE` writer - Expected SYNC interrupt enable
pub type ESYNCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `CEN` reader - Frequency error counter enable
pub type CEN_R = crate::BitReader<bool>;
///Field `CEN` writer - Frequency error counter enable
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `AUTOTRIMEN` reader - Automatic trimming enable
pub type AUTOTRIMEN_R = crate::BitReader<bool>;
///Field `AUTOTRIMEN` writer - Automatic trimming enable
pub type AUTOTRIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `SWSYNC` reader - Automatic trimming enable
pub type SWSYNC_R = crate::BitReader<bool>;
///Field `SWSYNC` writer - Automatic trimming enable
pub type SWSYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `TRIM` reader - HSI48 oscillator smooth trimming
pub type TRIM_R = crate::FieldReader<u8, u8>;
///Field `TRIM` writer - HSI48 oscillator smooth trimming
pub type TRIM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 6, O>;
impl R {
    ///Bit 0 - SYNC event OK interrupt enable
    #[inline(always)]
    pub fn syncokie(&self) -> SYNCOKIE_R {
        SYNCOKIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SYNC warning interrupt enable
    #[inline(always)]
    pub fn syncwarnie(&self) -> SYNCWARNIE_R {
        SYNCWARNIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization or trimming error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Expected SYNC interrupt enable
    #[inline(always)]
    pub fn esyncie(&self) -> ESYNCIE_R {
        ESYNCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - Frequency error counter enable
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Automatic trimming enable
    #[inline(always)]
    pub fn autotrimen(&self) -> AUTOTRIMEN_R {
        AUTOTRIMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Automatic trimming enable
    #[inline(always)]
    pub fn swsync(&self) -> SWSYNC_R {
        SWSYNC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:13 - HSI48 oscillator smooth trimming
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    ///Bit 0 - SYNC event OK interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn syncokie(&mut self) -> SYNCOKIE_W<0> {
        SYNCOKIE_W::new(self)
    }
    ///Bit 1 - SYNC warning interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn syncwarnie(&mut self) -> SYNCWARNIE_W<1> {
        SYNCWARNIE_W::new(self)
    }
    ///Bit 2 - Synchronization or trimming error interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<2> {
        ERRIE_W::new(self)
    }
    ///Bit 3 - Expected SYNC interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn esyncie(&mut self) -> ESYNCIE_W<3> {
        ESYNCIE_W::new(self)
    }
    ///Bit 5 - Frequency error counter enable
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<5> {
        CEN_W::new(self)
    }
    ///Bit 6 - Automatic trimming enable
    #[inline(always)]
    #[must_use]
    pub fn autotrimen(&mut self) -> AUTOTRIMEN_W<6> {
        AUTOTRIMEN_W::new(self)
    }
    ///Bit 7 - Automatic trimming enable
    #[inline(always)]
    #[must_use]
    pub fn swsync(&mut self) -> SWSYNC_W<7> {
        SWSYNC_W::new(self)
    }
    ///Bits 8:13 - HSI48 oscillator smooth trimming
    #[inline(always)]
    #[must_use]
    pub fn trim(&mut self) -> TRIM_W<8> {
        TRIM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CRS control register
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
///`reset()` method sets CR to value 0x2000
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000;
}
