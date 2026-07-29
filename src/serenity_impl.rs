use serenity::model::Colour;

use crate::{Rgb, RgbCast};

impl Into<Colour> for Rgb<u8> {
    fn into(self) -> Colour {
        Colour::from_rgb(self.r, self.g, self.b)
    }
}

impl Into<Colour> for Rgb<u32> {
    fn into(self) -> Colour {
        let o: Rgb<u8> = self.cast();
        o.into()
    }
}
