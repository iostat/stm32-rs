///Register `VAL` reader
pub struct R(crate::R<VAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VAL_SPEC>) -> Self {
        R(reader)
    }
}
///Register `VAL` writer
pub struct W(crate::W<VAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VAL_SPEC>;
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
impl From<crate::W<VAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VAL_SPEC>) -> Self {
        W(writer)
    }
}
///Field `CURRENT` reader - Current counter value
pub type CURRENT_R = crate::FieldReader<u32, u32>;
///Field `CURRENT` writer - Current counter value
pub type CURRENT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VAL_SPEC, u32, u32, 24, O>;
impl R {
    ///Bits 0:23 - Current counter value
    #[inline(always)]
    pub fn current(&self) -> CURRENT_R {
        CURRENT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    ///Bits 0:23 - Current counter value
    #[inline(always)]
    #[must_use]
    pub fn current(&mut self) -> CURRENT_W<0> {
        CURRENT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///SysTick current value register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [val](index.html) module
pub struct VAL_SPEC;
impl crate::RegisterSpec for VAL_SPEC {
    type Ux = u32;
}
///`read()` method returns [val::R](R) reader structure
impl crate::Readable for VAL_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [val::W](W) writer structure
impl crate::Writable for VAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets VAL to value 0
impl crate::Resettable for VAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
