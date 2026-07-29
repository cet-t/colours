use crate::ColourValue;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rgb<T: ColourValue> {
    pub(crate) r: T,
    pub(crate) g: T,
    pub(crate) b: T,
}

impl<T: ColourValue<Work = T>> Rgb<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self {
            r: r.clamp(),
            g: g.clamp(),
            b: b.clamp(),
        }
    }

    pub const fn with_red(self, r: T) -> Self {
        Self {
            r,
            g: self.g,
            b: self.b,
        }
    }

    pub const fn with_green(self, g: T) -> Self {
        Self {
            r: self.r,
            g,
            b: self.b,
        }
    }

    pub const fn with_blue(self, b: T) -> Self {
        Self {
            r: self.r,
            g: self.g,
            b,
        }
    }

    pub const fn as_slice(self) -> [T; 3] {
        [self.r, self.g, self.b]
    }

    pub const fn from_slice(src: [T; 3]) -> Self {
        Self {
            r: src[0],
            g: src[1],
            b: src[2],
        }
    }

    pub const fn as_tuple(self) -> (T, T, T) {
        (self.r, self.g, self.b)
    }

    pub const fn from_tuple(src: (T, T, T)) -> Self {
        Self {
            r: src.0,
            g: src.1,
            b: src.2,
        }
    }

    pub fn map<F: FnOnce(T, T, T) -> [T; 3]>(self, f: F) -> Self {
        Self::from_slice(f(self.r, self.g, self.b))
    }

    pub const fn red() -> Self {
        Self {
            r: T::MAX,
            g: T::MIN,
            b: T::MIN,
        }
    }

    pub const fn green() -> Self {
        Self {
            r: T::MIN,
            g: T::MAX,
            b: T::MIN,
        }
    }

    pub const fn blue() -> Self {
        Self {
            r: T::MIN,
            g: T::MIN,
            b: T::MAX,
        }
    }

    pub const fn black() -> Self {
        Self {
            r: T::MIN,
            g: T::MIN,
            b: T::MIN,
        }
    }

    pub const fn white() -> Self {
        Self {
            r: T::MAX,
            g: T::MAX,
            b: T::MAX,
        }
    }
}

crate::impl_rgb_methods!(u8, u16, u32, u64, u128);
crate::impl_rgb_methods!(f32, f64);

crate::impl_rgb_neg!(u; 8, 16, 32, 64, 128);
crate::impl_rgb_neg!(f; 32, 64);
