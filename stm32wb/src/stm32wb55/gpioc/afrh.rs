///Register `AFRH` reader
pub struct R(crate::R<AFRH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AFRH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AFRH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AFRH_SPEC>) -> Self {
        R(reader)
    }
}
///Register `AFRH` writer
pub struct W(crate::W<AFRH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AFRH_SPEC>;
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
impl From<crate::W<AFRH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AFRH_SPEC>) -> Self {
        W(writer)
    }
}
///Field `AFSEL8` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL8_R = crate::FieldReader<u8, u8>;
///Field `AFSEL8` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
///Field `AFSEL9` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL9_R = crate::FieldReader<u8, u8>;
///Field `AFSEL9` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
///Field `AFSEL10` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL10_R = crate::FieldReader<u8, u8>;
///Field `AFSEL10` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
///Field `AFSEL11` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL11_R = crate::FieldReader<u8, u8>;
///Field `AFSEL11` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
///Field `AFSEL12` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL12_R = crate::FieldReader<u8, u8>;
///Field `AFSEL12` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
///Field `AFSEL13` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL13_R = crate::FieldReader<u8, u8>;
///Field `AFSEL13` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
///Field `AFSEL14` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL14_R = crate::FieldReader<u8, u8>;
///Field `AFSEL14` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
///Field `AFSEL15` reader - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL15_R = crate::FieldReader<u8, u8>;
///Field `AFSEL15` writer - Alternate function selection for port x bit y (y = 8..15)
pub type AFSEL15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, AFRH_SPEC, u8, u8, 4, O>;
impl R {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel8(&self) -> AFSEL8_R {
        AFSEL8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel9(&self) -> AFSEL9_R {
        AFSEL9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel10(&self) -> AFSEL10_R {
        AFSEL10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel11(&self) -> AFSEL11_R {
        AFSEL11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel12(&self) -> AFSEL12_R {
        AFSEL12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel13(&self) -> AFSEL13_R {
        AFSEL13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel14(&self) -> AFSEL14_R {
        AFSEL14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    pub fn afsel15(&self) -> AFSEL15_R {
        AFSEL15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    ///Bits 0:3 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afsel8(&mut self) -> AFSEL8_W<0> {
        AFSEL8_W::new(self)
    }
    ///Bits 4:7 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afsel9(&mut self) -> AFSEL9_W<4> {
        AFSEL9_W::new(self)
    }
    ///Bits 8:11 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afsel10(&mut self) -> AFSEL10_W<8> {
        AFSEL10_W::new(self)
    }
    ///Bits 12:15 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afsel11(&mut self) -> AFSEL11_W<12> {
        AFSEL11_W::new(self)
    }
    ///Bits 16:19 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afsel12(&mut self) -> AFSEL12_W<16> {
        AFSEL12_W::new(self)
    }
    ///Bits 20:23 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afsel13(&mut self) -> AFSEL13_W<20> {
        AFSEL13_W::new(self)
    }
    ///Bits 24:27 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afsel14(&mut self) -> AFSEL14_W<24> {
        AFSEL14_W::new(self)
    }
    ///Bits 28:31 - Alternate function selection for port x bit y (y = 8..15)
    #[inline(always)]
    #[must_use]
    pub fn afsel15(&mut self) -> AFSEL15_W<28> {
        AFSEL15_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO alternate function high register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [afrh](index.html) module
pub struct AFRH_SPEC;
impl crate::RegisterSpec for AFRH_SPEC {
    type Ux = u32;
}
///`read()` method returns [afrh::R](R) reader structure
impl crate::Readable for AFRH_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [afrh::W](W) writer structure
impl crate::Writable for AFRH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets AFRH to value 0
impl crate::Resettable for AFRH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
