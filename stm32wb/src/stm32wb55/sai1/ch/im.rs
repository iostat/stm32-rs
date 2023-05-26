///Register `IM` reader
pub struct R(crate::R<IM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IM_SPEC>) -> Self {
        R(reader)
    }
}
///Register `IM` writer
pub struct W(crate::W<IM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IM_SPEC>;
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
impl From<crate::W<IM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IM_SPEC>) -> Self {
        W(writer)
    }
}
///Field `OVRUDRIE` reader - Overrun/underrun interrupt enable
pub type OVRUDRIE_R = crate::BitReader<OVRUDRIE_A>;
///Overrun/underrun interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRUDRIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<OVRUDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: OVRUDRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRUDRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OVRUDRIE_A {
        match self.bits {
            false => OVRUDRIE_A::Disabled,
            true => OVRUDRIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRUDRIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRUDRIE_A::Enabled
    }
}
///Field `OVRUDRIE` writer - Overrun/underrun interrupt enable
pub type OVRUDRIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, OVRUDRIE_A, O>;
impl<'a, const O: u8> OVRUDRIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRUDRIE_A::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRUDRIE_A::Enabled)
    }
}
///Field `MUTEDETIE` reader - Mute detection interrupt enable
pub type MUTEDETIE_R = crate::BitReader<MUTEDETIE_A>;
///Mute detection interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTEDETIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<MUTEDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEDETIE_A) -> Self {
        variant as u8 != 0
    }
}
impl MUTEDETIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MUTEDETIE_A {
        match self.bits {
            false => MUTEDETIE_A::Disabled,
            true => MUTEDETIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUTEDETIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUTEDETIE_A::Enabled
    }
}
///Field `MUTEDETIE` writer - Mute detection interrupt enable
pub type MUTEDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, MUTEDETIE_A, O>;
impl<'a, const O: u8> MUTEDETIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUTEDETIE_A::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUTEDETIE_A::Enabled)
    }
}
///Field `WCKCFGIE` reader - Wrong clock configuration interrupt enable
pub type WCKCFGIE_R = crate::BitReader<WCKCFGIE_A>;
///Wrong clock configuration interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCKCFGIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<WCKCFGIE_A> for bool {
    #[inline(always)]
    fn from(variant: WCKCFGIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WCKCFGIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> WCKCFGIE_A {
        match self.bits {
            false => WCKCFGIE_A::Disabled,
            true => WCKCFGIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WCKCFGIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WCKCFGIE_A::Enabled
    }
}
///Field `WCKCFGIE` writer - Wrong clock configuration interrupt enable
pub type WCKCFGIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, WCKCFGIE_A, O>;
impl<'a, const O: u8> WCKCFGIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WCKCFGIE_A::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WCKCFGIE_A::Enabled)
    }
}
///Field `FREQIE` reader - FIFO request interrupt enable
pub type FREQIE_R = crate::BitReader<FREQIE_A>;
///FIFO request interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FREQIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<FREQIE_A> for bool {
    #[inline(always)]
    fn from(variant: FREQIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FREQIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FREQIE_A {
        match self.bits {
            false => FREQIE_A::Disabled,
            true => FREQIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FREQIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FREQIE_A::Enabled
    }
}
///Field `FREQIE` writer - FIFO request interrupt enable
pub type FREQIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, FREQIE_A, O>;
impl<'a, const O: u8> FREQIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FREQIE_A::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FREQIE_A::Enabled)
    }
}
///Field `CNRDYIE` reader - Codec not ready interrupt enable
pub type CNRDYIE_R = crate::BitReader<CNRDYIE_A>;
///Codec not ready interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNRDYIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<CNRDYIE_A> for bool {
    #[inline(always)]
    fn from(variant: CNRDYIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CNRDYIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CNRDYIE_A {
        match self.bits {
            false => CNRDYIE_A::Disabled,
            true => CNRDYIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CNRDYIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CNRDYIE_A::Enabled
    }
}
///Field `CNRDYIE` writer - Codec not ready interrupt enable
pub type CNRDYIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, CNRDYIE_A, O>;
impl<'a, const O: u8> CNRDYIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CNRDYIE_A::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CNRDYIE_A::Enabled)
    }
}
///Field `AFSDETIE` reader - Anticipated frame synchronization detection interrupt enable
pub type AFSDETIE_R = crate::BitReader<AFSDETIE_A>;
///Anticipated frame synchronization detection interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFSDETIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<AFSDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: AFSDETIE_A) -> Self {
        variant as u8 != 0
    }
}
impl AFSDETIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> AFSDETIE_A {
        match self.bits {
            false => AFSDETIE_A::Disabled,
            true => AFSDETIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AFSDETIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AFSDETIE_A::Enabled
    }
}
///Field `AFSDETIE` writer - Anticipated frame synchronization detection interrupt enable
pub type AFSDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, AFSDETIE_A, O>;
impl<'a, const O: u8> AFSDETIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(AFSDETIE_A::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(AFSDETIE_A::Enabled)
    }
}
///Field `LFSDETIE` reader - Late frame synchronization detection interrupt enable
pub type LFSDETIE_R = crate::BitReader<LFSDETIE_A>;
///Late frame synchronization detection interrupt enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LFSDETIE_A {
    ///0: Interrupt is disabled
    Disabled = 0,
    ///1: Interrupt is enabled
    Enabled = 1,
}
impl From<LFSDETIE_A> for bool {
    #[inline(always)]
    fn from(variant: LFSDETIE_A) -> Self {
        variant as u8 != 0
    }
}
impl LFSDETIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LFSDETIE_A {
        match self.bits {
            false => LFSDETIE_A::Disabled,
            true => LFSDETIE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LFSDETIE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == LFSDETIE_A::Enabled
    }
}
///Field `LFSDETIE` writer - Late frame synchronization detection interrupt enable
pub type LFSDETIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IM_SPEC, LFSDETIE_A, O>;
impl<'a, const O: u8> LFSDETIE_W<'a, O> {
    ///Interrupt is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LFSDETIE_A::Disabled)
    }
    ///Interrupt is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(LFSDETIE_A::Enabled)
    }
}
impl R {
    ///Bit 0 - Overrun/underrun interrupt enable
    #[inline(always)]
    pub fn ovrudrie(&self) -> OVRUDRIE_R {
        OVRUDRIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Mute detection interrupt enable
    #[inline(always)]
    pub fn mutedetie(&self) -> MUTEDETIE_R {
        MUTEDETIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Wrong clock configuration interrupt enable
    #[inline(always)]
    pub fn wckcfgie(&self) -> WCKCFGIE_R {
        WCKCFGIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FIFO request interrupt enable
    #[inline(always)]
    pub fn freqie(&self) -> FREQIE_R {
        FREQIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Codec not ready interrupt enable
    #[inline(always)]
    pub fn cnrdyie(&self) -> CNRDYIE_R {
        CNRDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Anticipated frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn afsdetie(&self) -> AFSDETIE_R {
        AFSDETIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Late frame synchronization detection interrupt enable
    #[inline(always)]
    pub fn lfsdetie(&self) -> LFSDETIE_R {
        LFSDETIE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    ///Bit 0 - Overrun/underrun interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn ovrudrie(&mut self) -> OVRUDRIE_W<0> {
        OVRUDRIE_W::new(self)
    }
    ///Bit 1 - Mute detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn mutedetie(&mut self) -> MUTEDETIE_W<1> {
        MUTEDETIE_W::new(self)
    }
    ///Bit 2 - Wrong clock configuration interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn wckcfgie(&mut self) -> WCKCFGIE_W<2> {
        WCKCFGIE_W::new(self)
    }
    ///Bit 3 - FIFO request interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn freqie(&mut self) -> FREQIE_W<3> {
        FREQIE_W::new(self)
    }
    ///Bit 4 - Codec not ready interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn cnrdyie(&mut self) -> CNRDYIE_W<4> {
        CNRDYIE_W::new(self)
    }
    ///Bit 5 - Anticipated frame synchronization detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn afsdetie(&mut self) -> AFSDETIE_W<5> {
        AFSDETIE_W::new(self)
    }
    ///Bit 6 - Late frame synchronization detection interrupt enable
    #[inline(always)]
    #[must_use]
    pub fn lfsdetie(&mut self) -> LFSDETIE_W<6> {
        LFSDETIE_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AInterrupt mask register2
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [im](index.html) module
pub struct IM_SPEC;
impl crate::RegisterSpec for IM_SPEC {
    type Ux = u32;
}
///`read()` method returns [im::R](R) reader structure
impl crate::Readable for IM_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [im::W](W) writer structure
impl crate::Writable for IM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets IM to value 0
impl crate::Resettable for IM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
