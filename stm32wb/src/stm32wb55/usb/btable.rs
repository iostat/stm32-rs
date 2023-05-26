///Register `BTABLE` reader
pub struct R(crate::R<BTABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BTABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BTABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BTABLE_SPEC>) -> Self {
        R(reader)
    }
}
///Register `BTABLE` writer
pub struct W(crate::W<BTABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BTABLE_SPEC>;
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
impl From<crate::W<BTABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BTABLE_SPEC>) -> Self {
        W(writer)
    }
}
///Field `BTABLE` reader - Buffer table
pub type BTABLE_R = crate::FieldReader<u16, u16>;
///Field `BTABLE` writer - Buffer table
pub type BTABLE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, BTABLE_SPEC, u16, u16, 13, O>;
impl R {
    ///Bits 3:15 - Buffer table
    #[inline(always)]
    pub fn btable(&self) -> BTABLE_R {
        BTABLE_R::new((self.bits >> 3) & 0x1fff)
    }
}
impl W {
    ///Bits 3:15 - Buffer table
    #[inline(always)]
    #[must_use]
    pub fn btable(&mut self) -> BTABLE_W<3> {
        BTABLE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Buffer table address
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [btable](index.html) module
pub struct BTABLE_SPEC;
impl crate::RegisterSpec for BTABLE_SPEC {
    type Ux = u16;
}
///`read()` method returns [btable::R](R) reader structure
impl crate::Readable for BTABLE_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [btable::W](W) writer structure
impl crate::Writable for BTABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets BTABLE to value 0
impl crate::Resettable for BTABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
