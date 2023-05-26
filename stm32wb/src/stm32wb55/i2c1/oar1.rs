///Register `OAR1` reader
pub struct R(crate::R<OAR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OAR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OAR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OAR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OAR1` writer
pub struct W(crate::W<OAR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OAR1_SPEC>;
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
impl From<crate::W<OAR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OAR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OA1` reader - Interface address
pub type OA1_R = crate::FieldReader<u16, u16>;
///Field `OA1` writer - Interface address
pub type OA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OAR1_SPEC, u16, u16, 10, O>;
///Field `OA1MODE` reader - Own Address 1 10-bit mode
pub type OA1MODE_R = crate::BitReader<bool>;
///Field `OA1MODE` writer - Own Address 1 10-bit mode
pub type OA1MODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, bool, O>;
///Field `OA1EN` reader - Own Address 1 enable
pub type OA1EN_R = crate::BitReader<bool>;
///Field `OA1EN` writer - Own Address 1 enable
pub type OA1EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OAR1_SPEC, bool, O>;
impl R {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    pub fn oa1(&self) -> OA1_R {
        OA1_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Own Address 1 10-bit mode
    #[inline(always)]
    pub fn oa1mode(&self) -> OA1MODE_R {
        OA1MODE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 15 - Own Address 1 enable
    #[inline(always)]
    pub fn oa1en(&self) -> OA1EN_R {
        OA1EN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 0:9 - Interface address
    #[inline(always)]
    #[must_use]
    pub fn oa1(&mut self) -> OA1_W<0> {
        OA1_W::new(self)
    }
    ///Bit 10 - Own Address 1 10-bit mode
    #[inline(always)]
    #[must_use]
    pub fn oa1mode(&mut self) -> OA1MODE_W<10> {
        OA1MODE_W::new(self)
    }
    ///Bit 15 - Own Address 1 enable
    #[inline(always)]
    #[must_use]
    pub fn oa1en(&mut self) -> OA1EN_W<15> {
        OA1EN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Own address register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [oar1](index.html) module
pub struct OAR1_SPEC;
impl crate::RegisterSpec for OAR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [oar1::R](R) reader structure
impl crate::Readable for OAR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [oar1::W](W) writer structure
impl crate::Writable for OAR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OAR1 to value 0
impl crate::Resettable for OAR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
