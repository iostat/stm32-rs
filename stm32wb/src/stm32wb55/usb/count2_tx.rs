///Register `COUNT2_TX` reader
pub struct R(crate::R<COUNT2_TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT2_TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT2_TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT2_TX_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COUNT2_TX` writer
pub struct W(crate::W<COUNT2_TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT2_TX_SPEC>;
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
impl From<crate::W<COUNT2_TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT2_TX_SPEC>) -> Self {
        W(writer)
    }
}
///Field `COUNT2_TX` reader - Transmission byte count
pub type COUNT2_TX_R = crate::FieldReader<u16, u16>;
///Field `COUNT2_TX` writer - Transmission byte count
pub type COUNT2_TX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, COUNT2_TX_SPEC, u16, u16, 10, O>;
impl R {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    pub fn count2_tx(&self) -> COUNT2_TX_R {
        COUNT2_TX_R::new(self.bits & 0x03ff)
    }
}
impl W {
    ///Bits 0:9 - Transmission byte count
    #[inline(always)]
    #[must_use]
    pub fn count2_tx(&mut self) -> COUNT2_TX_W<0> {
        COUNT2_TX_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Transmission byte count 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [count2_tx](index.html) module
pub struct COUNT2_TX_SPEC;
impl crate::RegisterSpec for COUNT2_TX_SPEC {
    type Ux = u16;
}
///`read()` method returns [count2_tx::R](R) reader structure
impl crate::Readable for COUNT2_TX_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [count2_tx::W](W) writer structure
impl crate::Writable for COUNT2_TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets COUNT2_TX to value 0
impl crate::Resettable for COUNT2_TX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}