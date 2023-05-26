///Register `CRCPR` reader
pub struct R(crate::R<CRCPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRCPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRCPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRCPR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CRCPR` writer
pub struct W(crate::W<CRCPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRCPR_SPEC>;
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
impl From<crate::W<CRCPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRCPR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CRCPOLY` reader - CRC polynomial register
pub type CRCPOLY_R = crate::FieldReader<u16, u16>;
///Field `CRCPOLY` writer - CRC polynomial register
pub type CRCPOLY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CRCPR_SPEC, u16, u16, 16, O>;
impl R {
    ///Bits 0:15 - CRC polynomial register
    #[inline(always)]
    pub fn crcpoly(&self) -> CRCPOLY_R {
        CRCPOLY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    ///Bits 0:15 - CRC polynomial register
    #[inline(always)]
    #[must_use]
    pub fn crcpoly(&mut self) -> CRCPOLY_W<0> {
        CRCPOLY_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///CRC polynomial register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [crcpr](index.html) module
pub struct CRCPR_SPEC;
impl crate::RegisterSpec for CRCPR_SPEC {
    type Ux = u32;
}
///`read()` method returns [crcpr::R](R) reader structure
impl crate::Readable for CRCPR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [crcpr::W](W) writer structure
impl crate::Writable for CRCPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CRCPR to value 0x07
impl crate::Resettable for CRCPR_SPEC {
    const RESET_VALUE: Self::Ux = 0x07;
}
