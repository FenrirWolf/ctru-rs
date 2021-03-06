use std::convert::From;

pub enum Event {
    Psc0,
    Psc1,
    VBlank0,
    VBlank1,
    PPF,
    P3D,
    DMA,
}

#[derive(Copy, Clone)]
pub enum FramebufferFormat {
    Rgba8,
    Bgr8,
    Rgb565,
    Rgb5A1,
    Rgba4,
}

impl FramebufferFormat {
    pub fn pixel_depth_bytes(&self) -> usize {
        use self::FramebufferFormat::*;
        match *self {
            Rgba8 => 4usize,
            Bgr8 => 3usize,
            Rgb565 => 2usize,
            Rgb5A1 => 2usize,
            Rgba4 => 2usize,
        }
    }
}

impl From<::libctru::GSPGPU_FramebufferFormats> for FramebufferFormat {
    #[inline]
    fn from(g: ::libctru::GSPGPU_FramebufferFormats) -> FramebufferFormat {
        use self::FramebufferFormat::*;
        match g {
            ::libctru::GSP_RGBA8_OES => Rgba8,
            ::libctru::GSP_BGR8_OES => Bgr8,
            ::libctru::GSP_RGB565_OES => Rgb565,
            ::libctru::GSP_RGB5_A1_OES => Rgb5A1,
            ::libctru::GSP_RGBA4_OES => Rgba4,
            _ => unreachable!(),
        }
    }
}

impl From<FramebufferFormat> for ::libctru::GSPGPU_FramebufferFormats {
    #[inline]
    fn from(g: FramebufferFormat) -> ::libctru::GSPGPU_FramebufferFormats {
        use self::FramebufferFormat::*;
        match g {
            Rgba8 => ::libctru::GSP_RGBA8_OES,
            Bgr8 => ::libctru::GSP_BGR8_OES,
            Rgb565 => ::libctru::GSP_RGB565_OES,
            Rgb5A1 => ::libctru::GSP_RGB5_A1_OES,
            Rgba4 => ::libctru::GSP_RGBA4_OES,
        }
    }
}

fn to_raw_event(ev: Event) -> ::libctru::GSPGPU_Event {
    use self::Event::*;

    match ev {
        Psc0 => ::libctru::GSPGPU_EVENT_PSC0,
        Psc1 => ::libctru::GSPGPU_EVENT_PSC1,
        VBlank0 => ::libctru::GSPGPU_EVENT_VBlank0,
        VBlank1 => ::libctru::GSPGPU_EVENT_VBlank1,
        PPF => ::libctru::GSPGPU_EVENT_PPF,
        P3D => ::libctru::GSPGPU_EVENT_P3D,
        DMA => ::libctru::GSPGPU_EVENT_DMA,
    }
}

/// Sleep until GSP event fires.
///
/// # Examples
///
/// Wait for VBlank.
///
/// ```
/// use ctru::services::apt;
/// apt::main_loop(|| {
///     wait_for_event(Event::VBlank0);
/// });
pub fn wait_for_event(ev: Event) -> () {
    unsafe {
        // TODO second argument?
        ::libctru::gspWaitForEvent(to_raw_event(ev), false);
    }
}
