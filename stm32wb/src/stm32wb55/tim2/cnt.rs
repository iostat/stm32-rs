///Register `CNT` reader
pub struct R(crate::R<CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNT_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CNT` writer
pub struct W(crate::W<CNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNT_SPEC>;
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
impl From<crate::W<CNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNT_SPEC>) -> Self {
        W(writer)
    }
}
///Field `UIFREMAP_CNT` reader - Counter value when CR1.UIFREMAP=1
pub type UIFREMAP_CNT_R = crate::FieldReader<u32, u32>;
///Field `UIFREMAP_CNT` writer - Counter value when CR1.UIFREMAP=1
pub type UIFREMAP_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_SPEC, u32, u32, 31, O>;
///Field `CNT` reader - Counter value
pub type CNT_R = crate::FieldReader<u32, u32>;
///Field `CNT` writer - Counter value
pub type CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CNT_SPEC, u32, u32, 32, O>;
///Field `UIFCPY` reader - Copy of ISR.UIF when CR1.UIFREMAP=1
pub type UIFCPY_R = crate::BitReader<bool>;
impl R {
    ///Bits 0:30 - Counter value when CR1.UIFREMAP=1
    #[inline(always)]
    pub fn uifremap_cnt(&self) -> UIFREMAP_CNT_R {
        UIFREMAP_CNT_R::new(self.bits & 0x7fff_ffff)
    }
    ///Bits 0:31 - Counter value
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits)
    }
    ///Bit 31 - Copy of ISR.UIF when CR1.UIFREMAP=1
    #[inline(always)]
    pub fn uifcpy(&self) -> UIFCPY_R {
        UIFCPY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    ///Bits 0:30 - Counter value when CR1.UIFREMAP=1
    #[inline(always)]
    #[must_use]
    pub fn uifremap_cnt(&mut self) -> UIFREMAP_CNT_W<0> {
        UIFREMAP_CNT_W::new(self)
    }
    ///Bits 0:31 - Counter value
    #[inline(always)]
    #[must_use]
    pub fn cnt(&mut self) -> CNT_W<0> {
        CNT_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///counter
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cnt](index.html) module
pub struct CNT_SPEC;
impl crate::RegisterSpec for CNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [cnt::R](R) reader structure
impl crate::Readable for CNT_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cnt::W](W) writer structure
impl crate::Writable for CNT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CNT to value 0
impl crate::Resettable for CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
