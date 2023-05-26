///Register `CR4` reader
pub struct R(crate::R<CR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR4_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR4` writer
pub struct W(crate::W<CR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR4_SPEC>;
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
impl From<crate::W<CR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR4_SPEC>) -> Self {
        W(writer)
    }
}
///Field `WP1` reader - Wakeup pin WKUP1 polarity
pub type WP1_R = crate::BitReader<bool>;
///Field `WP1` writer - Wakeup pin WKUP1 polarity
pub type WP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
///Field `WP2` reader - Wakeup pin WKUP2 polarity
pub type WP2_R = crate::BitReader<bool>;
///Field `WP2` writer - Wakeup pin WKUP2 polarity
pub type WP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
///Field `WP3` reader - Wakeup pin WKUP3 polarity
pub type WP3_R = crate::BitReader<bool>;
///Field `WP3` writer - Wakeup pin WKUP3 polarity
pub type WP3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
///Field `WP4` reader - Wakeup pin WKUP4 polarity
pub type WP4_R = crate::BitReader<bool>;
///Field `WP4` writer - Wakeup pin WKUP4 polarity
pub type WP4_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
///Field `WP5` reader - Wakeup pin WKUP5 polarity
pub type WP5_R = crate::BitReader<bool>;
///Field `WP5` writer - Wakeup pin WKUP5 polarity
pub type WP5_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
///Field `VBE` reader - VBAT battery charging enable
pub type VBE_R = crate::BitReader<bool>;
///Field `VBE` writer - VBAT battery charging enable
pub type VBE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
///Field `VBRS` reader - VBAT battery charging resistor selection
pub type VBRS_R = crate::BitReader<bool>;
///Field `VBRS` writer - VBAT battery charging resistor selection
pub type VBRS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
///Field `C2BOOT` reader - BOOT CPU2 after reset or wakeup from Stop or Standby modes
pub type C2BOOT_R = crate::BitReader<bool>;
///Field `C2BOOT` writer - BOOT CPU2 after reset or wakeup from Stop or Standby modes
pub type C2BOOT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR4_SPEC, bool, O>;
impl R {
    ///Bit 0 - Wakeup pin WKUP1 polarity
    #[inline(always)]
    pub fn wp1(&self) -> WP1_R {
        WP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity
    #[inline(always)]
    pub fn wp2(&self) -> WP2_R {
        WP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity
    #[inline(always)]
    pub fn wp3(&self) -> WP3_R {
        WP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity
    #[inline(always)]
    pub fn wp4(&self) -> WP4_R {
        WP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity
    #[inline(always)]
    pub fn wp5(&self) -> WP5_R {
        WP5_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - VBAT battery charging enable
    #[inline(always)]
    pub fn vbe(&self) -> VBE_R {
        VBE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - VBAT battery charging resistor selection
    #[inline(always)]
    pub fn vbrs(&self) -> VBRS_R {
        VBRS_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 15 - BOOT CPU2 after reset or wakeup from Stop or Standby modes
    #[inline(always)]
    pub fn c2boot(&self) -> C2BOOT_R {
        C2BOOT_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Wakeup pin WKUP1 polarity
    #[inline(always)]
    #[must_use]
    pub fn wp1(&mut self) -> WP1_W<0> {
        WP1_W::new(self)
    }
    ///Bit 1 - Wakeup pin WKUP2 polarity
    #[inline(always)]
    #[must_use]
    pub fn wp2(&mut self) -> WP2_W<1> {
        WP2_W::new(self)
    }
    ///Bit 2 - Wakeup pin WKUP3 polarity
    #[inline(always)]
    #[must_use]
    pub fn wp3(&mut self) -> WP3_W<2> {
        WP3_W::new(self)
    }
    ///Bit 3 - Wakeup pin WKUP4 polarity
    #[inline(always)]
    #[must_use]
    pub fn wp4(&mut self) -> WP4_W<3> {
        WP4_W::new(self)
    }
    ///Bit 4 - Wakeup pin WKUP5 polarity
    #[inline(always)]
    #[must_use]
    pub fn wp5(&mut self) -> WP5_W<4> {
        WP5_W::new(self)
    }
    ///Bit 8 - VBAT battery charging enable
    #[inline(always)]
    #[must_use]
    pub fn vbe(&mut self) -> VBE_W<8> {
        VBE_W::new(self)
    }
    ///Bit 9 - VBAT battery charging resistor selection
    #[inline(always)]
    #[must_use]
    pub fn vbrs(&mut self) -> VBRS_W<9> {
        VBRS_W::new(self)
    }
    ///Bit 15 - BOOT CPU2 after reset or wakeup from Stop or Standby modes
    #[inline(always)]
    #[must_use]
    pub fn c2boot(&mut self) -> C2BOOT_W<15> {
        C2BOOT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Power control register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr4](index.html) module
pub struct CR4_SPEC;
impl crate::RegisterSpec for CR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr4::R](R) reader structure
impl crate::Readable for CR4_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr4::W](W) writer structure
impl crate::Writable for CR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR4 to value 0
impl crate::Resettable for CR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
