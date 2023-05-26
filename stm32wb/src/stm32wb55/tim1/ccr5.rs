///Register `CCR5` reader
pub struct R(crate::R<CCR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR5_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CCR5` writer
pub struct W(crate::W<CCR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR5_SPEC>;
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
impl From<crate::W<CCR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR5_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CCR5` reader - Capture/Compare value
pub type CCR5_R = crate::FieldReader<u16, u16>;
///Field `CCR5` writer - Capture/Compare value
pub type CCR5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CCR5_SPEC, u16, u16, 16, O>;
///Field `GC5C1` reader - Group Channel 5 and Channel 1
pub type GC5C1_R = crate::BitReader<bool>;
///Field `GC5C1` writer - Group Channel 5 and Channel 1
pub type GC5C1_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR5_SPEC, bool, O>;
///Field `GC5C2` reader - Group Channel 5 and Channel 2
pub type GC5C2_R = crate::BitReader<bool>;
///Field `GC5C2` writer - Group Channel 5 and Channel 2
pub type GC5C2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR5_SPEC, bool, O>;
///Field `GC5C3` reader - Group Channel 5 and Channel 3
pub type GC5C3_R = crate::BitReader<bool>;
///Field `GC5C3` writer - Group Channel 5 and Channel 3
pub type GC5C3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR5_SPEC, bool, O>;
impl R {
    ///Bits 0:15 - Capture/Compare value
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new((self.bits & 0xffff) as u16)
    }
    ///Bit 29 - Group Channel 5 and Channel 1
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Group Channel 5 and Channel 2
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Group Channel 5 and Channel 3
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:15 - Capture/Compare value
    #[inline(always)]
    #[must_use]
    pub fn ccr5(&mut self) -> CCR5_W<0> {
        CCR5_W::new(self)
    }
    ///Bit 29 - Group Channel 5 and Channel 1
    #[inline(always)]
    #[must_use]
    pub fn gc5c1(&mut self) -> GC5C1_W<29> {
        GC5C1_W::new(self)
    }
    ///Bit 30 - Group Channel 5 and Channel 2
    #[inline(always)]
    #[must_use]
    pub fn gc5c2(&mut self) -> GC5C2_W<30> {
        GC5C2_W::new(self)
    }
    ///Bit 31 - Group Channel 5 and Channel 3
    #[inline(always)]
    #[must_use]
    pub fn gc5c3(&mut self) -> GC5C3_W<31> {
        GC5C3_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///capture/compare register 4
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [ccr5](index.html) module
pub struct CCR5_SPEC;
impl crate::RegisterSpec for CCR5_SPEC {
    type Ux = u32;
}
///`read()` method returns [ccr5::R](R) reader structure
impl crate::Readable for CCR5_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [ccr5::W](W) writer structure
impl crate::Writable for CCR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CCR5 to value 0
impl crate::Resettable for CCR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
