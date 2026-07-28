use crate::ColourValue;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rgb<T: ColourValue>(pub(crate) T, pub(crate) T, pub(crate) T);

impl<T: ColourValue<Work = T>> Rgb<T> {
    pub fn new(r: T, g: T, b: T) -> Self {
        Self(r.clamp(), g.clamp(), b.clamp())
    }

    pub const fn r(self) -> T {
        self.0
    }

    pub const fn g(self) -> T {
        self.1
    }

    pub const fn b(self) -> T {
        self.2
    }

    pub const fn with_red(self, r: T) -> Self {
        Self(r, self.1, self.2)
    }

    pub const fn with_green(self, g: T) -> Self {
        Self(self.0, g, self.2)
    }

    pub const fn with_blue(self, b: T) -> Self {
        Self(self.0, self.1, b)
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
        Self(T::MAX, T::MIN, T::MIN)
    }

    pub const fn green() -> Self {
        Self(T::MIN, T::MAX, T::MIN)
    }

    pub const fn blue() -> Self {
        Self(T::MIN, T::MIN, T::MAX)
    }

    pub const fn black() -> Self {
        Self(T::MIN, T::MIN, T::MIN)
    }

    pub const fn white() -> Self {
        Self(T::MAX, T::MAX, T::MAX)
    }
}

crate::impl_rgb_methods!(u8, u16, u32, u64, u128);
crate::impl_rgb_methods!(f32, f64);

crate::impl_rgb_neg!(u; 8, 16, 32, 64, 128);
crate::impl_rgb_neg!(f; 32, 64);
