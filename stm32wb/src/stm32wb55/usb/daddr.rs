///Register `DADDR` reader
pub struct R(crate::R<DADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DADDR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `DADDR` writer
pub struct W(crate::W<DADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DADDR_SPEC>;
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
impl From<crate::W<DADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DADDR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ADD` reader - Device address
pub type ADD_R = crate::FieldReader<u8, u8>;
///Field `ADD` writer - Device address
pub type ADD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, DADDR_SPEC, u8, u8, 7, O>;
///Field `EF` reader - Enable function
pub type EF_R = crate::BitReader<bool>;
///Field `EF` writer - Enable function
pub type EF_W<'a, const O: u8> = crate::BitWriter<'a, u16, DADDR_SPEC, bool, O>;
impl R {
    ///Bits 0:6 - Device address
    #[inline(always)]
    pub fn add(&self) -> ADD_R {
        ADD_R::new((self.bits & 0x7f) as u8)
    }
    ///Bit 7 - Enable function
    #[inline(always)]
    pub fn ef(&self) -> EF_R {
        EF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    ///Bits 0:6 - Device address
    #[inline(always)]
    #[must_use]
    pub fn add(&mut self) -> ADD_W<0> {
        ADD_W::new(self)
    }
    ///Bit 7 - Enable function
    #[inline(always)]
    #[must_use]
    pub fn ef(&mut self) -> EF_W<7> {
        EF_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///device address
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [daddr](index.html) module
pub struct DADDR_SPEC;
impl crate::RegisterSpec for DADDR_SPEC {
    type Ux = u16;
}
///`read()` method returns [daddr::R](R) reader structure
impl crate::Readable for DADDR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [daddr::W](W) writer structure
impl crate::Writable for DADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets DADDR to value 0
impl crate::Resettable for DADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
