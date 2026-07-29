use crate::{
    ColourValue, Hsv, Rgb, impl_rgb_cast, impl_rgb_cast_f_to_u, impl_rgb_cast_identity,
    impl_rgb_cast_u_to_f,
};

pub trait RgbCast<T: ColourValue> {
    fn cast(self) -> Rgb<T>;
}

pub trait HsvCast<T: ColourValue> {
    fn cast(self) -> Hsv<T>;
}

impl_rgb_cast_identity!(f32);
impl_rgb_cast_identity!(f64);
impl_rgb_cast_identity!(u8);
impl_rgb_cast_identity!(u16);
impl_rgb_cast_identity!(u32);
impl_rgb_cast_identity!(u64);
impl_rgb_cast_identity!(u128);

impl_rgb_cast!(f32, f64);
impl_rgb_cast_f_to_u!(f32, u8);
impl_rgb_cast_f_to_u!(f32, u16);
impl_rgb_cast_f_to_u!(f32, u32);
impl_rgb_cast_f_to_u!(f32, u64);
impl_rgb_cast_f_to_u!(f32, u128);

impl_rgb_cast!(f64, f32);
impl_rgb_cast_f_to_u!(f64, u8);
impl_rgb_cast_f_to_u!(f64, u16);
impl_rgb_cast_f_to_u!(f64, u32);
impl_rgb_cast_f_to_u!(f64, u64);
impl_rgb_cast_f_to_u!(f64, u128);

impl_rgb_cast!(u8, u16);
impl_rgb_cast!(u8, u32);
impl_rgb_cast!(u8, u64);
impl_rgb_cast!(u8, u128);
impl_rgb_cast_u_to_f!(u8, f32);
impl_rgb_cast_u_to_f!(u8, f64);

impl_rgb_cast!(u16, u8);
impl_rgb_cast!(u16, u32);
impl_rgb_cast!(u16, u64);
impl_rgb_cast!(u16, u128);
impl_rgb_cast_u_to_f!(u16, f32);
impl_rgb_cast_u_to_f!(u16, f64);

impl_rgb_cast!(u32, u8);
impl_rgb_cast!(u32, u16);
impl_rgb_cast!(u32, u64);
impl_rgb_cast!(u32, u128);
impl_rgb_cast_u_to_f!(u32, f32);
impl_rgb_cast_u_to_f!(u32, f64);

impl_rgb_cast!(u64, u8);
impl_rgb_cast!(u64, u16);
impl_rgb_cast!(u64, u32);
impl_rgb_cast!(u64, u128);
impl_rgb_cast_u_to_f!(u64, f32);
impl_rgb_cast_u_to_f!(u64, f64);

impl_rgb_cast!(u128, u8);
impl_rgb_cast!(u128, u16);
impl_rgb_cast!(u128, u32);
impl_rgb_cast!(u128, u64);
impl_rgb_cast_u_to_f!(u128, f32);
impl_rgb_cast_u_to_f!(u128, f64);
