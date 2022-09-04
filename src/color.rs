#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Color {
    Rgb { r: u8, g: u8, b: u8 },
    Default,
}

impl Color {
    pub const fn rgb(color: [u8; 3]) -> Self {
        Self::Rgb {
            r: color[0],
            g: color[1],
            b: color[2],
        }
    }

    pub const fn white() -> Self {
        Self::Rgb {
            r: 255,
            g: 255,
            b: 255,
        }
    }

    pub const fn black() -> Self {
        Self::Rgb { r: 0, g: 0, b: 0 }
    }

    pub const fn red() -> Self {
        Self::Rgb { r: 255, g: 0, b: 0 }
    }

    pub const fn green() -> Self {
        Self::Rgb { r: 0, g: 255, b: 0 }
    }

    pub const fn blue() -> Self {
        Self::Rgb { r: 0, g: 0, b: 255 }
    }

    pub fn terminal_format(&self) -> String {
        match self {
            Color::Rgb { r, g, b } => format!("\x1b[38;2;{};{};{}m", r, g, b),
            Color::Default => String::from("\x1b[0m"),
        }
    }
}
