///Register `TR` reader
pub struct R(crate::R<TR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `TR` writer
pub struct W(crate::W<TR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TR_SPEC>;
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
impl From<crate::W<TR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `SU` reader - Second units in BCD format
pub type SU_R = crate::FieldReader<u8, u8>;
///Field `SU` writer - Second units in BCD format
pub type SU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 4, O>;
///Field `ST` reader - Second tens in BCD format
pub type ST_R = crate::FieldReader<u8, u8>;
///Field `ST` writer - Second tens in BCD format
pub type ST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 3, O>;
///Field `MNU` reader - Minute units in BCD format
pub type MNU_R = crate::FieldReader<u8, u8>;
///Field `MNU` writer - Minute units in BCD format
pub type MNU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 4, O>;
///Field `MNT` reader - Minute tens in BCD format
pub type MNT_R = crate::FieldReader<u8, u8>;
///Field `MNT` writer - Minute tens in BCD format
pub type MNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 3, O>;
///Field `HU` reader - Hour units in BCD format
pub type HU_R = crate::FieldReader<u8, u8>;
///Field `HU` writer - Hour units in BCD format
pub type HU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 4, O>;
///Field `HT` reader - Hour tens in BCD format
pub type HT_R = crate::FieldReader<u8, u8>;
///Field `HT` writer - Hour tens in BCD format
pub type HT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TR_SPEC, u8, u8, 2, O>;
///Field `PM` reader - AM/PM notation
pub type PM_R = crate::BitReader<bool>;
///Field `PM` writer - AM/PM notation
pub type PM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TR_SPEC, bool, O>;
impl R {
    ///Bits 0:3 - Second units in BCD format
    #[inline(always)]
    pub fn su(&self) -> SU_R {
        SU_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:6 - Second tens in BCD format
    #[inline(always)]
    pub fn st(&self) -> ST_R {
        ST_R::new(((self.bits >> 4) & 7) as u8)
    }
    ///Bits 8:11 - Minute units in BCD format
    #[inline(always)]
    pub fn mnu(&self) -> MNU_R {
        MNU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:14 - Minute tens in BCD format
    #[inline(always)]
    pub fn mnt(&self) -> MNT_R {
        MNT_R::new(((self.bits >> 12) & 7) as u8)
    }
    ///Bits 16:19 - Hour units in BCD format
    #[inline(always)]
    pub fn hu(&self) -> HU_R {
        HU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:21 - Hour tens in BCD format
    #[inline(always)]
    pub fn ht(&self) -> HT_R {
        HT_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - AM/PM notation
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    ///Bits 0:3 - Second units in BCD format
    #[inline(always)]
    #[must_use]
    pub fn su(&mut self) -> SU_W<0> {
        SU_W::new(self)
    }
    ///Bits 4:6 - Second tens in BCD format
    #[inline(always)]
    #[must_use]
    pub fn st(&mut self) -> ST_W<4> {
        ST_W::new(self)
    }
    ///Bits 8:11 - Minute units in BCD format
    #[inline(always)]
    #[must_use]
    pub fn mnu(&mut self) -> MNU_W<8> {
        MNU_W::new(self)
    }
    ///Bits 12:14 - Minute tens in BCD format
    #[inline(always)]
    #[must_use]
    pub fn mnt(&mut self) -> MNT_W<12> {
        MNT_W::new(self)
    }
    ///Bits 16:19 - Hour units in BCD format
    #[inline(always)]
    #[must_use]
    pub fn hu(&mut self) -> HU_W<16> {
        HU_W::new(self)
    }
    ///Bits 20:21 - Hour tens in BCD format
    #[inline(always)]
    #[must_use]
    pub fn ht(&mut self) -> HT_W<20> {
        HT_W::new(self)
    }
    ///Bit 22 - AM/PM notation
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<22> {
        PM_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///time register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [tr](index.html) module
pub struct TR_SPEC;
impl crate::RegisterSpec for TR_SPEC {
    type Ux = u32;
}
///`read()` method returns [tr::R](R) reader structure
impl crate::Readable for TR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [tr::W](W) writer structure
impl crate::Writable for TR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets TR to value 0
impl crate::Resettable for TR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
