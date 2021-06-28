use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Copy, Clone)]
pub enum LayoutSystem {
    Tiny,
    Small,
    Medium,
    Large,
    LargeX,
    LargeXX,
}

impl Default for LayoutSystem {
    fn default() -> Self {
        Self::Large
    }
}

impl Display for LayoutSystem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tiny => f.write_str("xs"),
            Self::Small => f.write_str("sm"),
            Self::Medium => f.write_str("md"),
            Self::Large => f.write_str("lg"),
            Self::LargeX => f.write_str("xl"),
            Self::LargeXX => f.write_str("2xl"),
        }
    }
}

impl From<usize> for LayoutSystem {
    fn from(n: usize) -> Self {
        match n {
            n if n >= 1536 => Self::LargeXX,
            n if n >= 1280 => Self::LargeX,
            n if n >= 1024 => Self::Large,
            n if n >= 768 => Self::Medium,
            n if n >= 640 => Self::Small,
            _ => Self::Tiny,
        }
    }
}
