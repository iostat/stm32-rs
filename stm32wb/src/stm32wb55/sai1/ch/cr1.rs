///Register `CR1` reader
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
///Register `CR1` writer
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
///Field `MODE` reader - Audio block mode
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
///Audio block mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    ///0: Master transmitter
    MasterTx = 0,
    ///1: Master receiver
    MasterRx = 1,
    ///2: Slave transmitter
    SlaveTx = 2,
    ///3: Slave receiver
    SlaveRx = 3,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MODE_A {
        match self.bits {
            0 => MODE_A::MasterTx,
            1 => MODE_A::MasterRx,
            2 => MODE_A::SlaveTx,
            3 => MODE_A::SlaveRx,
            _ => unreachable!(),
        }
    }
    ///Checks if the value of the field is `MasterTx`
    #[inline(always)]
    pub fn is_master_tx(&self) -> bool {
        *self == MODE_A::MasterTx
    }
    ///Checks if the value of the field is `MasterRx`
    #[inline(always)]
    pub fn is_master_rx(&self) -> bool {
        *self == MODE_A::MasterRx
    }
    ///Checks if the value of the field is `SlaveTx`
    #[inline(always)]
    pub fn is_slave_tx(&self) -> bool {
        *self == MODE_A::SlaveTx
    }
    ///Checks if the value of the field is `SlaveRx`
    #[inline(always)]
    pub fn is_slave_rx(&self) -> bool {
        *self == MODE_A::SlaveRx
    }
}
///Field `MODE` writer - Audio block mode
pub type MODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CR1_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    ///Master transmitter
    #[inline(always)]
    pub fn master_tx(self) -> &'a mut W {
        self.variant(MODE_A::MasterTx)
    }
    ///Master receiver
    #[inline(always)]
    pub fn master_rx(self) -> &'a mut W {
        self.variant(MODE_A::MasterRx)
    }
    ///Slave transmitter
    #[inline(always)]
    pub fn slave_tx(self) -> &'a mut W {
        self.variant(MODE_A::SlaveTx)
    }
    ///Slave receiver
    #[inline(always)]
    pub fn slave_rx(self) -> &'a mut W {
        self.variant(MODE_A::SlaveRx)
    }
}
///Field `PRTCFG` reader - Protocol configuration
pub type PRTCFG_R = crate::FieldReader<u8, PRTCFG_A>;
///Protocol configuration
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PRTCFG_A {
    ///0: Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol
    Free = 0,
    ///1: SPDIF protocol
    Spdif = 1,
    ///2: AC’97 protocol
    Ac97 = 2,
}
impl From<PRTCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PRTCFG_A) -> Self {
        variant as _
    }
}
impl PRTCFG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<PRTCFG_A> {
        match self.bits {
            0 => Some(PRTCFG_A::Free),
            1 => Some(PRTCFG_A::Spdif),
            2 => Some(PRTCFG_A::Ac97),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Free`
    #[inline(always)]
    pub fn is_free(&self) -> bool {
        *self == PRTCFG_A::Free
    }
    ///Checks if the value of the field is `Spdif`
    #[inline(always)]
    pub fn is_spdif(&self) -> bool {
        *self == PRTCFG_A::Spdif
    }
    ///Checks if the value of the field is `Ac97`
    #[inline(always)]
    pub fn is_ac97(&self) -> bool {
        *self == PRTCFG_A::Ac97
    }
}
///Field `PRTCFG` writer - Protocol configuration
pub type PRTCFG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, PRTCFG_A, 2, O>;
impl<'a, const O: u8> PRTCFG_W<'a, O> {
    ///Free protocol. Free protocol allows to use the powerful configuration of the audio block to address a specific audio protocol
    #[inline(always)]
    pub fn free(self) -> &'a mut W {
        self.variant(PRTCFG_A::Free)
    }
    ///SPDIF protocol
    #[inline(always)]
    pub fn spdif(self) -> &'a mut W {
        self.variant(PRTCFG_A::Spdif)
    }
    ///AC’97 protocol
    #[inline(always)]
    pub fn ac97(self) -> &'a mut W {
        self.variant(PRTCFG_A::Ac97)
    }
}
///Field `DS` reader - Data size
pub type DS_R = crate::FieldReader<u8, DS_A>;
///Data size
///
///Value on reset: 2
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DS_A {
    ///2: 8 bits
    Bit8 = 2,
    ///3: 10 bits
    Bit10 = 3,
    ///4: 16 bits
    Bit16 = 4,
    ///5: 20 bits
    Bit20 = 5,
    ///6: 24 bits
    Bit24 = 6,
    ///7: 32 bits
    Bit32 = 7,
}
impl From<DS_A> for u8 {
    #[inline(always)]
    fn from(variant: DS_A) -> Self {
        variant as _
    }
}
impl DS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<DS_A> {
        match self.bits {
            2 => Some(DS_A::Bit8),
            3 => Some(DS_A::Bit10),
            4 => Some(DS_A::Bit16),
            5 => Some(DS_A::Bit20),
            6 => Some(DS_A::Bit24),
            7 => Some(DS_A::Bit32),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Bit8`
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == DS_A::Bit8
    }
    ///Checks if the value of the field is `Bit10`
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == DS_A::Bit10
    }
    ///Checks if the value of the field is `Bit16`
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == DS_A::Bit16
    }
    ///Checks if the value of the field is `Bit20`
    #[inline(always)]
    pub fn is_bit20(&self) -> bool {
        *self == DS_A::Bit20
    }
    ///Checks if the value of the field is `Bit24`
    #[inline(always)]
    pub fn is_bit24(&self) -> bool {
        *self == DS_A::Bit24
    }
    ///Checks if the value of the field is `Bit32`
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == DS_A::Bit32
    }
}
///Field `DS` writer - Data size
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, DS_A, 3, O>;
impl<'a, const O: u8> DS_W<'a, O> {
    ///8 bits
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(DS_A::Bit8)
    }
    ///10 bits
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(DS_A::Bit10)
    }
    ///16 bits
    #[inline(always)]
    pub fn bit16(self) -> &'a mut W {
        self.variant(DS_A::Bit16)
    }
    ///20 bits
    #[inline(always)]
    pub fn bit20(self) -> &'a mut W {
        self.variant(DS_A::Bit20)
    }
    ///24 bits
    #[inline(always)]
    pub fn bit24(self) -> &'a mut W {
        self.variant(DS_A::Bit24)
    }
    ///32 bits
    #[inline(always)]
    pub fn bit32(self) -> &'a mut W {
        self.variant(DS_A::Bit32)
    }
}
///Field `LSBFIRST` reader - Least significant bit first
pub type LSBFIRST_R = crate::BitReader<LSBFIRST_A>;
///Least significant bit first
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFIRST_A {
    ///0: Data are transferred with MSB first
    MsbFirst = 0,
    ///1: Data are transferred with LSB first
    LsbFirst = 1,
}
impl From<LSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl LSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> LSBFIRST_A {
        match self.bits {
            false => LSBFIRST_A::MsbFirst,
            true => LSBFIRST_A::LsbFirst,
        }
    }
    ///Checks if the value of the field is `MsbFirst`
    #[inline(always)]
    pub fn is_msb_first(&self) -> bool {
        *self == LSBFIRST_A::MsbFirst
    }
    ///Checks if the value of the field is `LsbFirst`
    #[inline(always)]
    pub fn is_lsb_first(&self) -> bool {
        *self == LSBFIRST_A::LsbFirst
    }
}
///Field `LSBFIRST` writer - Least significant bit first
pub type LSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, LSBFIRST_A, O>;
impl<'a, const O: u8> LSBFIRST_W<'a, O> {
    ///Data are transferred with MSB first
    #[inline(always)]
    pub fn msb_first(self) -> &'a mut W {
        self.variant(LSBFIRST_A::MsbFirst)
    }
    ///Data are transferred with LSB first
    #[inline(always)]
    pub fn lsb_first(self) -> &'a mut W {
        self.variant(LSBFIRST_A::LsbFirst)
    }
}
///Field `CKSTR` reader - Clock strobing edge
pub type CKSTR_R = crate::BitReader<CKSTR_A>;
///Clock strobing edge
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CKSTR_A {
    ///0: Data strobing edge is falling edge of SCK
    FallingEdge = 0,
    ///1: Data strobing edge is rising edge of SCK
    RisingEdge = 1,
}
impl From<CKSTR_A> for bool {
    #[inline(always)]
    fn from(variant: CKSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl CKSTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> CKSTR_A {
        match self.bits {
            false => CKSTR_A::FallingEdge,
            true => CKSTR_A::RisingEdge,
        }
    }
    ///Checks if the value of the field is `FallingEdge`
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CKSTR_A::FallingEdge
    }
    ///Checks if the value of the field is `RisingEdge`
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CKSTR_A::RisingEdge
    }
}
///Field `CKSTR` writer - Clock strobing edge
pub type CKSTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, CKSTR_A, O>;
impl<'a, const O: u8> CKSTR_W<'a, O> {
    ///Data strobing edge is falling edge of SCK
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut W {
        self.variant(CKSTR_A::FallingEdge)
    }
    ///Data strobing edge is rising edge of SCK
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut W {
        self.variant(CKSTR_A::RisingEdge)
    }
}
///Field `SYNCEN` reader - Synchronization enable
pub type SYNCEN_R = crate::FieldReader<u8, SYNCEN_A>;
///Synchronization enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYNCEN_A {
    ///0: audio sub-block in asynchronous mode
    Asynchronous = 0,
    ///1: audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode
    Internal = 1,
    ///2: audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode
    External = 2,
}
impl From<SYNCEN_A> for u8 {
    #[inline(always)]
    fn from(variant: SYNCEN_A) -> Self {
        variant as _
    }
}
impl SYNCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> Option<SYNCEN_A> {
        match self.bits {
            0 => Some(SYNCEN_A::Asynchronous),
            1 => Some(SYNCEN_A::Internal),
            2 => Some(SYNCEN_A::External),
            _ => None,
        }
    }
    ///Checks if the value of the field is `Asynchronous`
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == SYNCEN_A::Asynchronous
    }
    ///Checks if the value of the field is `Internal`
    #[inline(always)]
    pub fn is_internal(&self) -> bool {
        *self == SYNCEN_A::Internal
    }
    ///Checks if the value of the field is `External`
    #[inline(always)]
    pub fn is_external(&self) -> bool {
        *self == SYNCEN_A::External
    }
}
///Field `SYNCEN` writer - Synchronization enable
pub type SYNCEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, SYNCEN_A, 2, O>;
impl<'a, const O: u8> SYNCEN_W<'a, O> {
    ///audio sub-block in asynchronous mode
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(SYNCEN_A::Asynchronous)
    }
    ///audio sub-block is synchronous with the other internal audio sub-block. In this case, the audio sub-block must be configured in slave mode
    #[inline(always)]
    pub fn internal(self) -> &'a mut W {
        self.variant(SYNCEN_A::Internal)
    }
    ///audio sub-block is synchronous with an external SAI embedded peripheral. In this case the audio sub-block should be configured in Slave mode
    #[inline(always)]
    pub fn external(self) -> &'a mut W {
        self.variant(SYNCEN_A::External)
    }
}
///Field `MONO` reader - Mono mode
pub type MONO_R = crate::BitReader<MONO_A>;
///Mono mode
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MONO_A {
    ///0: Stereo mode
    Stereo = 0,
    ///1: Mono mode
    Mono = 1,
}
impl From<MONO_A> for bool {
    #[inline(always)]
    fn from(variant: MONO_A) -> Self {
        variant as u8 != 0
    }
}
impl MONO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> MONO_A {
        match self.bits {
            false => MONO_A::Stereo,
            true => MONO_A::Mono,
        }
    }
    ///Checks if the value of the field is `Stereo`
    #[inline(always)]
    pub fn is_stereo(&self) -> bool {
        *self == MONO_A::Stereo
    }
    ///Checks if the value of the field is `Mono`
    #[inline(always)]
    pub fn is_mono(&self) -> bool {
        *self == MONO_A::Mono
    }
}
///Field `MONO` writer - Mono mode
pub type MONO_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, MONO_A, O>;
impl<'a, const O: u8> MONO_W<'a, O> {
    ///Stereo mode
    #[inline(always)]
    pub fn stereo(self) -> &'a mut W {
        self.variant(MONO_A::Stereo)
    }
    ///Mono mode
    #[inline(always)]
    pub fn mono(self) -> &'a mut W {
        self.variant(MONO_A::Mono)
    }
}
///Field `OUTDRIV` reader - Output drive
pub type OUTDRIV_R = crate::BitReader<OUTDRIV_A>;
///Output drive
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTDRIV_A {
    ///0: Audio block output driven when SAIEN is set
    OnStart = 0,
    ///1: Audio block output driven immediately after the setting of this bit
    Immediately = 1,
}
impl From<OUTDRIV_A> for bool {
    #[inline(always)]
    fn from(variant: OUTDRIV_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTDRIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> OUTDRIV_A {
        match self.bits {
            false => OUTDRIV_A::OnStart,
            true => OUTDRIV_A::Immediately,
        }
    }
    ///Checks if the value of the field is `OnStart`
    #[inline(always)]
    pub fn is_on_start(&self) -> bool {
        *self == OUTDRIV_A::OnStart
    }
    ///Checks if the value of the field is `Immediately`
    #[inline(always)]
    pub fn is_immediately(&self) -> bool {
        *self == OUTDRIV_A::Immediately
    }
}
///Field `OUTDRIV` writer - Output drive
pub type OUTDRIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, OUTDRIV_A, O>;
impl<'a, const O: u8> OUTDRIV_W<'a, O> {
    ///Audio block output driven when SAIEN is set
    #[inline(always)]
    pub fn on_start(self) -> &'a mut W {
        self.variant(OUTDRIV_A::OnStart)
    }
    ///Audio block output driven immediately after the setting of this bit
    #[inline(always)]
    pub fn immediately(self) -> &'a mut W {
        self.variant(OUTDRIV_A::Immediately)
    }
}
///Field `SAIEN` reader - Audio block B enable
pub type SAIEN_R = crate::BitReader<SAIEN_A>;
///Audio block B enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAIEN_A {
    ///0: SAI audio block disabled
    Disabled = 0,
    ///1: SAI audio block enabled
    Enabled = 1,
}
impl From<SAIEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAIEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SAIEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> SAIEN_A {
        match self.bits {
            false => SAIEN_A::Disabled,
            true => SAIEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SAIEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SAIEN_A::Enabled
    }
}
///Field `SAIEN` writer - Audio block B enable
pub type SAIEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, SAIEN_A, O>;
impl<'a, const O: u8> SAIEN_W<'a, O> {
    ///SAI audio block disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SAIEN_A::Disabled)
    }
    ///SAI audio block enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SAIEN_A::Enabled)
    }
}
///Field `DMAEN` reader - DMA enable
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
///DMA enable
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    ///0: DMA disabled
    Disabled = 0,
    ///1: DMA enabled
    Enabled = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::Disabled,
            true => DMAEN_A::Enabled,
        }
    }
    ///Checks if the value of the field is `Disabled`
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAEN_A::Disabled
    }
    ///Checks if the value of the field is `Enabled`
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAEN_A::Enabled
    }
}
///Field `DMAEN` writer - DMA enable
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    ///DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Disabled)
    }
    ///DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAEN_A::Enabled)
    }
}
///Field `NODIV` reader - No divider
pub type NODIV_R = crate::BitReader<NODIV_A>;
///No divider
///
///Value on reset: 0
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NODIV_A {
    ///0: MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value
    MasterClock = 0,
    ///1: MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL.
    NoDiv = 1,
}
impl From<NODIV_A> for bool {
    #[inline(always)]
    fn from(variant: NODIV_A) -> Self {
        variant as u8 != 0
    }
}
impl NODIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub fn variant(&self) -> NODIV_A {
        match self.bits {
            false => NODIV_A::MasterClock,
            true => NODIV_A::NoDiv,
        }
    }
    ///Checks if the value of the field is `MasterClock`
    #[inline(always)]
    pub fn is_master_clock(&self) -> bool {
        *self == NODIV_A::MasterClock
    }
    ///Checks if the value of the field is `NoDiv`
    #[inline(always)]
    pub fn is_no_div(&self) -> bool {
        *self == NODIV_A::NoDiv
    }
}
///Field `NODIV` writer - No divider
pub type NODIV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, NODIV_A, O>;
impl<'a, const O: u8> NODIV_W<'a, O> {
    ///MCLK output is enabled. Forces the ratio between FS and MCLK to 256 or 512 according to the OSR value
    #[inline(always)]
    pub fn master_clock(self) -> &'a mut W {
        self.variant(NODIV_A::MasterClock)
    }
    ///MCLK output enable set by the MCKEN bit (where present, else 0). Ratio between FS and MCLK depends on FRL.
    #[inline(always)]
    pub fn no_div(self) -> &'a mut W {
        self.variant(NODIV_A::NoDiv)
    }
}
///Field `MCKDIV` reader - Master clock divider
pub type MCKDIV_R = crate::FieldReader<u8, u8>;
///Field `MCKDIV` writer - Master clock divider
pub type MCKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CR1_SPEC, u8, u8, 6, O>;
///Field `OSR` reader - Oversampling ratio for master clock
pub type OSR_R = crate::BitReader<bool>;
///Field `OSR` writer - Oversampling ratio for master clock
pub type OSR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
///Field `MCKEN` reader - Master clock generation enable
pub type MCKEN_R = crate::BitReader<bool>;
///Field `MCKEN` writer - Master clock generation enable
pub type MCKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR1_SPEC, bool, O>;
impl R {
    ///Bits 0:1 - Audio block mode
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Protocol configuration
    #[inline(always)]
    pub fn prtcfg(&self) -> PRTCFG_R {
        PRTCFG_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 5:7 - Data size
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bit 8 - Least significant bit first
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Clock strobing edge
    #[inline(always)]
    pub fn ckstr(&self) -> CKSTR_R {
        CKSTR_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bits 10:11 - Synchronization enable
    #[inline(always)]
    pub fn syncen(&self) -> SYNCEN_R {
        SYNCEN_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bit 12 - Mono mode
    #[inline(always)]
    pub fn mono(&self) -> MONO_R {
        MONO_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Output drive
    #[inline(always)]
    pub fn outdriv(&self) -> OUTDRIV_R {
        OUTDRIV_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - Audio block B enable
    #[inline(always)]
    pub fn saien(&self) -> SAIEN_R {
        SAIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - DMA enable
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - No divider
    #[inline(always)]
    pub fn nodiv(&self) -> NODIV_R {
        NODIV_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bits 20:25 - Master clock divider
    #[inline(always)]
    pub fn mckdiv(&self) -> MCKDIV_R {
        MCKDIV_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    ///Bit 26 - Oversampling ratio for master clock
    #[inline(always)]
    pub fn osr(&self) -> OSR_R {
        OSR_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Master clock generation enable
    #[inline(always)]
    pub fn mcken(&self) -> MCKEN_R {
        MCKEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    ///Bits 0:1 - Audio block mode
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<0> {
        MODE_W::new(self)
    }
    ///Bits 2:3 - Protocol configuration
    #[inline(always)]
    #[must_use]
    pub fn prtcfg(&mut self) -> PRTCFG_W<2> {
        PRTCFG_W::new(self)
    }
    ///Bits 5:7 - Data size
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<5> {
        DS_W::new(self)
    }
    ///Bit 8 - Least significant bit first
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<8> {
        LSBFIRST_W::new(self)
    }
    ///Bit 9 - Clock strobing edge
    #[inline(always)]
    #[must_use]
    pub fn ckstr(&mut self) -> CKSTR_W<9> {
        CKSTR_W::new(self)
    }
    ///Bits 10:11 - Synchronization enable
    #[inline(always)]
    #[must_use]
    pub fn syncen(&mut self) -> SYNCEN_W<10> {
        SYNCEN_W::new(self)
    }
    ///Bit 12 - Mono mode
    #[inline(always)]
    #[must_use]
    pub fn mono(&mut self) -> MONO_W<12> {
        MONO_W::new(self)
    }
    ///Bit 13 - Output drive
    #[inline(always)]
    #[must_use]
    pub fn outdriv(&mut self) -> OUTDRIV_W<13> {
        OUTDRIV_W::new(self)
    }
    ///Bit 16 - Audio block B enable
    #[inline(always)]
    #[must_use]
    pub fn saien(&mut self) -> SAIEN_W<16> {
        SAIEN_W::new(self)
    }
    ///Bit 17 - DMA enable
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<17> {
        DMAEN_W::new(self)
    }
    ///Bit 19 - No divider
    #[inline(always)]
    #[must_use]
    pub fn nodiv(&mut self) -> NODIV_W<19> {
        NODIV_W::new(self)
    }
    ///Bits 20:25 - Master clock divider
    #[inline(always)]
    #[must_use]
    pub fn mckdiv(&mut self) -> MCKDIV_W<20> {
        MCKDIV_W::new(self)
    }
    ///Bit 26 - Oversampling ratio for master clock
    #[inline(always)]
    #[must_use]
    pub fn osr(&mut self) -> OSR_W<26> {
        OSR_W::new(self)
    }
    ///Bit 27 - Master clock generation enable
    #[inline(always)]
    #[must_use]
    pub fn mcken(&mut self) -> MCKEN_W<27> {
        MCKEN_W::new(self)
    }
    ///Writes raw bits to the register.
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
///AConfiguration register 1
///
///This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [cr1](index.html) module
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [cr1::R](R) reader structure
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
///`write(|w| ..)` method takes [cr1::W](W) writer structure
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
///`reset()` method sets CR1 to value 0x40
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x40;
}
