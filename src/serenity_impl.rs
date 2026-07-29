use serenity::model::Colour;

use crate::{ColourValue, Rgb, RgbCast};

impl<T: ColourValue> Into<Colour> for Rgb<T>
where
    Rgb<T>: RgbCast<u8>,
{
    fn into(self) -> Colour {
        let o: Rgb<u8> = self.cast();
        Colour::from_rgb(o.r, o.g, o.b)
    }
}
