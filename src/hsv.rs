use crate::ColourValue;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Hsv<T: ColourValue>(pub(crate) T, pub(crate) T, pub(crate) T);

impl<T: ColourValue<Work = T>> Hsv<T> {
    pub fn new(h: T, s: T, v: T) -> Self {
        Self(h.clamp(), s.clamp(), v.clamp())
    }

    pub const fn h(self) -> T {
        self.0
    }

    pub const fn s(self) -> T {
        self.1
    }

    pub const fn v(self) -> T {
        self.2
    }

    pub const fn with_hue(self, h: T) -> Self {
        Self(h, self.1, self.2)
    }

    pub const fn with_saturation(self, s: T) -> Self {
        Self(self.0, s, self.2)
    }

    pub const fn with_value(self, v: T) -> Self {
        Self(self.0, self.1, v)
    }

    pub const fn as_slice(self) -> [T; 3] {
        [self.0, self.1, self.2]
    }

    pub const fn from_slice(src: [T; 3]) -> Self {
        Self(src[0], src[1], src[2])
    }

    pub const fn as_tuple(self) -> (T, T, T) {
        (self.0, self.1, self.2)
    }

    pub const fn from_tuple(src: (T, T, T)) -> Self {
        Self(src.0, src.1, src.2)
    }

    pub fn map<F: FnOnce(T, T, T) -> [T; 3]>(self, f: F) -> Self {
        Self::from_slice(f(self.0, self.1, self.2))
    }

    pub const fn red() -> Self {
        Self(T::MIN, T::MIN, T::MIN)
    }

    pub const fn purple() -> Self {
        Self(T::MAX, T::MIN, T::MIN)
    }
}
