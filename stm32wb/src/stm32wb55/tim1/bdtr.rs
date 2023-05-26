///Register `BDTR` reader
pub struct R(crate::R<BDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BDTR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BDTR` writer
pub struct W(crate::W<BDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BDTR_SPEC>;
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
impl From<crate::W<BDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BDTR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DTG` reader - Dead-time generator setup
pub type DTG_R = crate::FieldReader<u8, u8>;
///Field `DTG` writer - Dead-time generator setup
pub type DTG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDTR_SPEC, u8, u8, 8, O>;
///Field `LOCK` reader - Lock configuration
pub type LOCK_R = crate::FieldReader<u8, u8>;
///Field `LOCK` writer - Lock configuration
pub type LOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDTR_SPEC, u8, u8, 2, O>;
///Field `OSSI` reader - Off-state selection for Idle mode
pub type OSSI_R = crate::BitReader<bool>;
///Field `OSSI` writer - Off-state selection for Idle mode
pub type OSSI_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
///Field `OSSR` reader - Off-state selection for Run mode
pub type OSSR_R = crate::BitReader<bool>;
///Field `OSSR` writer - Off-state selection for Run mode
pub type OSSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
///Field `BKE` reader - Break enable
pub type BKE_R = crate::BitReader<bool>;
///Field `BKE` writer - Break enable
pub type BKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
///Field `BKP` reader - Break polarity
pub type BKP_R = crate::BitReader<bool>;
///Field `BKP` writer - Break polarity
pub type BKP_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
///Field `AOE` reader - Automatic output enable
pub type AOE_R = crate::BitReader<bool>;
///Field `AOE` writer - Automatic output enable
pub type AOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
///Field `MOE` reader - Main output enable
pub type MOE_R = crate::BitReader<bool>;
///Field `MOE` writer - Main output enable
pub type MOE_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
///Field `BKF` reader - Break filter
pub type BKF_R = crate::FieldReader<u8, u8>;
///Field `BKF` writer - Break filter
pub type BKF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDTR_SPEC, u8, u8, 4, O>;
///Field `BK2F` reader - Break 2 filter
pub type BK2F_R = crate::FieldReader<u8, u8>;
///Field `BK2F` writer - Break 2 filter
pub type BK2F_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BDTR_SPEC, u8, u8, 4, O>;
///Field `BK2E` reader - Break 2 enable
pub type BK2E_R = crate::BitReader<bool>;
///Field `BK2E` writer - Break 2 enable
pub type BK2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
///Field `BK2P` reader - Break 2 polarity
pub type BK2P_R = crate::BitReader<bool>;
///Field `BK2P` writer - Break 2 polarity
pub type BK2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, BDTR_SPEC, bool, O>;
impl R {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    pub fn dtg(&self) -> DTG_R {
        DTG_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    pub fn ossi(&self) -> OSSI_R {
        OSSI_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    pub fn ossr(&self) -> OSSR_R {
        OSSR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    pub fn bke(&self) -> BKE_R {
        BKE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    pub fn aoe(&self) -> AOE_R {
        AOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    pub fn moe(&self) -> MOE_R {
        MOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    pub fn bkf(&self) -> BKF_R {
        BKF_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Break 2 filter
    #[inline(always)]
    pub fn bk2f(&self) -> BK2F_R {
        BK2F_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bit 24 - Break 2 enable
    #[inline(always)]
    pub fn bk2e(&self) -> BK2E_R {
        BK2E_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Break 2 polarity
    #[inline(always)]
    pub fn bk2p(&self) -> BK2P_R {
        BK2P_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    ///Bits 0:7 - Dead-time generator setup
    #[inline(always)]
    #[must_use]
    pub fn dtg(&mut self) -> DTG_W<0> {
        DTG_W::new(self)
    }
    ///Bits 8:9 - Lock configuration
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<8> {
        LOCK_W::new(self)
    }
    ///Bit 10 - Off-state selection for Idle mode
    #[inline(always)]
    #[must_use]
    pub fn ossi(&mut self) -> OSSI_W<10> {
        OSSI_W::new(self)
    }
    ///Bit 11 - Off-state selection for Run mode
    #[inline(always)]
    #[must_use]
    pub fn ossr(&mut self) -> OSSR_W<11> {
        OSSR_W::new(self)
    }
    ///Bit 12 - Break enable
    #[inline(always)]
    #[must_use]
    pub fn bke(&mut self) -> BKE_W<12> {
        BKE_W::new(self)
    }
    ///Bit 13 - Break polarity
    #[inline(always)]
    #[must_use]
    pub fn bkp(&mut self) -> BKP_W<13> {
        BKP_W::new(self)
    }
    ///Bit 14 - Automatic output enable
    #[inline(always)]
    #[must_use]
    pub fn aoe(&mut self) -> AOE_W<14> {
        AOE_W::new(self)
    }
    ///Bit 15 - Main output enable
    #[inline(always)]
    #[must_use]
    pub fn moe(&mut self) -> MOE_W<15> {
        MOE_W::new(self)
    }
    ///Bits 16:19 - Break filter
    #[inline(always)]
    #[must_use]
    pub fn bkf(&mut self) -> BKF_W<16> {
        BKF_W::new(self)
    }
    ///Bits 20:23 - Break 2 filter
    #[inline(always)]
    #[must_use]
    pub fn bk2f(&mut self) -> BK2F_W<20> {
        BK2F_W::new(self)
    }
    ///Bit 24 - Break 2 enable
    #[inline(always)]
    #[must_use]
    pub fn bk2e(&mut self) -> BK2E_W<24> {
        BK2E_W::new(self)
    }
    ///Bit 25 - Break 2 polarity
    #[inline(always)]
    #[must_use]
    pub fn bk2p(&mut self) -> BK2P_W<25> {
        BK2P_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///break and dead-time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [bdtr](index.html) module
pub struct BDTR_SPEC;
impl crate::RegisterSpec for BDTR_SPEC {
    type Ux = u32;
}
///`read()` method returns [bdtr::R](R) reader structure
impl crate::Readable for BDTR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [bdtr::W](W) writer structure
impl crate::Writable for BDTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BDTR to value 0
impl crate::Resettable for BDTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
