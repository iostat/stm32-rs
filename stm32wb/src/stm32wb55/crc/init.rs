///Register `INIT` reader
pub struct R(crate::R<INIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INIT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `INIT` writer
pub struct W(crate::W<INIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INIT_SPEC>;
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
impl From<crate::W<INIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INIT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CRC_INIT` reader - Programmable initial CRC value
pub type CRC_INIT_R = crate::FieldReader<u32, u32>;
///Field `CRC_INIT` writer - Programmable initial CRC value
pub type CRC_INIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, INIT_SPEC, u32, u32, 32, O>;
impl R {
    ///Bits 0:31 - Programmable initial CRC value
    #[inline(always)]
    pub fn crc_init(&self) -> CRC_INIT_R {
        CRC_INIT_R::new(self.bits)
    }
}
impl W {
    ///Bits 0:31 - Programmable initial CRC value
    #[inline(always)]
    #[must_use]
    pub fn crc_init(&mut self) -> CRC_INIT_W<0> {
        CRC_INIT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Initial CRC value
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [init](index.html) module
pub struct INIT_SPEC;
impl crate::RegisterSpec for INIT_SPEC {
    type Ux = u32;
}
///`read()` method returns [init::R](R) reader structure
impl crate::Readable for INIT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [init::W](W) writer structure
impl crate::Writable for INIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets INIT to value 0xffff_ffff
impl crate::Resettable for INIT_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
