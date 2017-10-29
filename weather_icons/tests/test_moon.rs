extern crate weather_icons;

use weather_icons::moon;

#[test]
fn check_phase_icons() {
    assert_eq!(moon::phase(0f64 / 28f64), '\u{f095}'); // new moon
    assert_eq!(moon::phase(1f64 / 28f64), '\u{f096}');
    assert_eq!(moon::phase(2f64 / 28f64), '\u{f097}');
    assert_eq!(moon::phase(3f64 / 28f64), '\u{f098}');
    assert_eq!(moon::phase(4f64 / 28f64), '\u{f099}');
    assert_eq!(moon::phase(5f64 / 28f64), '\u{f09a}');
    assert_eq!(moon::phase(6f64 / 28f64), '\u{f09b}');
    assert_eq!(moon::phase(7f64 / 28f64), '\u{f09c}'); // first quarter moon
    assert_eq!(moon::phase(8f64 / 28f64), '\u{f09d}');
    assert_eq!(moon::phase(9f64 / 28f64), '\u{f09e}');
    assert_eq!(moon::phase(10f64 / 28f64), '\u{f09f}');
    assert_eq!(moon::phase(11f64 / 28f64), '\u{f0a0}');
    assert_eq!(moon::phase(12f64 / 28f64), '\u{f0a1}');
    assert_eq!(moon::phase(13f64 / 28f64), '\u{f0a2}');
    assert_eq!(moon::phase(14f64 / 28f64), '\u{f0a3}'); // full moon
    assert_eq!(moon::phase(15f64 / 28f64), '\u{f0a4}');
    assert_eq!(moon::phase(16f64 / 28f64), '\u{f0a5}');
    assert_eq!(moon::phase(17f64 / 28f64), '\u{f0a6}');
    assert_eq!(moon::phase(18f64 / 28f64), '\u{f0a7}');
    assert_eq!(moon::phase(19f64 / 28f64), '\u{f0a8}');
    assert_eq!(moon::phase(20f64 / 28f64), '\u{f0a9}');
    assert_eq!(moon::phase(21f64 / 28f64), '\u{f0aa}'); // last quarter moon
    assert_eq!(moon::phase(22f64 / 28f64), '\u{f0ab}');
    assert_eq!(moon::phase(23f64 / 28f64), '\u{f0ac}');
    assert_eq!(moon::phase(24f64 / 28f64), '\u{f0ad}');
    assert_eq!(moon::phase(25f64 / 28f64), '\u{f0ae}');
    assert_eq!(moon::phase(26f64 / 28f64), '\u{f0af}');
    assert_eq!(moon::phase(27f64 / 28f64), '\u{f0b0}');
    assert_eq!(moon::phase(28f64 / 28f64), '\u{f095}'); // Back to new moon
}

#[test]
#[should_panic]
fn lunar_number_less_than_0() {
    moon::phase(-1f64);
}

#[test]
#[should_panic]
fn lunar_number_greater_than_1() {
    moon::phase(2f64);
}
