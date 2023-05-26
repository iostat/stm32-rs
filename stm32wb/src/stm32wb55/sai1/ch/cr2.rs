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
///Field `FTH` reader - FIFO threshold
pub type FTH_R = crate::FieldReader<u8, FTH_A>;
///FIFO threshold
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTH_A {
    ///0: FIFO empty
    Empty = 0,
    ///1: 1⁄4 FIFO
    Quarter1 = 1,
    ///2: 1⁄2 FIFO
    Quarter2 = 2,
    ///3: 3⁄4 FIFO
    Quarter3 = 3,
    ///4: FIFO full
    Full = 4,
}
impl From<FTH_A> for u8 {
    #[inline(always)]
    fn from(variant: FTH_A) -> Self {
        variant as _
    }
}
impl FTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<FTH_A> {
        match self.bits {
            0 => Some(FTH_A::Empty),
            1 => Some(FTH_A::Quarter1),
            2 => Some(FTH_A::Quarter2),
            3 => Some(FTH_A::Quarter3),
            4 => Some(FTH_A::Full),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Empty`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FTH_A::Empty
    }
    ///Checks if the value of the field is `Quarter1`
    #[inline(always)]
    pub fn is_quarter1(&self) -> bool {
        *self == FTH_A::Quarter1
    }
    ///Checks if the value of the field is `Quarter2`
    #[inline(always)]
    pub fn is_quarter2(&self) -> bool {
        *self == FTH_A::Quarter2
    }
    ///Checks if the value of the field is `Quarter3`
    #[inline(always)]
    pub fn is_quarter3(&self) -> bool {
        *self == FTH_A::Quarter3
    }
    ///Checks if the value of the field is `Full`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTH_A::Full
    }
}
///Field `FTH` writer - FIFO threshold
pub type FTH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, FTH_A, 3, O>;
impl<'a, const O: u8> FTH_W<'a, O> {
    ///FIFO empty
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(FTH_A::Empty)
    }
    ///1⁄4 FIFO
    #[inline(always)]
    pub fn quarter1(self) -> &'a mut W {
        self.variant(FTH_A::Quarter1)
    }
    ///1⁄2 FIFO
    #[inline(always)]
    pub fn quarter2(self) -> &'a mut W {
        self.variant(FTH_A::Quarter2)
    }
    ///3⁄4 FIFO
    #[inline(always)]
    pub fn quarter3(self) -> &'a mut W {
        self.variant(FTH_A::Quarter3)
    }
    ///FIFO full
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(FTH_A::Full)
    }
}
///Field `FFLUSH` reader - FIFO flush
pub type FFLUSH_R = crate::BitReader<FFLUSH_A>;
///FIFO flush
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FFLUSH_A {
    ///0: No FIFO flush
    NoFlush = 0,
    ///1: FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared
    Flush = 1,
}
impl From<FFLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: FFLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl FFLUSH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> FFLUSH_A {
        match self.bits {
            false => FFLUSH_A::NoFlush,
            true => FFLUSH_A::Flush,
        }
    }
    ///Checks if the value of the field is `NoFlush`
    #[inline(always)]
    pub fn is_no_flush(&self) -> bool {
        *self == FFLUSH_A::NoFlush
    }
    ///Checks if the value of the field is `Flush`
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == FFLUSH_A::Flush
    }
}
///Field `FFLUSH` writer - FIFO flush
pub type FFLUSH_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, FFLUSH_A, O>;
impl<'a, const O: u8> FFLUSH_W<'a, O> {
    ///No FIFO flush
    #[inline(always)]
    pub fn no_flush(self) -> &'a mut W {
        self.variant(FFLUSH_A::NoFlush)
    }
    ///FIFO flush. Programming this bit to 1 triggers the FIFO Flush. All the internal FIFO pointers (read and write) are cleared
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(FFLUSH_A::Flush)
    }
}
///Field `TRIS` reader - Tristate management on data line
pub type TRIS_R = crate::BitReader<bool>;
///Field `TRIS` writer - Tristate management on data line
pub type TRIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, bool, O>;
///Field `MUTE` reader - Mute
pub type MUTE_R = crate::BitReader<MUTE_A>;
///Mute
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTE_A {
    ///0: No mute mode
    Disabled = 0,
    ///1: Mute mode enabled
    Enabled = 1,
}
impl From<MUTE_A> for bool {
    #[inline(always)]
    fn from(variant: MUTE_A) -> Self {
        variant as u8 != 0
    }
}
impl MUTE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MUTE_A {
        match self.bits {
            false => MUTE_A::Disabled,
            true => MUTE_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUTE_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUTE_A::Enabled
    }
}
///Field `MUTE` writer - Mute
pub type MUTE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, MUTE_A, O>;
impl<'a, const O: u8> MUTE_W<'a, O> {
    ///No mute mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MUTE_A::Disabled)
    }
    ///Mute mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MUTE_A::Enabled)
    }
}
///Field `MUTEVAL` reader - Mute value
pub type MUTEVAL_R = crate::BitReader<MUTEVAL_A>;
///Mute value
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUTEVAL_A {
    ///0: Bit value 0 is sent during the mute mode
    SendZero = 0,
    ///1: Last values are sent during the mute mode
    SendLast = 1,
}
impl From<MUTEVAL_A> for bool {
    #[inline(always)]
    fn from(variant: MUTEVAL_A) -> Self {
        variant as u8 != 0
    }
}
impl MUTEVAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MUTEVAL_A {
        match self.bits {
            false => MUTEVAL_A::SendZero,
            true => MUTEVAL_A::SendLast,
        }
    }
    ///Checks if the value of the field is `SendZero`
    #[inline(always)]
    pub fn is_send_zero(&self) -> bool {
        *self == MUTEVAL_A::SendZero
    }
    ///Checks if the value of the field is `SendLast`
    #[inline(always)]
    pub fn is_send_last(&self) -> bool {
        *self == MUTEVAL_A::SendLast
    }
}
///Field `MUTEVAL` writer - Mute value
pub type MUTEVAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, MUTEVAL_A, O>;
impl<'a, const O: u8> MUTEVAL_W<'a, O> {
    ///Bit value 0 is sent during the mute mode
    #[inline(always)]
    pub fn send_zero(self) -> &'a mut W {
        self.variant(MUTEVAL_A::SendZero)
    }
    ///Last values are sent during the mute mode
    #[inline(always)]
    pub fn send_last(self) -> &'a mut W {
        self.variant(MUTEVAL_A::SendLast)
    }
}
///Field `MUTECNT` reader - Mute counter
pub type MUTECNT_R = crate::FieldReader<u8, u8>;
///Field `MUTECNT` writer - Mute counter
pub type MUTECNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, u8, 6, O>;
///Field `CPL` reader - Complement bit
pub type CPL_R = crate::BitReader<CPL_A>;
///Complement bit
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPL_A {
    ///0: 1’s complement representation
    OnesComplement = 0,
    ///1: 2’s complement representation
    TwosComplement = 1,
}
impl From<CPL_A> for bool {
    #[inline(always)]
    fn from(variant: CPL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CPL_A {
        match self.bits {
            false => CPL_A::OnesComplement,
            true => CPL_A::TwosComplement,
        }
    }
    ///Checks if the value of the field is `OnesComplement`
    #[inline(always)]
    pub fn is_ones_complement(&self) -> bool {
        *self == CPL_A::OnesComplement
    }
    ///Checks if the value of the field is `TwosComplement`
    #[inline(always)]
    pub fn is_twos_complement(&self) -> bool {
        *self == CPL_A::TwosComplement
    }
}
///Field `CPL` writer - Complement bit
pub type CPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR2_SPEC, CPL_A, O>;
impl<'a, const O: u8> CPL_W<'a, O> {
    ///1’s complement representation
    #[inline(always)]
    pub fn ones_complement(self) -> &'a mut W {
        self.variant(CPL_A::OnesComplement)
    }
    ///2’s complement representation
    #[inline(always)]
    pub fn twos_complement(self) -> &'a mut W {
        self.variant(CPL_A::TwosComplement)
    }
}
///Field `COMP` reader - Companding mode
pub type COMP_R = crate::FieldReader<u8, COMP_A>;
///Companding mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMP_A {
    ///0: No companding algorithm
    NoCompanding = 0,
    ///2: μ-Law algorithm
    MuLaw = 2,
    ///3: A-Law algorithm
    Alaw = 3,
}
impl From<COMP_A> for u8 {
    #[inline(always)]
    fn from(variant: COMP_A) -> Self {
        variant as _
    }
}
impl COMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<COMP_A> {
        match self.bits {
            0 => Some(COMP_A::NoCompanding),
            2 => Some(COMP_A::MuLaw),
            3 => Some(COMP_A::Alaw),
            _ => None,
        }
    }
    ///Checks if the value of the field is `NoCompanding`
    #[inline(always)]
    pub fn is_no_companding(&self) -> bool {
        *self == COMP_A::NoCompanding
    }
    ///Checks if the value of the field is `MuLaw`
    #[inline(always)]
    pub fn is_mu_law(&self) -> bool {
        *self == COMP_A::MuLaw
    }
    ///Checks if the value of the field is `Alaw`
    #[inline(always)]
    pub fn is_alaw(&self) -> bool {
        *self == COMP_A::Alaw
    }
}
///Field `COMP` writer - Companding mode
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR2_SPEC, u8, COMP_A, 2, O>;
impl<'a, const O: u8> COMP_W<'a, O> {
    ///No companding algorithm
    #[inline(always)]
    pub fn no_companding(self) -> &'a mut W {
        self.variant(COMP_A::NoCompanding)
    }
    ///μ-Law algorithm
    #[inline(always)]
    pub fn mu_law(self) -> &'a mut W {
        self.variant(COMP_A::MuLaw)
    }
    ///A-Law algorithm
    #[inline(always)]
    pub fn alaw(self) -> &'a mut W {
        self.variant(COMP_A::Alaw)
    }
}
impl R {
    ///Bits 0:2 - FIFO threshold
    #[inline(always)]
    pub fn fth(&self) -> FTH_R {
        FTH_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - FIFO flush
    #[inline(always)]
    pub fn fflush(&self) -> FFLUSH_R {
        FFLUSH_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Tristate management on data line
    #[inline(always)]
    pub fn tris(&self) -> TRIS_R {
        TRIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Mute
    #[inline(always)]
    pub fn mute(&self) -> MUTE_R {
        MUTE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Mute value
    #[inline(always)]
    pub fn muteval(&self) -> MUTEVAL_R {
        MUTEVAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:12 - Mute counter
    #[inline(always)]
    pub fn mutecnt(&self) -> MUTECNT_R {
        MUTECNT_R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    ///Bit 13 - Complement bit
    #[inline(always)]
    pub fn cpl(&self) -> CPL_R {
        CPL_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bits 14:15 - Companding mode
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    ///Bits 0:2 - FIFO threshold
    #[inline(always)]
    #[must_use]
    pub fn fth(&mut self) -> FTH_W<0> {
        FTH_W::new(self)
    }
    ///Bit 3 - FIFO flush
    #[inline(always)]
    #[must_use]
    pub fn fflush(&mut self) -> FFLUSH_W<3> {
        FFLUSH_W::new(self)
    }
    ///Bit 4 - Tristate management on data line
    #[inline(always)]
    #[must_use]
    pub fn tris(&mut self) -> TRIS_W<4> {
        TRIS_W::new(self)
    }
    ///Bit 5 - Mute
    #[inline(always)]
    #[must_use]
    pub fn mute(&mut self) -> MUTE_W<5> {
        MUTE_W::new(self)
    }
    ///Bit 6 - Mute value
    #[inline(always)]
    #[must_use]
    pub fn muteval(&mut self) -> MUTEVAL_W<6> {
        MUTEVAL_W::new(self)
    }
    ///Bits 7:12 - Mute counter
    #[inline(always)]
    #[must_use]
    pub fn mutecnt(&mut self) -> MUTECNT_W<7> {
        MUTECNT_W::new(self)
    }
    ///Bit 13 - Complement bit
    #[inline(always)]
    #[must_use]
    pub fn cpl(&mut self) -> CPL_W<13> {
        CPL_W::new(self)
    }
    ///Bits 14:15 - Companding mode
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<14> {
        COMP_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AConfiguration register 2
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
