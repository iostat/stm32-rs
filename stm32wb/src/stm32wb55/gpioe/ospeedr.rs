///Register `OSPEEDR` reader
pub struct R(crate::R<OSPEEDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSPEEDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSPEEDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSPEEDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `OSPEEDR` writer
pub struct W(crate::W<OSPEEDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSPEEDR_SPEC>;
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
impl From<crate::W<OSPEEDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSPEEDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OSPEEDR0` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR0_R = crate::FieldReader<u8, u8>;
///Field `OSPEEDR0` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
///Field `OSPEEDR1` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR1_R = crate::FieldReader<u8, u8>;
///Field `OSPEEDR1` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
///Field `OSPEEDR2` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR2_R = crate::FieldReader<u8, u8>;
///Field `OSPEEDR2` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
///Field `OSPEEDR3` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR3_R = crate::FieldReader<u8, u8>;
///Field `OSPEEDR3` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
///Field `OSPEEDR4` reader - Port x configuration bits (y = 0..15)
pub type OSPEEDR4_R = crate::FieldReader<u8, u8>;
///Field `OSPEEDR4` writer - Port x configuration bits (y = 0..15)
pub type OSPEEDR4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OSPEEDR_SPEC, u8, u8, 2, O>;
impl R {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr0(&self) -> OSPEEDR0_R {
        OSPEEDR0_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr1(&self) -> OSPEEDR1_R {
        OSPEEDR1_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr2(&self) -> OSPEEDR2_R {
        OSPEEDR2_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr3(&self) -> OSPEEDR3_R {
        OSPEEDR3_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ospeedr4(&self) -> OSPEEDR4_R {
        OSPEEDR4_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    ///Bits 0:1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ospeedr0(&mut self) -> OSPEEDR0_W<0> {
        OSPEEDR0_W::new(self)
    }
    ///Bits 2:3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ospeedr1(&mut self) -> OSPEEDR1_W<2> {
        OSPEEDR1_W::new(self)
    }
    ///Bits 4:5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ospeedr2(&mut self) -> OSPEEDR2_W<4> {
        OSPEEDR2_W::new(self)
    }
    ///Bits 6:7 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ospeedr3(&mut self) -> OSPEEDR3_W<6> {
        OSPEEDR3_W::new(self)
    }
    ///Bits 8:9 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    #[must_use]
    pub fn ospeedr4(&mut self) -> OSPEEDR4_W<8> {
        OSPEEDR4_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///GPIO port output speed register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ospeedr](index.html) module
pub struct OSPEEDR_SPEC;
impl crate::RegisterSpec for OSPEEDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [ospeedr::R](R) reader structure
impl crate::Readable for OSPEEDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ospeedr::W](W) writer structure
impl crate::Writable for OSPEEDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets OSPEEDR to value 0xc0
impl crate::Resettable for OSPEEDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
