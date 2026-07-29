use crate::{Hsv, Rgb, impl_colour_value_trait, impl_hsv_to_rgb, impl_rgb_to_hsv};

#[cfg(not(feature = "serde"))]
pub trait ColourValue: Sized + Copy + Default {
    type Work: Sized + Copy + Default;
    const MIN: Self::Work;
    const MAX: Self::Work;
    const QUATER: Self::Work;

    fn clamp(self) -> Self::Work;

    fn add(self, v: Self::Work) -> Self::Work;
    fn sub(self, v: Self::Work) -> Self::Work;
    fn mul(self, v: Self::Work) -> Self::Work;
    fn div(self, v: Self::Work) -> Self::Work;
}

#[cfg(feature = "serde")]
pub trait ColourValue:
    Sized + Copy + Default + serde::Serialize + for<'de> serde::Deserialize<'de>
{
    type Work: Sized + Copy + Default;
    const MIN: Self::Work;
    const MAX: Self::Work;
    const QUATER: Self::Work;

    fn clamp(self) -> Self::Work;

    fn add(self, v: Self::Work) -> Self::Work;
    fn sub(self, v: Self::Work) -> Self::Work;
    fn mul(self, v: Self::Work) -> Self::Work;
    fn div(self, v: Self::Work) -> Self::Work;
}

impl_colour_value_trait!(f; 32, 64);
impl_colour_value_trait!(u; 8, 16, 32, 64, 128);

impl_rgb_to_hsv!(u; 8, 16, 32, 64, 128);
impl_rgb_to_hsv!(f; 32, 64);

impl_hsv_to_rgb!(u; 8, 16, 32, 64, 128);
impl_hsv_to_rgb!(f; 32, 64);
