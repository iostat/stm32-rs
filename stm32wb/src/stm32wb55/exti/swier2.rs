///Register `SWIER2` reader
pub struct R(crate::R<SWIER2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWIER2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWIER2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWIER2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SWIER2` writer
pub struct W(crate::W<SWIER2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWIER2_SPEC>;
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
impl From<crate::W<SWIER2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWIER2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SWI33` reader - Software interrupt on event
pub type SWI33_R = crate::BitReader<bool>;
///Field `SWI33` writer - Software interrupt on event
pub type SWI33_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWIER2_SPEC, bool, O>;
///Field `SWI40_41` reader - Software interrupt on event
pub type SWI40_41_R = crate::FieldReader<u8, u8>;
///Field `SWI40_41` writer - Software interrupt on event
pub type SWI40_41_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SWIER2_SPEC, u8, u8, 2, O>;
impl R {
    ///Bit 1 - Software interrupt on event
    #[inline(always)]
    pub fn swi33(&self) -> SWI33_R {
        SWI33_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 8:9 - Software interrupt on event
    #[inline(always)]
    pub fn swi40_41(&self) -> SWI40_41_R {
        SWI40_41_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    ///Bit 1 - Software interrupt on event
    #[inline(always)]
    #[must_use]
    pub fn swi33(&mut self) -> SWI33_W<1> {
        SWI33_W::new(self)
    }
    ///Bits 8:9 - Software interrupt on event
    #[inline(always)]
    #[must_use]
    pub fn swi40_41(&mut self) -> SWI40_41_W<8> {
        SWI40_41_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///software interrupt event register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [swier2](index.html) module
pub struct SWIER2_SPEC;
impl crate::RegisterSpec for SWIER2_SPEC {
    type Ux = u32;
}
///`read()` method returns [swier2::R](R) reader structure
impl crate::Readable for SWIER2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [swier2::W](W) writer structure
impl crate::Writable for SWIER2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SWIER2 to value 0
impl crate::Resettable for SWIER2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
