use crate::{ColourValue, Hsv, Rgb};

impl<T: ColourValue> serde::Serialize for Rgb<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Rgb", 3)?;
        state.serialize_field("r", &self.r)?;
        state.serialize_field("g", &self.g)?;
        state.serialize_field("b", &self.b)?;
        state.end()
    }
}

impl<'de, T: ColourValue> serde::Deserialize<'de> for Rgb<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(rename = "Rgb")]
        struct RgbHelper<T> {
            r: T,
            g: T,
            b: T,
        }
        let helper = RgbHelper::<T>::deserialize(deserializer)?;
        Ok(Rgb {
            r: helper.r,
            g: helper.g,
            b: helper.b,
        })
    }
}

impl<T: ColourValue> serde::Serialize for Hsv<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("Hsv", 3)?;
        state.serialize_field("h", &self.h)?;
        state.serialize_field("s", &self.s)?;
        state.serialize_field("v", &self.v)?;
        state.end()
    }
}

impl<'de, T: ColourValue> serde::Deserialize<'de> for Hsv<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        #[serde(rename = "Hsv")]
        struct HsvHelper<T> {
            h: T,
            s: T,
            v: T,
        }
        let helper = HsvHelper::<T>::deserialize(deserializer)?;
        Ok(Hsv {
            h: helper.h,
            s: helper.s,
            v: helper.v,
        })
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn rgb_serde_works() {
        let rgb = crate::Rgb::<u8>::new(0xff, 0, 0);
        let se = serde_json::to_string(&rgb).unwrap();
        assert_eq!(se, "{\"r\":255,\"g\":0,\"b\":0}".to_owned());

        let de: crate::Rgb<u8> = serde_json::from_str(&se).unwrap();
        assert_eq!(de, crate::Rgb::<u8>::new(255, 0, 0));
    }

    #[test]
    fn hsv_serde_works() {
        let hsv = crate::Hsv::<f32>::new(1., 0., 0.);
        let se = serde_json::to_string(&hsv).unwrap();
        assert_eq!(se, "{\"h\":1.0,\"s\":0.0,\"v\":0.0}".to_owned());

        let de: crate::Hsv<f32> = serde_json::from_str(&se).unwrap();
        assert_eq!(de, crate::Hsv::<f32>::new(1., 0., 0.));
    }
}
