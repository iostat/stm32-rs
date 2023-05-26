///Register `IOGCSR` reader
pub struct R(crate::R<IOGCSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IOGCSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IOGCSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IOGCSR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IOGCSR` writer
pub struct W(crate::W<IOGCSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IOGCSR_SPEC>;
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
impl From<crate::W<IOGCSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IOGCSR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `G1E` reader - Analog I/O group x enable
pub type G1E_R = crate::BitReader<bool>;
///Field `G1E` writer - Analog I/O group x enable
pub type G1E_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOGCSR_SPEC, bool, O>;
///Field `G2E` reader - Analog I/O group x enable
pub type G2E_R = crate::BitReader<bool>;
///Field `G2E` writer - Analog I/O group x enable
pub type G2E_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOGCSR_SPEC, bool, O>;
///Field `G3E` reader - Analog I/O group x enable
pub type G3E_R = crate::BitReader<bool>;
///Field `G3E` writer - Analog I/O group x enable
pub type G3E_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOGCSR_SPEC, bool, O>;
///Field `G4E` reader - Analog I/O group x enable
pub type G4E_R = crate::BitReader<bool>;
///Field `G4E` writer - Analog I/O group x enable
pub type G4E_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOGCSR_SPEC, bool, O>;
///Field `G5E` reader - Analog I/O group x enable
pub type G5E_R = crate::BitReader<bool>;
///Field `G5E` writer - Analog I/O group x enable
pub type G5E_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOGCSR_SPEC, bool, O>;
///Field `G6E` reader - Analog I/O group x enable
pub type G6E_R = crate::BitReader<bool>;
///Field `G6E` writer - Analog I/O group x enable
pub type G6E_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOGCSR_SPEC, bool, O>;
///Field `G7E` reader - Analog I/O group x enable
pub type G7E_R = crate::BitReader<bool>;
///Field `G7E` writer - Analog I/O group x enable
pub type G7E_W<'a, const O: u8> = crate::BitWriter<'a, u32, IOGCSR_SPEC, bool, O>;
///Field `G1S` reader - Analog I/O group x status
pub type G1S_R = crate::BitReader<bool>;
///Field `G2S` reader - Analog I/O group x status
pub type G2S_R = crate::BitReader<bool>;
///Field `G3S` reader - Analog I/O group x status
pub type G3S_R = crate::BitReader<bool>;
///Field `G4S` reader - Analog I/O group x status
pub type G4S_R = crate::BitReader<bool>;
///Field `G5S` reader - Analog I/O group x status
pub type G5S_R = crate::BitReader<bool>;
///Field `G6S` reader - Analog I/O group x status
pub type G6S_R = crate::BitReader<bool>;
///Field `G7S` reader - Analog I/O group x status
pub type G7S_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - Analog I/O group x enable
    #[inline(always)]
    pub fn g1e(&self) -> G1E_R {
        G1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Analog I/O group x enable
    #[inline(always)]
    pub fn g2e(&self) -> G2E_R {
        G2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Analog I/O group x enable
    #[inline(always)]
    pub fn g3e(&self) -> G3E_R {
        G3E_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Analog I/O group x enable
    #[inline(always)]
    pub fn g4e(&self) -> G4E_R {
        G4E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Analog I/O group x enable
    #[inline(always)]
    pub fn g5e(&self) -> G5E_R {
        G5E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Analog I/O group x enable
    #[inline(always)]
    pub fn g6e(&self) -> G6E_R {
        G6E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Analog I/O group x enable
    #[inline(always)]
    pub fn g7e(&self) -> G7E_R {
        G7E_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 16 - Analog I/O group x status
    #[inline(always)]
    pub fn g1s(&self) -> G1S_R {
        G1S_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Analog I/O group x status
    #[inline(always)]
    pub fn g2s(&self) -> G2S_R {
        G2S_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Analog I/O group x status
    #[inline(always)]
    pub fn g3s(&self) -> G3S_R {
        G3S_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Analog I/O group x status
    #[inline(always)]
    pub fn g4s(&self) -> G4S_R {
        G4S_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Analog I/O group x status
    #[inline(always)]
    pub fn g5s(&self) -> G5S_R {
        G5S_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Analog I/O group x status
    #[inline(always)]
    pub fn g6s(&self) -> G6S_R {
        G6S_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Analog I/O group x status
    #[inline(always)]
    pub fn g7s(&self) -> G7S_R {
        G7S_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Analog I/O group x enable
    #[inline(always)]
    #[must_use]
    pub fn g1e(&mut self) -> G1E_W<0> {
        G1E_W::new(self)
    }
    ///Bit 1 - Analog I/O group x enable
    #[inline(always)]
    #[must_use]
    pub fn g2e(&mut self) -> G2E_W<1> {
        G2E_W::new(self)
    }
    ///Bit 2 - Analog I/O group x enable
    #[inline(always)]
    #[must_use]
    pub fn g3e(&mut self) -> G3E_W<2> {
        G3E_W::new(self)
    }
    ///Bit 3 - Analog I/O group x enable
    #[inline(always)]
    #[must_use]
    pub fn g4e(&mut self) -> G4E_W<3> {
        G4E_W::new(self)
    }
    ///Bit 4 - Analog I/O group x enable
    #[inline(always)]
    #[must_use]
    pub fn g5e(&mut self) -> G5E_W<4> {
        G5E_W::new(self)
    }
    ///Bit 5 - Analog I/O group x enable
    #[inline(always)]
    #[must_use]
    pub fn g6e(&mut self) -> G6E_W<5> {
        G6E_W::new(self)
    }
    ///Bit 6 - Analog I/O group x enable
    #[inline(always)]
    #[must_use]
    pub fn g7e(&mut self) -> G7E_W<6> {
        G7E_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///I/O group control status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [iogcsr](index.html) module
pub struct IOGCSR_SPEC;
impl crate::RegisterSpec for IOGCSR_SPEC {
    type Ux = u32;
}
///`read()` method returns [iogcsr::R](R) reader structure
impl crate::Readable for IOGCSR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [iogcsr::W](W) writer structure
impl crate::Writable for IOGCSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IOGCSR to value 0
impl crate::Resettable for IOGCSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
