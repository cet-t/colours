use crate::ColourValue;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Hsv<T: ColourValue> {
    pub h: T,
    pub s: T,
    pub v: T,
}

impl<T: ColourValue<Work = T>> Hsv<T> {
    pub fn new(h: T, s: T, v: T) -> Self {
        Self {
            h: h.clamp(),
            s: s.clamp(),
            v: v.clamp(),
        }
    }

    pub const fn with_hue(self, h: T) -> Self {
        Self {
            h,
            s: self.s,
            v: self.v,
        }
    }

    pub const fn with_saturation(self, s: T) -> Self {
        Self {
            h: self.h,
            s,
            v: self.v,
        }
    }

    pub const fn with_value(self, v: T) -> Self {
        Self {
            h: self.h,
            s: self.s,
            v,
        }
    }

    pub const fn as_slice(self) -> [T; 3] {
        [self.h, self.s, self.v]
    }

    pub const fn from_slice(src: [T; 3]) -> Self {
        Self {
            h: src[0],
            s: src[1],
            v: src[2],
        }
    }

    pub const fn as_tuple(self) -> (T, T, T) {
        (self.h, self.s, self.v)
    }

    pub const fn from_tuple(src: (T, T, T)) -> Self {
        Self {
            h: src.0,
            s: src.1,
            v: src.2,
        }
    }

    pub fn map<F: FnOnce(T, T, T) -> [T; 3]>(self, f: F) -> Self {
        Self::from_slice(f(self.h, self.s, self.v))
    }

    pub const fn red() -> Self {
        Self {
            h: T::MIN,
            s: T::MIN,
            v: T::MIN,
        }
    }

    pub const fn purple() -> Self {
        Self {
            h: T::MAX,
            s: T::MIN,
            v: T::MIN,
        }
    }
}
