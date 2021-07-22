use std::{
    fmt::{Debug, Display, Formatter},
    num::ParseIntError,
    str::FromStr,
};

/// Responsive layout system for mainstream screen
///
/// ## epx
/// Note that mobile devices does not depend on the actual pixel, but on the equivalent pixel.
///
/// For example `Phone 11Pro Max` has a `1242px × 2688px @ 3x` screen, the actual size is `414epx × 896epx`.
///
/// For 2K screen, which has a `2560px × 1440px @ 1.5x` screen, the actual size is `1706epx × 960epx`.
///
/// For 4K screen, which has a `4096px × 2160px @ 2x` screen, the actual size is `2048epx × 1024epx`.
///
/// ## Example
///
/// see: [`crate::use_window_layout`]
#[derive(Debug, Copy, Clone)]
pub enum ResponsiveLayout {
    /// `width ⩽ 375epx`
    ///
    /// ## Devices
    /// - ios: `3GS`, `4/4S`, `Phone X,Xs,11Pro`
    Tiny,
    /// `width ⩽ 640epx`
    /// ## Devices
    /// - ios: `Phone 6p,6sp,7p,8p`, `Phone 11,Xr,Phone 11Pro Max,Xs Max`
    Small,
    /// `width ⩽ 992epx`
    /// ## Devices
    /// - Vertical ipad
    Medium,
    /// `width ⩽ 1366epx`
    /// ## Devices
    /// - Horizontal ipad
    Large,
    /// `width ⩽ 2048epx`
    /// ## Devices
    /// - Mainstream Desktop Screen(16:10,16:9)
    ExtraLarge,
    /// `width > 2048epx`
    /// ## Devices
    /// - Hairtail Screen(21:9)
    UltraLarge,
}

impl Default for ResponsiveLayout {
    fn default() -> Self {
        Self::Large
    }
}

impl Display for ResponsiveLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tiny => f.write_str("xs"),
            Self::Small => f.write_str("sm"),
            Self::Medium => f.write_str("md"),
            Self::Large => f.write_str("lg"),
            Self::ExtraLarge => f.write_str("xl"),
            Self::UltraLarge => f.write_str("ul"),
        }
    }
}

impl From<usize> for ResponsiveLayout {
    fn from(n: usize) -> Self {
        match n {
            n if n <= 375 => Self::Tiny,
            n if n <= 640 => Self::Small,
            n if n <= 992 => Self::Medium,
            n if n <= 1366 => Self::Large,
            n if n <= 2048 => Self::ExtraLarge,
            _ => Self::UltraLarge,
        }
    }
}

impl FromStr for ResponsiveLayout {
    type Err = ParseIntError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s.to_ascii_lowercase().as_str() {
            "t" | "xs" | "tiny" => Self::Tiny,
            "s" | "sm" | "small" => Self::Small,
            "m" | "md" | "medium" => Self::Medium,
            "l" | "lg" | "large" => Self::Large,
            "x" | "xl" | "extra" => Self::ExtraLarge,
            "u" | "ul" | "xxl" | "2xl" | "ultra" => Self::UltraLarge,
            s => Self::from(s.parse::<usize>()?),
        };
        Ok(out)
    }
}
