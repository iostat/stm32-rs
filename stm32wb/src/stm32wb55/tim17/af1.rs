///Register `AF1` reader
pub struct R(crate::R<AF1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AF1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AF1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AF1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AF1` writer
pub struct W(crate::W<AF1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AF1_SPEC>;
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
impl From<crate::W<AF1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AF1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BKINE` reader - BRK BKIN input enable
pub type BKINE_R = crate::BitReader<bool>;
///Field `BKINE` writer - BRK BKIN input enable
pub type BKINE_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
///Field `BKCMP1E` reader - BRK COMP1 enable
pub type BKCMP1E_R = crate::BitReader<bool>;
///Field `BKCMP1E` writer - BRK COMP1 enable
pub type BKCMP1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
///Field `BKCMP2E` reader - BRK COMP2 enable
pub type BKCMP2E_R = crate::BitReader<bool>;
///Field `BKCMP2E` writer - BRK COMP2 enable
pub type BKCMP2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
///Field `BKINP` reader - BRK BKIN input polarity
pub type BKINP_R = crate::BitReader<bool>;
///Field `BKINP` writer - BRK BKIN input polarity
pub type BKINP_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
///Field `BKCMP1P` reader - BRK COMP1 input polarity
pub type BKCMP1P_R = crate::BitReader<bool>;
///Field `BKCMP1P` writer - BRK COMP1 input polarity
pub type BKCMP1P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
///Field `BKCMP2P` reader - BRK COMP2 input polarit
pub type BKCMP2P_R = crate::BitReader<bool>;
///Field `BKCMP2P` writer - BRK COMP2 input polarit
pub type BKCMP2P_W<'a, const O: u8> = crate::BitWriter<'a, u32, AF1_SPEC, bool, O>;
impl R {
    ///Bit 0 - BRK BKIN input enable
    #[inline(always)]
    pub fn bkine(&self) -> BKINE_R {
        BKINE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BRK COMP1 enable
    #[inline(always)]
    pub fn bkcmp1e(&self) -> BKCMP1E_R {
        BKCMP1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - BRK COMP2 enable
    #[inline(always)]
    pub fn bkcmp2e(&self) -> BKCMP2E_R {
        BKCMP2E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 9 - BRK BKIN input polarity
    #[inline(always)]
    pub fn bkinp(&self) -> BKINP_R {
        BKINP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BRK COMP1 input polarity
    #[inline(always)]
    pub fn bkcmp1p(&self) -> BKCMP1P_R {
        BKCMP1P_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - BRK COMP2 input polarit
    #[inline(always)]
    pub fn bkcmp2p(&self) -> BKCMP2P_R {
        BKCMP2P_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - BRK BKIN input enable
    #[inline(always)]
    #[must_use]
    pub fn bkine(&mut self) -> BKINE_W<0> {
        BKINE_W::new(self)
    }
    ///Bit 1 - BRK COMP1 enable
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1e(&mut self) -> BKCMP1E_W<1> {
        BKCMP1E_W::new(self)
    }
    ///Bit 2 - BRK COMP2 enable
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2e(&mut self) -> BKCMP2E_W<2> {
        BKCMP2E_W::new(self)
    }
    ///Bit 9 - BRK BKIN input polarity
    #[inline(always)]
    #[must_use]
    pub fn bkinp(&mut self) -> BKINP_W<9> {
        BKINP_W::new(self)
    }
    ///Bit 10 - BRK COMP1 input polarity
    #[inline(always)]
    #[must_use]
    pub fn bkcmp1p(&mut self) -> BKCMP1P_W<10> {
        BKCMP1P_W::new(self)
    }
    ///Bit 11 - BRK COMP2 input polarit
    #[inline(always)]
    #[must_use]
    pub fn bkcmp2p(&mut self) -> BKCMP2P_W<11> {
        BKCMP2P_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///TIM17 Alternate function
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [af1](index.html) module
pub struct AF1_SPEC;
impl crate::RegisterSpec for AF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [af1::R](R) reader structure
impl crate::Readable for AF1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [af1::W](W) writer structure
impl crate::Writable for AF1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AF1 to value 0
impl crate::Resettable for AF1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
