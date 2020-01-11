use super::*;
use crate::color::Color as TrovoColor;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub(crate) color: TrovoColor,
}

impl Encodable for Color {
    fn encode<W: ?Sized + Write>(&self, writer: &mut W) -> std::io::Result<()> {
        command(&format!("/color {}", self.color)).encode(writer)
    }
}

pub fn color(color: TrovoColor) -> Color {
    Color { color }
}
