///Register `CR2` reader
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR2` writer
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SADD` reader - Slave address bit (master mode)
pub type SADD_R = crate::FieldReader<u16, u16>;
///Field `SADD` writer - Slave address bit (master mode)
pub type SADD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u16, u16, 10, O>;
///Field `RD_WRN` reader - Transfer direction (master mode)
pub type RD_WRN_R = crate::BitReader<bool>;
///Field `RD_WRN` writer - Transfer direction (master mode)
pub type RD_WRN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `ADD10` reader - 10-bit addressing mode (master mode)
pub type ADD10_R = crate::BitReader<bool>;
///Field `ADD10` writer - 10-bit addressing mode (master mode)
pub type ADD10_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode)
pub type HEAD10R_R = crate::BitReader<bool>;
///Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode)
pub type HEAD10R_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `START` reader - Start generation
pub type START_R = crate::BitReader<bool>;
///Field `START` writer - Start generation
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `STOP` reader - Stop generation (master mode)
pub type STOP_R = crate::BitReader<bool>;
///Field `STOP` writer - Stop generation (master mode)
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `NACK` reader - NACK generation (slave mode)
pub type NACK_R = crate::BitReader<bool>;
///Field `NACK` writer - NACK generation (slave mode)
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `NBYTES` reader - Number of bytes
pub type NBYTES_R = crate::FieldReader<u8, u8>;
///Field `NBYTES` writer - Number of bytes
pub type NBYTES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 8, O>;
///Field `RELOAD` reader - NBYTES reload mode
pub type RELOAD_R = crate::BitReader<bool>;
///Field `RELOAD` writer - NBYTES reload mode
pub type RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `AUTOEND` reader - Automatic end mode (master mode)
pub type AUTOEND_R = crate::BitReader<bool>;
///Field `AUTOEND` writer - Automatic end mode (master mode)
pub type AUTOEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `PECBYTE` reader - Packet error checking byte
pub type PECBYTE_R = crate::BitReader<bool>;
///Field `PECBYTE` writer - Packet error checking byte
pub type PECBYTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
impl R {
    ///Bits 0:9 - Slave address bit (master mode)
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    ///Bit 10 - Transfer direction (master mode)
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - 10-bit addressing mode (master mode)
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode)
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Start generation
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Stop generation (master mode)
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - NACK generation (slave mode)
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:23 - Number of bytes
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 24 - NBYTES reload mode
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Automatic end mode (master mode)
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Packet error checking byte
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    ///Bits 0:9 - Slave address bit (master mode)
    #[inline(always)]
    #[must_use]
    pub fn sadd(&mut self) -> SADD_W<0> {
        SADD_W::new(self)
    }
    ///Bit 10 - Transfer direction (master mode)
    #[inline(always)]
    #[must_use]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<10> {
        RD_WRN_W::new(self)
    }
    ///Bit 11 - 10-bit addressing mode (master mode)
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<11> {
        ADD10_W::new(self)
    }
    ///Bit 12 - 10-bit address header only read direction (master receiver mode)
    #[inline(always)]
    #[must_use]
    pub fn head10r(&mut self) -> HEAD10R_W<12> {
        HEAD10R_W::new(self)
    }
    ///Bit 13 - Start generation
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<13> {
        START_W::new(self)
    }
    ///Bit 14 - Stop generation (master mode)
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<14> {
        STOP_W::new(self)
    }
    ///Bit 15 - NACK generation (slave mode)
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<15> {
        NACK_W::new(self)
    }
    ///Bits 16:23 - Number of bytes
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<16> {
        NBYTES_W::new(self)
    }
    ///Bit 24 - NBYTES reload mode
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<24> {
        RELOAD_W::new(self)
    }
    ///Bit 25 - Automatic end mode (master mode)
    #[inline(always)]
    #[must_use]
    pub fn autoend(&mut self) -> AUTOEND_W<25> {
        AUTOEND_W::new(self)
    }
    ///Bit 26 - Packet error checking byte
    #[inline(always)]
    #[must_use]
    pub fn pecbyte(&mut self) -> PECBYTE_W<26> {
        PECBYTE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///Control register 2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr2](index.html) module
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr2::R](R) reader structure
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr2::W](W) writer structure
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
