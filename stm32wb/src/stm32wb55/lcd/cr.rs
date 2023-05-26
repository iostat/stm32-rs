///Register `CR` reader
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR` writer
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `LCDEN` reader - LCD controller enable
pub type LCDEN_R = crate::BitReader<bool>;
///Field `LCDEN` writer - LCD controller enable
pub type LCDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `VSEL` reader - Voltage source selection
pub type VSEL_R = crate::BitReader<bool>;
///Field `VSEL` writer - Voltage source selection
pub type VSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `DUTY` reader - Duty selection
pub type DUTY_R = crate::FieldReader<u8, u8>;
///Field `DUTY` writer - Duty selection
pub type DUTY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 3, O>;
///Field `BIAS` reader - Bias selector
pub type BIAS_R = crate::FieldReader<u8, u8>;
///Field `BIAS` writer - Bias selector
pub type BIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR_SPEC, u8, u8, 2, O>;
///Field `MUX_SEG` reader - Mux segment enable
pub type MUX_SEG_R = crate::BitReader<bool>;
///Field `MUX_SEG` writer - Mux segment enable
pub type MUX_SEG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
///Field `BUFEN` reader - Voltage output buffer enable
pub type BUFEN_R = crate::BitReader<bool>;
///Field `BUFEN` writer - Voltage output buffer enable
pub type BUFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, bool, O>;
impl R {
    ///Bit 0 - LCD controller enable
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Voltage source selection
    #[inline(always)]
    pub fn vsel(&self) -> VSEL_R {
        VSEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - Duty selection
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(((self.bits >> 2) & 7) as u8)
    }
    ///Bits 5:6 - Bias selector
    #[inline(always)]
    pub fn bias(&self) -> BIAS_R {
        BIAS_R::new(((self.bits >> 5) & 3) as u8)
    }
    ///Bit 7 - Mux segment enable
    #[inline(always)]
    pub fn mux_seg(&self) -> MUX_SEG_R {
        MUX_SEG_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Voltage output buffer enable
    #[inline(always)]
    pub fn bufen(&self) -> BUFEN_R {
        BUFEN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - LCD controller enable
    #[inline(always)]
    #[must_use]
    pub fn lcden(&mut self) -> LCDEN_W<0> {
        LCDEN_W::new(self)
    }
    ///Bit 1 - Voltage source selection
    #[inline(always)]
    #[must_use]
    pub fn vsel(&mut self) -> VSEL_W<1> {
        VSEL_W::new(self)
    }
    ///Bits 2:4 - Duty selection
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<2> {
        DUTY_W::new(self)
    }
    ///Bits 5:6 - Bias selector
    #[inline(always)]
    #[must_use]
    pub fn bias(&mut self) -> BIAS_W<5> {
        BIAS_W::new(self)
    }
    ///Bit 7 - Mux segment enable
    #[inline(always)]
    #[must_use]
    pub fn mux_seg(&mut self) -> MUX_SEG_W<7> {
        MUX_SEG_W::new(self)
    }
    ///Bit 8 - Voltage output buffer enable
    #[inline(always)]
    #[must_use]
    pub fn bufen(&mut self) -> BUFEN_W<8> {
        BUFEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///control register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr](index.html) module
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr::R](R) reader structure
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr::W](W) writer structure
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
