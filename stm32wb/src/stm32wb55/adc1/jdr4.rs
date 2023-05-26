///Register `JDR4` reader
pub struct R(crate::R<JDR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<JDR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<JDR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<JDR4_SPEC>) -> Self {
        R(reader)
    }
}
///Field `JDATA4` reader - ADC group injected sequencer rank 4 conversion data
pub type JDATA4_R = crate::FieldReader<u16, u16>;
impl R {
    ///Bits 0:15 - ADC group injected sequencer rank 4 conversion data
    #[inline(always)]
    pub fn jdata4(&self) -> JDATA4_R {
        JDATA4_R::new((self.bits & 0xffff) as u16)
    }
}
///ADC group injected sequencer rank 4 register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [jdr4](index.html) module
pub struct JDR4_SPEC;
impl crate::RegisterSpec for JDR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [jdr4::R](R) reader structure
impl crate::Readable for JDR4_SPEC {
    type Reader = R;
}
///`reset()` method sets JDR4 to value 0
impl crate::Resettable for JDR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
