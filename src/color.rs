use anyhow::anyhow;
use crossterm::style::Color as CTColor;
use hex_color::HexColor;

#[derive(Debug, Clone)]
pub struct Color(pub CTColor);

impl std::str::FromStr for Color {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(color) = CTColor::try_from(s) {
            return Ok(Self(color));
        }

        if s.starts_with('#') {
            match HexColor::parse_rgb(s) {
                Ok(color) => return Ok(Self(CTColor::from((color.r, color.g, color.b)))),
                Err(e) => return Err(anyhow!("Invalid color '{}': {}", s, e)),
            }
        }

        Err(anyhow!("Invalid color: '{}'", s))
    }
}
