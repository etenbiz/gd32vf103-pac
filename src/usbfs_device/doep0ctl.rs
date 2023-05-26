#[doc = "Register `DOEP0CTL` reader"]
pub struct R(crate::R<DOEP0CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP0CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP0CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP0CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP0CTL` writer"]
pub struct W(crate::W<DOEP0CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP0CTL_SPEC>;
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
impl From<crate::W<DOEP0CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP0CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MPL` reader - Maximum packet length"]
pub type MPL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EPACT` reader - Endpoint active"]
pub type EPACT_R = crate::BitReader<bool>;
#[doc = "Field `NAKS` reader - NAK status"]
pub type NAKS_R = crate::BitReader<bool>;
#[doc = "Field `EPTYPE` reader - Endpoint type"]
pub type EPTYPE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SNOOP` reader - Snoop mode"]
pub type SNOOP_R = crate::BitReader<bool>;
#[doc = "Field `SNOOP` writer - Snoop mode"]
pub type SNOOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0CTL_SPEC, bool, O>;
#[doc = "Field `STALL` reader - STALL handshake"]
pub type STALL_R = crate::BitReader<bool>;
#[doc = "Field `STALL` writer - STALL handshake"]
pub type STALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0CTL_SPEC, bool, O>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0CTL_SPEC, bool, O>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SNAK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0CTL_SPEC, bool, O>;
#[doc = "Field `EPD` reader - Endpoint disable"]
pub type EPD_R = crate::BitReader<bool>;
#[doc = "Field `EPEN` reader - Endpoint enable"]
pub type EPEN_R = crate::BitReader<bool>;
#[doc = "Field `EPEN` writer - Endpoint enable"]
pub type EPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEP0CTL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Maximum packet length"]
    #[inline(always)]
    pub fn mpl(&self) -> MPL_R {
        MPL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - Endpoint active"]
    #[inline(always)]
    pub fn epact(&self) -> EPACT_R {
        EPACT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK status"]
    #[inline(always)]
    pub fn naks(&self) -> NAKS_R {
        NAKS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptype(&self) -> EPTYPE_R {
        EPTYPE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snoop(&self) -> SNOOP_R {
        SNOOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epd(&self) -> EPD_R {
        EPD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epen(&self) -> EPEN_R {
        EPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    #[must_use]
    pub fn snoop(&mut self) -> SNOOP_W<20> {
        SNOOP_W::new(self)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<21> {
        STALL_W::new(self)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<26> {
        CNAK_W::new(self)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<27> {
        SNAK_W::new(self)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    #[must_use]
    pub fn epen(&mut self) -> EPEN_W<31> {
        EPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "device endpoint-0 control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0ctl](index.html) module"]
pub struct DOEP0CTL_SPEC;
impl crate::RegisterSpec for DOEP0CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep0ctl::R](R) reader structure"]
impl crate::Readable for DOEP0CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep0ctl::W](W) writer structure"]
impl crate::Writable for DOEP0CTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEP0CTL to value 0x8000"]
impl crate::Resettable for DOEP0CTL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000;
}
