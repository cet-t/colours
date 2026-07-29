#![allow(unused)]

macro_rules! impl_colour_value_trait {
    (f; $($bits:expr),+ $(,)?) => {
        $(::pastey::paste! {
            impl ColourValue for [<f $bits>] {
                type Work = [<f $bits>];
                const MIN: Self::Work = 0 as Self::Work;
                const MAX: Self::Work = 1 as Self::Work;
                const QUATER: Self::Work = Self::MAX / (4 as [<f $bits>]);

                fn clamp(self) -> Self::Work {
                    self.clamp(Self::MIN, Self::MAX)
                }

                fn add(self, v: Self::Work) -> Self::Work {
                    self + v
                }

                fn sub(self, v: Self::Work) -> Self::Work {
                    self - v
                }

                fn mul(self, v: Self::Work) -> Self::Work {
                    self / v
                }

                fn div(self, v: Self::Work) -> Self::Work {
                    self / v
                }
            }
        })+
    };
    (u; $($bits:expr),+ $(,)?) => {
        $(::pastey::paste! {
            impl ColourValue for [<u $bits>] {
                type Work = [<u $bits>];
                const MIN: Self::Work = 0x00 as Self::Work;
                const MAX: Self::Work = 0xff as Self::Work;
                const QUATER: Self::Work = Self::MAX / (4 as [<u $bits>]);

                fn clamp(self) -> Self::Work {
                    if self < Self::MIN {
                        Self::MIN
                    } else if self > Self::MAX {
                        Self::MAX
                    } else {
                        self
                    }
                }

                fn add(self, v: Self::Work) -> Self::Work {
                    self + v
                }

                fn sub(self, v: Self::Work) -> Self::Work {
                    self - v
                }

                fn mul(self, v: Self::Work) -> Self::Work {
                    self / v
                }

                fn div(self, v: Self::Work) -> Self::Work {
                    self / v
                }
            }
        })+
    };
}
pub(crate) use impl_colour_value_trait;

macro_rules! impl_rgb_methods {
    ($($t:ty),+ $(,)?) => {
        $(
            impl std::ops::Add for Rgb<$t> {
                type Output = Self;

                fn add(self, rhs: Self) -> Self::Output {
                    Self {
                        r: self.r + rhs.r,
                        g: self.g + rhs.g,
                        b: self.b + rhs.b,
                    }
                }
            }

            impl std::ops::Sub for Rgb<$t> {
                type Output = Self;

                fn sub(self, rhs: Self) -> Self::Output {
                    Self {
                        r: self.r - rhs.r,
                        g: self.g - rhs.g,
                        b: self.b - rhs.b,
                    }
                }
            }

            impl std::ops::Mul for Rgb<$t> {
                type Output = Self;

                fn mul(self, rhs: Self) -> Self::Output {
                    Self {
                        r: self.r * rhs.r,
                        g: self.g * rhs.g,
                        b: self.b * rhs.b,
                    }
                }
            }
        )+
    };
}
pub(crate) use impl_rgb_methods;

macro_rules! impl_rgb_neg {
    (u; $($bits:expr),+ $(,)?) => {
        $(::pastey::paste! {
            impl std::ops::Neg for Rgb<[<u $bits>]> {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self::black() - self
                }
            }
        })+
    };
    (f; $($bits:expr),+ $(,)?) => {
        $(::pastey::paste! {
            impl std::ops::Neg for Rgb<[<f $bits>]> {
                type Output = Self;

                fn neg(self) -> Self::Output {
                    Self::black() - self
                }
            }
        })+
    };
}
pub(crate) use impl_rgb_neg;

macro_rules! impl_colour_trait {
    ($($t:ty),+ $(,)?) => {
        $(::pastey::paste! {
            impl<T: ColourValue> Colour for $t<T> {
                type Word = T;
            }
        })+
    };
}
pub(crate) use impl_colour_trait;

macro_rules! impl_rgb_to_hsv {
    (u; $($bits:expr),+ $(,)?) => {
        $(::pastey::paste! {
            impl Into<Hsv<[<u $bits>]>> for Rgb<[<u $bits>]> {
                fn into(self) -> Hsv<[<u $bits>]> {
                    const MAX_VAL: f64 = [<u $bits>]::MAX as f64;
                    const UNIT: f64 = MAX_VAL / 6.0;

                    let r = self.r as f64;
                    let g = self.g as f64;
                    let b = self.b as f64;

                    let c_max = r.max(g.max(b));
                    let c_min = r.min(g.min(b));
                    let delta = c_max - c_min;

                    let h = if delta == 0.0 {
                        0.0
                    } else if c_max == r {
                        UNIT * (((g - b) / delta).rem_euclid(6.0))
                    } else if c_max == g {
                        UNIT * (((b - r) / delta) + 2.0)
                    } else {
                        UNIT * (((r - g) / delta) + 4.0)
                    };
                    let s = if c_max == 0.0 { 0.0 } else { delta / c_max };
                    let v = c_max;

                    Hsv::new(h as [<u $bits>], (s * MAX_VAL) as [<u $bits>], v as [<u $bits>])
                }
            }
        })+
    };
    (f; $($bits:expr),+ $(,)?) => {
        $(::pastey::paste! {
            impl Into<Hsv<[<f $bits>]>> for Rgb<[<f $bits>]> {
                fn into(self) -> Hsv<[<f $bits>]> {
                    const UNIT: f64 = 1.0 / 6.0;

                    let r = self.r as f64;
                    let g = self.g as f64;
                    let b = self.b as f64;

                    let c_max = r.max(g.max(b));
                    let c_min = r.min(g.min(b));
                    let delta = c_max - c_min;

                    let h = if delta == 0.0 {
                        0.0
                    } else if c_max == r {
                        UNIT * (((g - b) / delta).rem_euclid(6.0))
                    } else if c_max == g {
                        UNIT * (((b - r) / delta) + 2.0)
                    } else {
                        UNIT * (((r - g) / delta) + 4.0)
                    };
                    let s = if c_max == 0.0 { 0.0 } else { delta / c_max };
                    let v = c_max;

                    Hsv::new(h as [<f $bits>], s as [<f $bits>], v as [<f $bits>])
                }
            }
        })+
    };
}
pub(crate) use impl_rgb_to_hsv;

macro_rules! impl_hsv_to_rgb {
    (u; $($bits:expr),+ $(,)?) => {
        $(::pastey::paste! {
            impl Into<Rgb<[<u $bits>]>> for Hsv<[<u $bits>]> {
                fn into(self) -> Rgb<[<u $bits>]> {
                    const MAX_VAL: f64 = [<u $bits>]::MAX as f64;

                    let h = self.h as f64 / MAX_VAL;
                    let s = self.s as f64 / MAX_VAL;
                    let v = self.v as f64 / MAX_VAL;

                    let h6 = h * 6.0;
                    let c = v * s;
                    let x = c * (1.0 - ((h6 % 2.0) - 1.0).abs());
                    let m = v - c;

                    let (r, g, b) = match h6 as u32 {
                        0 => (c, x, 0.0),
                        1 => (x, c, 0.0),
                        2 => (0.0, c, x),
                        3 => (0.0, x, c),
                        4 => (x, 0.0, c),
                        _ => (c, 0.0, x),
                    };

                    Rgb::new(
                        ((r + m) * MAX_VAL).round() as [<u $bits>],
                        ((g + m) * MAX_VAL).round() as [<u $bits>],
                        ((b + m) * MAX_VAL).round() as [<u $bits>],
                    )
                }
            }
        })+
    };
    (f; $($bits:expr),+ $(,)?) => {
        $(::pastey::paste! {
            impl Into<Rgb<[<f $bits>]>> for Hsv<[<f $bits>]> {
                fn into(self) -> Rgb<[<f $bits>]> {
                    let h = self.h as f64;
                    let s = self.s as f64;
                    let v = self.v as f64;

                    let h6 = h * 6.0;
                    let c = v * s;
                    let x = c * (1.0 - ((h6 % 2.0) - 1.0).abs());
                    let m = v - c;

                    let (r, g, b) = match h6 as u32 {
                        0 => (c, x, 0.0),
                        1 => (x, c, 0.0),
                        2 => (0.0, c, x),
                        3 => (0.0, x, c),
                        4 => (x, 0.0, c),
                        _ => (c, 0.0, x),
                    };

                    Rgb::new((r + m) as [<f $bits>], (g + m) as [<f $bits>], (b + m) as [<f $bits>])
                }
            }
        })+
    };
}
pub(crate) use impl_hsv_to_rgb;

macro_rules! impl_rgb_cast {
    ($from:ty, $to:ty) => {
        impl RgbCast<$to> for Rgb<$from> {
            fn cast(self) -> Rgb<$to> {
                Rgb::new(self.r as $to, self.g as $to, self.b as $to)
            }
        }

        impl HsvCast<$to> for Hsv<$from> {
            fn cast(self) -> Hsv<$to> {
                Hsv::new(self.h as $to, self.s as $to, self.v as $to)
            }
        }
    };
}
pub(crate) use impl_rgb_cast;

macro_rules! impl_rgb_cast_f_to_u {
    ($f:ty, $u:ty) => {
        impl RgbCast<$u> for Rgb<$f> {
            fn cast(self) -> Rgb<$u> {
                let to_u = |x| (x * 255.0) as $u;
                Rgb::new(to_u(self.r), to_u(self.g), to_u(self.b))
            }
        }

        impl HsvCast<$u> for Hsv<$f> {
            fn cast(self) -> Hsv<$u> {
                let to_u = |x| (x * 255.0) as $u;
                Hsv::new(to_u(self.h), to_u(self.s), to_u(self.v))
            }
        }
    };
}
pub(crate) use impl_rgb_cast_f_to_u;

macro_rules! impl_rgb_cast_u_to_f {
    ($u:ty, $f:ty) => {
        impl RgbCast<$f> for Rgb<$u> {
            fn cast(self) -> Rgb<$f> {
                let to_f = |x| (x as $f) / 255.0;
                Rgb::new(to_f(self.r), to_f(self.g), to_f(self.b))
            }
        }

        impl HsvCast<$f> for Hsv<$u> {
            fn cast(self) -> Hsv<$f> {
                let to_f = |x| (x as $f) / 255.0;
                Hsv::new(to_f(self.h), to_f(self.s), to_f(self.v))
            }
        }
    };
}
pub(crate) use impl_rgb_cast_u_to_f;
