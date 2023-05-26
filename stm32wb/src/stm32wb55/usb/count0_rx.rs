///Register `COUNT0_RX` reader
pub struct R(crate::R<COUNT0_RX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COUNT0_RX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COUNT0_RX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COUNT0_RX_SPEC>) -> Self {
        R(reader)
    }
}
///Register `COUNT0_RX` writer
pub struct W(crate::W<COUNT0_RX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COUNT0_RX_SPEC>;
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
impl From<crate::W<COUNT0_RX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COUNT0_RX_SPEC>) -> Self {
        W(writer)
    }
}
///Field `COUNT0_RX` reader - Reception byte count
pub type COUNT0_RX_R = crate::FieldReader<u16, u16>;
///Field `NUM_BLOCK` reader - Number of blocks
pub type NUM_BLOCK_R = crate::FieldReader<u8, u8>;
///Field `NUM_BLOCK` writer - Number of blocks
pub type NUM_BLOCK_W<'a, const O: u8> = crate::FieldWriter<'a, u16, COUNT0_RX_SPEC, u8, u8, 5, O>;
///Field `BL_SIZE` reader - Block size
pub type BL_SIZE_R = crate::BitReader<bool>;
///Field `BL_SIZE` writer - Block size
pub type BL_SIZE_W<'a, const O: u8> = crate::BitWriter<'a, u16, COUNT0_RX_SPEC, bool, O>;
impl R {
    ///Bits 0:9 - Reception byte count
    #[inline(always)]
    pub fn count0_rx(&self) -> COUNT0_RX_R {
        COUNT0_RX_R::new(self.bits & 0x03ff)
    }
    ///Bits 10:14 - Number of blocks
    #[inline(always)]
    pub fn num_block(&self) -> NUM_BLOCK_R {
        NUM_BLOCK_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    ///Bit 15 - Block size
    #[inline(always)]
    pub fn bl_size(&self) -> BL_SIZE_R {
        BL_SIZE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    ///Bits 10:14 - Number of blocks
    #[inline(always)]
    #[must_use]
    pub fn num_block(&mut self) -> NUM_BLOCK_W<10> {
        NUM_BLOCK_W::new(self)
    }
    ///Bit 15 - Block size
    #[inline(always)]
    #[must_use]
    pub fn bl_size(&mut self) -> BL_SIZE_W<15> {
        BL_SIZE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Reception byte count 0
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [count0_rx](index.html) module
pub struct COUNT0_RX_SPEC;
impl crate::RegisterSpec for COUNT0_RX_SPEC {
    type Ux = u16;
}
///`read()` method returns [count0_rx::R](R) reader structure
impl crate::Readable for COUNT0_RX_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [count0_rx::W](W) writer structure
impl crate::Writable for COUNT0_RX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets COUNT0_RX to value 0
impl crate::Resettable for COUNT0_RX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
