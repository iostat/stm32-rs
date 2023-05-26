///Register `DR` reader
pub struct R(crate::R<DR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DR` writer
pub struct W(crate::W<DR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_SPEC>;
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
impl From<crate::W<DR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `DU` reader - Date units in BCD format
pub type DU_R = crate::FieldReader<u8, u8>;
///Field `DU` writer - Date units in BCD format
pub type DU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
///Field `DT` reader - Date tens in BCD format
pub type DT_R = crate::FieldReader<u8, u8>;
///Field `DT` writer - Date tens in BCD format
pub type DT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 2, O>;
///Field `MU` reader - Month units in BCD format
pub type MU_R = crate::FieldReader<u8, u8>;
///Field `MU` writer - Month units in BCD format
pub type MU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
///Field `MT` reader - Month tens in BCD format
pub type MT_R = crate::BitReader<bool>;
///Field `MT` writer - Month tens in BCD format
pub type MT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DR_SPEC, bool, O>;
///Field `WDU` reader - Week day units
pub type WDU_R = crate::FieldReader<u8, u8>;
///Field `WDU` writer - Week day units
pub type WDU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 3, O>;
///Field `YU` reader - Year units in BCD format
pub type YU_R = crate::FieldReader<u8, u8>;
///Field `YU` writer - Year units in BCD format
pub type YU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
///Field `YT` reader - Year tens in BCD format
pub type YT_R = crate::FieldReader<u8, u8>;
///Field `YT` writer - Year tens in BCD format
pub type YT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - Date units in BCD format
    #[inline(always)]
    pub fn du(&self) -> DU_R {
        DU_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:5 - Date tens in BCD format
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 8:11 - Month units in BCD format
    #[inline(always)]
    pub fn mu(&self) -> MU_R {
        MU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Month tens in BCD format
    #[inline(always)]
    pub fn mt(&self) -> MT_R {
        MT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:15 - Week day units
    #[inline(always)]
    pub fn wdu(&self) -> WDU_R {
        WDU_R::new(((self.bits >> 13) & 7) as u8)
    }
    ///Bits 16:19 - Year units in BCD format
    #[inline(always)]
    pub fn yu(&self) -> YU_R {
        YU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Year tens in BCD format
    #[inline(always)]
    pub fn yt(&self) -> YT_R {
        YT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Date units in BCD format
    #[inline(always)]
    #[must_use]
    pub fn du(&mut self) -> DU_W<0> {
        DU_W::new(self)
    }
    ///Bits 4:5 - Date tens in BCD format
    #[inline(always)]
    #[must_use]
    pub fn dt(&mut self) -> DT_W<4> {
        DT_W::new(self)
    }
    ///Bits 8:11 - Month units in BCD format
    #[inline(always)]
    #[must_use]
    pub fn mu(&mut self) -> MU_W<8> {
        MU_W::new(self)
    }
    ///Bit 12 - Month tens in BCD format
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MT_W<12> {
        MT_W::new(self)
    }
    ///Bits 13:15 - Week day units
    #[inline(always)]
    #[must_use]
    pub fn wdu(&mut self) -> WDU_W<13> {
        WDU_W::new(self)
    }
    ///Bits 16:19 - Year units in BCD format
    #[inline(always)]
    #[must_use]
    pub fn yu(&mut self) -> YU_W<16> {
        YU_W::new(self)
    }
    ///Bits 20:23 - Year tens in BCD format
    #[inline(always)]
    #[must_use]
    pub fn yt(&mut self) -> YT_W<20> {
        YT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///date register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [dr](index.html) module
pub struct DR_SPEC;
impl crate::RegisterSpec for DR_SPEC {
    type Ux = u32;
}
///`read()` method returns [dr::R](R) reader structure
impl crate::Readable for DR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [dr::W](W) writer structure
impl crate::Writable for DR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DR to value 0x2101
impl crate::Resettable for DR_SPEC {
    const RESET_VALUE: Self::Ux = 0x2101;
}
