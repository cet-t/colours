use crate::{Hsv, HsvCast, Rgb, RgbCast};

#[test]
fn rgb_works() {
    assert_eq!(Rgb::<u8>::new(0xff, 0x00, 0x00), Rgb::red());
    assert_eq!(Rgb::<u16>::new(0x00, 0xff, 0x00), Rgb::green());
    assert_eq!(Rgb::<u32>::new(0x00, 0x00, 0xff), Rgb::blue());
    assert_eq!(Rgb::<u64>::new(0xff, 0xff, 0xff), Rgb::white());
    assert_eq!(Rgb::<u128>::new(0x00, 0x00, 0x00), Rgb::black());
}

#[test]
fn hsv_works() {
    assert_eq!(Hsv::<f32>::new(0.0, 0.0, 0.0), Hsv::red());
    assert_eq!(Hsv::<f32>::new(1.0, 0.0, 0.0), Hsv::purple());
}

#[test]
fn rgb_to_hsv_works() {
    let rgb_red = Rgb::<u8>::red();
    assert_eq!(rgb_red, Rgb::new(0xff, 0x00, 0x00));

    let hsv_red: Hsv<_> = rgb_red.into();
    assert_eq!(hsv_red, Hsv::new(0x00, 0xff, 0xff));

    let hsv_blue: Hsv<_> = Rgb::<u8>::blue().into();
    assert_eq!(hsv_blue, Hsv::new(170, 0xff, 0xff));

    let hsv_white: Hsv<_> = Rgb::<u8>::white().into();
    assert_eq!(hsv_white, Hsv::new(0x00, 0x00, 0xff));

    let rgb_green = Rgb::<f32>::green();
    assert_eq!(rgb_green, Rgb::new(0.0, 1.0, 0.0));

    let hsv_green: Hsv<_> = rgb_green.into();
    assert_eq!(hsv_green, Hsv::new(1.0 / 3.0, 1.0, 1.0));
}

fn assert_rgb_close(a: Rgb<f32>, b: Rgb<f32>) {
    let [ar, ag, ab] = a.as_slice();
    let [br, bg, bb] = b.as_slice();
    let eps = 1e-6;
    assert!(
        (ar - br).abs() < eps && (ag - bg).abs() < eps && (ab - bb).abs() < eps,
        "left: {a:?}, right: {b:?}"
    );
}

fn assert_hsv_close(a: Hsv<f32>, b: Hsv<f32>) {
    let [ar, ag, ab] = a.as_slice();
    let [br, bg, bb] = b.as_slice();
    let eps = 1e-6;
    assert!(
        (ar - br).abs() < eps && (ag - bg).abs() < eps && (ab - bb).abs() < eps,
        "left: {a:?}, right: {b:?}"
    );
}

#[test]
fn hsv_to_rgb_works() {
    let hsv_red = Hsv::<f32>::new(0.0, 1.0, 1.0);
    let rgb_red: Rgb<_> = hsv_red.into();
    assert_rgb_close(rgb_red, Rgb::red());

    let hsv_green = Hsv::<f32>::new(1.0 / 3.0, 1.0, 1.0);
    let rgb_green: Rgb<_> = hsv_green.into();
    assert_rgb_close(rgb_green, Rgb::green());

    let hsv_blue = Hsv::<f32>::new(2.0 / 3.0, 1.0, 1.0);
    let rgb_blue: Rgb<_> = hsv_blue.into();
    assert_rgb_close(rgb_blue, Rgb::blue());

    let hsv_black = Hsv::<f32>::new(0.0, 0.0, 0.0);
    let rgb_black: Rgb<_> = hsv_black.into();
    assert_rgb_close(rgb_black, Rgb::black());

    let hsv_white = Hsv::<f32>::new(0.0, 0.0, 1.0);
    let rgb_white: Rgb<_> = hsv_white.into();
    assert_rgb_close(rgb_white, Rgb::white());
}

#[test]
fn hsv_to_rgb_u8_works() {
    let hsv_red: Hsv<u8> = Rgb::<u8>::red().into();
    let rgb_red: Rgb<_> = hsv_red.into();
    assert_eq!(rgb_red, Rgb::red());

    let hsv_blue: Hsv<u8> = Rgb::<u8>::blue().into();
    let rgb_blue: Rgb<_> = hsv_blue.into();
    assert_eq!(rgb_blue, Rgb::blue());

    let hsv_white: Hsv<u8> = Rgb::<u8>::white().into();
    let rgb_white: Rgb<_> = hsv_white.into();
    assert_eq!(rgb_white, Rgb::white());
}

#[test]
fn rgb_hsv_round_trip_works() {
    for rgb in [
        Rgb::<f32>::red(),
        Rgb::<f32>::green(),
        Rgb::<f32>::blue(),
        Rgb::<f32>::white(),
        Rgb::<f32>::black(),
    ] {
        let f: Hsv<_> = rgb.into();
        let b: Rgb<_> = f.into();
        assert_rgb_close(rgb, b);
    }
}

#[test]
fn rgb_cast_f32_to_f64_works() {
    let rgb_f32 = Rgb::<f32>::new(0.1, 0.2, 0.3);
    let rgb_f64: Rgb<f64> = rgb_f32.cast();
    let [ar, ag, ab] = rgb_f64.as_slice();
    let eps = 1e-6;
    assert!(
        (ar - 0.1f64).abs() < eps && (ag - 0.2f64).abs() < eps && (ab - 0.3f64).abs() < eps,
        "f64 cast: {rgb_f64:?}"
    );
}

#[test]
fn rgb_cast_f64_to_f32_works() {
    let rgb_f64 = Rgb::<f64>::new(0.1, 0.2, 0.3);
    let rgb_f32: Rgb<f32> = rgb_f64.cast();
    assert_rgb_close(rgb_f32, Rgb::new(0.1, 0.2, 0.3));
}

#[test]
fn rgb_cast_f32_to_u8_works() {
    let rgb_f32 = Rgb::<f32>::new(0.5, 0.25, 0.75);
    let rgb_u8: Rgb<u8> = rgb_f32.cast();
    assert_eq!(rgb_u8, Rgb::new(127, 63, 191));
}

#[test]
fn rgb_cast_f64_to_u8_works() {
    let rgb_f64 = Rgb::<f64>::new(0.5, 0.25, 0.75);
    let rgb_u8: Rgb<u8> = rgb_f64.cast();
    assert_eq!(rgb_u8, Rgb::new(127, 63, 191));
}

#[test]
fn rgb_cast_f32_to_u16_works() {
    let rgb_f32 = Rgb::<f32>::new(0.5, 0.25, 0.75);
    let rgb_u16: Rgb<u16> = rgb_f32.cast();
    assert_eq!(rgb_u16, Rgb::new(127, 63, 191));
}

#[test]
fn rgb_cast_u8_to_f32_works() {
    let rgb_u8 = Rgb::<u8>::new(127, 63, 191);
    let rgb_f32: Rgb<f32> = rgb_u8.cast();
    assert_rgb_close(
        rgb_f32,
        Rgb::new(127.0 / 255.0, 63.0 / 255.0, 191.0 / 255.0),
    );
}

#[test]
fn rgb_cast_u8_to_f64_works() {
    let rgb_u8 = Rgb::<u8>::new(127, 63, 191);
    let rgb_f64: Rgb<f64> = rgb_u8.cast();
    assert_eq!(
        rgb_f64,
        Rgb::new(127.0 / 255.0, 63.0 / 255.0, 191.0 / 255.0),
        "u8 to f64 cast"
    );
}

#[test]
fn rgb_cast_u16_to_f32_works() {
    let rgb_u16 = Rgb::<u16>::new(127, 63, 191);
    let rgb_f32: Rgb<f32> = rgb_u16.cast();
    assert_rgb_close(
        rgb_f32,
        Rgb::new(127.0 / 255.0, 63.0 / 255.0, 191.0 / 255.0),
    );
}

#[test]
fn rgb_cast_u16_to_f64_works() {
    let rgb_u16 = Rgb::<u16>::new(127, 63, 191);
    let rgb_f64: Rgb<f64> = rgb_u16.cast();
    assert_eq!(
        rgb_f64,
        Rgb::new(127.0 / 255.0, 63.0 / 255.0, 191.0 / 255.0),
        "u16 to f64 cast"
    );
}

#[test]
fn rgb_cast_u8_to_u16_works() {
    let rgb_u8 = Rgb::<u8>::new(0x11, 0x22, 0x33);
    let rgb_u16: Rgb<u16> = rgb_u8.cast();
    assert_eq!(rgb_u16, Rgb::new(0x11, 0x22, 0x33));
}

#[test]
fn rgb_cast_u8_to_u32_works() {
    let rgb_u8 = Rgb::<u8>::new(0x11, 0x22, 0x33);
    let rgb_u32: Rgb<u32> = rgb_u8.cast();
    assert_eq!(rgb_u32, Rgb::new(0x11, 0x22, 0x33));
}

#[test]
fn rgb_cast_u16_to_u8_works() {
    let rgb_u16 = Rgb::<u16>::new(0x11, 0x22, 0x33);
    let rgb_u8: Rgb<u8> = rgb_u16.cast();
    assert_eq!(rgb_u8, Rgb::new(0x11, 0x22, 0x33));
}

#[test]
fn rgb_cast_u16_to_u32_works() {
    let rgb_u16 = Rgb::<u16>::new(0x11, 0x22, 0x33);
    let rgb_u32: Rgb<u32> = rgb_u16.cast();
    assert_eq!(rgb_u32, Rgb::new(0x11, 0x22, 0x33));
}

#[test]
fn rgb_cast_u32_to_u16_works() {
    let rgb_u32 = Rgb::<u32>::new(0x11, 0x22, 0x33);
    let rgb_u16: Rgb<u16> = rgb_u32.cast();
    assert_eq!(rgb_u16, Rgb::new(0x11, 0x22, 0x33));
}

#[test]
fn rgb_cast_u64_to_u32_works() {
    let rgb_u64 = Rgb::<u64>::new(0x11, 0x22, 0x33);
    let rgb_u32: Rgb<u32> = rgb_u64.cast();
    assert_eq!(rgb_u32, Rgb::new(0x11, 0x22, 0x33));
}

#[test]
fn rgb_cast_u128_to_u64_works() {
    let rgb_u128 = Rgb::<u128>::new(0x11, 0x22, 0x33);
    let rgb_u64: Rgb<u64> = rgb_u128.cast();
    assert_eq!(rgb_u64, Rgb::new(0x11, 0x22, 0x33));
}

#[test]
fn hsv_cast_f32_to_f64_works() {
    let hsv_f32 = Hsv::<f32>::new(0.1, 0.2, 0.3);
    let hsv_f64: Hsv<f64> = hsv_f32.cast();
    let [ar, ag, ab] = hsv_f64.as_slice();
    let eps = 1e-6;
    assert!(
        (ar - 0.1f64).abs() < eps && (ag - 0.2f64).abs() < eps && (ab - 0.3f64).abs() < eps,
        "hsv f64 cast: {hsv_f64:?}"
    );
}

#[test]
fn hsv_cast_f64_to_f32_works() {
    let hsv_f64 = Hsv::<f64>::new(0.1, 0.2, 0.3);
    let hsv_f32: Hsv<f32> = hsv_f64.cast();
    let [ar, ag, ab] = hsv_f32.as_slice();
    let eps = 1e-6;
    assert!(
        (ar - 0.1f32).abs() < eps && (ag - 0.2f32).abs() < eps && (ab - 0.3f32).abs() < eps,
        "hsv f32 cast: {hsv_f32:?}"
    );
}

#[test]
fn hsv_cast_f32_to_u8_works() {
    let hsv_f32 = Hsv::<f32>::new(0.5, 0.25, 0.75);
    let hsv_u8: Hsv<u8> = hsv_f32.cast();
    assert_eq!(hsv_u8, Hsv::new(127, 63, 191));
}

#[test]
fn hsv_cast_f64_to_u8_works() {
    let hsv_f64 = Hsv::<f64>::new(0.5, 0.25, 0.75);
    let hsv_u8: Hsv<u8> = hsv_f64.cast();
    assert_eq!(hsv_u8, Hsv::new(127, 63, 191));
}

#[test]
fn hsv_cast_f32_to_u16_works() {
    let hsv_f32 = Hsv::<f32>::new(0.5, 0.25, 0.75);
    let hsv_u16: Hsv<u16> = hsv_f32.cast();
    assert_eq!(hsv_u16, Hsv::new(127, 63, 191));
}

#[test]
fn hsv_cast_u8_to_f32_works() {
    let hsv_u8 = Hsv::<u8>::new(127, 63, 191);
    let hsv_f32: Hsv<f32> = hsv_u8.cast();
    assert_hsv_close(
        hsv_f32,
        Hsv::new(127.0 / 255.0, 63.0 / 255.0, 191.0 / 255.0),
    );
}

#[test]
fn hsv_cast_u8_to_f64_works() {
    let hsv_u8 = Hsv::<u8>::new(127, 63, 191);
    let hsv_f64: Hsv<f64> = hsv_u8.cast();
    let [ar, ag, ab] = hsv_f64.as_slice();
    let eps = 1e-9;
    assert!(
        (ar - 127.0 / 255.0).abs() < eps
            && (ag - 63.0 / 255.0).abs() < eps
            && (ab - 191.0 / 255.0).abs() < eps,
        "u8 to f64 cast: {hsv_f64:?}"
    );
}

#[test]
fn hsv_cast_u16_to_f32_works() {
    let hsv_u16 = Hsv::<u16>::new(127, 63, 191);
    let hsv_f32: Hsv<f32> = hsv_u16.cast();
    assert_hsv_close(
        hsv_f32,
        Hsv::new(127.0 / 255.0, 63.0 / 255.0, 191.0 / 255.0),
    );
}

#[test]
fn hsv_cast_u16_to_f64_works() {
    let hsv_u16 = Hsv::<u16>::new(127, 63, 191);
    let hsv_f64: Hsv<f64> = hsv_u16.cast();
    let [ar, ag, ab] = hsv_f64.as_slice();
    let eps = 1e-9;
    assert!(
        (ar - 127.0 / 255.0).abs() < eps
            && (ag - 63.0 / 255.0).abs() < eps
            && (ab - 191.0 / 255.0).abs() < eps,
        "u16 to f64 cast: {hsv_f64:?}"
    );
}

#[test]
fn hsv_cast_u8_to_u16_works() {
    let hsv_u8 = Hsv::<u8>::new(0x11, 0x22, 0x33);
    let hsv_u16: Hsv<u16> = hsv_u8.cast();
    assert_eq!(hsv_u16, Hsv::new(0x11, 0x22, 0x33));
}

#[test]
fn hsv_cast_u8_to_u32_works() {
    let hsv_u8 = Hsv::<u8>::new(0x11, 0x22, 0x33);
    let hsv_u32: Hsv<u32> = hsv_u8.cast();
    assert_eq!(hsv_u32, Hsv::new(0x11, 0x22, 0x33));
}

#[test]
fn hsv_cast_u16_to_u8_works() {
    let hsv_u16 = Hsv::<u16>::new(0x11, 0x22, 0x33);
    let hsv_u8: Hsv<u8> = hsv_u16.cast();
    assert_eq!(hsv_u8, Hsv::new(0x11, 0x22, 0x33));
}

#[test]
fn hsv_cast_u16_to_u32_works() {
    let hsv_u16 = Hsv::<u16>::new(0x11, 0x22, 0x33);
    let hsv_u32: Hsv<u32> = hsv_u16.cast();
    assert_eq!(hsv_u32, Hsv::new(0x11, 0x22, 0x33));
}

#[test]
fn hsv_cast_u32_to_u16_works() {
    let hsv_u32 = Hsv::<u32>::new(0x11, 0x22, 0x33);
    let hsv_u16: Hsv<u16> = hsv_u32.cast();
    assert_eq!(hsv_u16, Hsv::new(0x11, 0x22, 0x33));
}

#[test]
fn hsv_cast_u64_to_u32_works() {
    let hsv_u64 = Hsv::<u64>::new(0x11, 0x22, 0x33);
    let hsv_u32: Hsv<u32> = hsv_u64.cast();
    assert_eq!(hsv_u32, Hsv::new(0x11, 0x22, 0x33));
}

#[test]
fn hsv_cast_u128_to_u64_works() {
    let hsv_u128 = Hsv::<u128>::new(0x11, 0x22, 0x33);
    let hsv_u64: Hsv<u64> = hsv_u128.cast();
    assert_eq!(hsv_u64, Hsv::new(0x11, 0x22, 0x33));
}
