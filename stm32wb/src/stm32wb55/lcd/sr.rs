///Register `SR` reader
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
///Register `SR` writer
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
///Field `ENS` reader - ENS
pub type ENS_R = crate::BitReader<bool>;
///Field `SOF` reader - Start of frame flag
pub type SOF_R = crate::BitReader<bool>;
///Field `UDR` reader - Update display request
pub type UDR_R = crate::BitReader<bool>;
///Field `UDR` writer - Update display request
pub type UDR_W<'a, const O: u8> = crate::BitWriter<'a, u32, SR_SPEC, bool, O>;
///Field `UDD` reader - Update Display Done
pub type UDD_R = crate::BitReader<bool>;
///Field `RDY` reader - Ready flag
pub type RDY_R = crate::BitReader<bool>;
///Field `FCRSF` reader - LCD Frame Control Register Synchronization flag
pub type FCRSF_R = crate::BitReader<bool>;
impl R {
    ///Bit 0 - ENS
    #[inline(always)]
    pub fn ens(&self) -> ENS_R {
        ENS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Start of frame flag
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Update display request
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Update Display Done
    #[inline(always)]
    pub fn udd(&self) -> UDD_R {
        UDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Ready flag
    #[inline(always)]
    pub fn rdy(&self) -> RDY_R {
        RDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - LCD Frame Control Register Synchronization flag
    #[inline(always)]
    pub fn fcrsf(&self) -> FCRSF_R {
        FCRSF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    ///Bit 2 - Update display request
    #[inline(always)]
    #[must_use]
    pub fn udr(&mut self) -> UDR_W<2> {
        UDR_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///status register
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [sr](index.html) module
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
///`read()` method returns [sr::R](R) reader structure
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [sr::W](W) writer structure
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets SR to value 0x20
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
