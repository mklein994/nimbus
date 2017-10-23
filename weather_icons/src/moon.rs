use std::fmt;

pub enum Moon {
    MoonNew,
    MoonWaxingCrescent1,
    MoonWaxingCrescent2,
    MoonWaxingCrescent3,
    MoonWaxingCrescent4,
    MoonWaxingCrescent5,
    MoonWaxingCrescent6,
    MoonFirstQuarter,
    MoonWaxingGibbous1,
    MoonWaxingGibbous2,
    MoonWaxingGibbous3,
    MoonWaxingGibbous4,
    MoonWaxingGibbous5,
    MoonWaxingGibbous6,
    MoonFull,
    MoonWaningGibbous1,
    MoonWaningGibbous2,
    MoonWaningGibbous3,
    MoonWaningGibbous4,
    MoonWaningGibbous5,
    MoonWaningGibbous6,
    MoonThirdQuarter,
    MoonWaningCrescent1,
    MoonWaningCrescent2,
    MoonWaningCrescent3,
    MoonWaningCrescent4,
    MoonWaningCrescent5,
    MoonWaningCrescent6,
    MoonAltNew,
    MoonAltWaxingCrescent1,
    MoonAltWaxingCrescent2,
    MoonAltWaxingCrescent3,
    MoonAltWaxingCrescent4,
    MoonAltWaxingCrescent5,
    MoonAltWaxingCrescent6,
    MoonAltFirstQuarter,
    MoonAltWaxingGibbous1,
    MoonAltWaxingGibbous2,
    MoonAltWaxingGibbous3,
    MoonAltWaxingGibbous4,
    MoonAltWaxingGibbous5,
    MoonAltWaxingGibbous6,
    MoonAltFull,
    MoonAltWaningGibbous1,
    MoonAltWaningGibbous2,
    MoonAltWaningGibbous3,
    MoonAltWaningGibbous4,
    MoonAltWaningGibbous5,
    MoonAltWaningGibbous6,
    MoonAltThirdQuarter,
    MoonAltWaningCrescent1,
    MoonAltWaningCrescent2,
    MoonAltWaningCrescent3,
    MoonAltWaningCrescent4,
    MoonAltWaningCrescent5,
    MoonAltWaningCrescent6,
}

impl Moon {
    pub fn icon(&self) -> &'static str {
        use self::Moon::*;
        match *self {
            MoonNew => "\u{f095}",
            MoonWaxingCrescent1 => "\u{f096}",
            MoonWaxingCrescent2 => "\u{f097}",
            MoonWaxingCrescent3 => "\u{f098}",
            MoonWaxingCrescent4 => "\u{f099}",
            MoonWaxingCrescent5 => "\u{f09a}",
            MoonWaxingCrescent6 => "\u{f09b}",
            MoonFirstQuarter => "\u{f09c}",
            MoonWaxingGibbous1 => "\u{f09d}",
            MoonWaxingGibbous2 => "\u{f09e}",
            MoonWaxingGibbous3 => "\u{f09f}",
            MoonWaxingGibbous4 => "\u{f0a0}",
            MoonWaxingGibbous5 => "\u{f0a1}",
            MoonWaxingGibbous6 => "\u{f0a2}",
            MoonFull => "\u{f0a3}",
            MoonWaningGibbous1 => "\u{f0a4}",
            MoonWaningGibbous2 => "\u{f0a5}",
            MoonWaningGibbous3 => "\u{f0a6}",
            MoonWaningGibbous4 => "\u{f0a7}",
            MoonWaningGibbous5 => "\u{f0a8}",
            MoonWaningGibbous6 => "\u{f0a9}",
            MoonThirdQuarter => "\u{f0aa}",
            MoonWaningCrescent1 => "\u{f0ab}",
            MoonWaningCrescent2 => "\u{f0ac}",
            MoonWaningCrescent3 => "\u{f0ad}",
            MoonWaningCrescent4 => "\u{f0ae}",
            MoonWaningCrescent5 => "\u{f0af}",
            MoonWaningCrescent6 => "\u{f0b0}",
            MoonAltNew => "\u{f0eb}",
            MoonAltWaxingCrescent1 => "\u{f0d0}",
            MoonAltWaxingCrescent2 => "\u{f0d1}",
            MoonAltWaxingCrescent3 => "\u{f0d2}",
            MoonAltWaxingCrescent4 => "\u{f0d3}",
            MoonAltWaxingCrescent5 => "\u{f0d4}",
            MoonAltWaxingCrescent6 => "\u{f0d5}",
            MoonAltFirstQuarter => "\u{f0d6}",
            MoonAltWaxingGibbous1 => "\u{f0d7}",
            MoonAltWaxingGibbous2 => "\u{f0d8}",
            MoonAltWaxingGibbous3 => "\u{f0d9}",
            MoonAltWaxingGibbous4 => "\u{f0da}",
            MoonAltWaxingGibbous5 => "\u{f0db}",
            MoonAltWaxingGibbous6 => "\u{f0dc}",
            MoonAltFull => "\u{f0dd}",
            MoonAltWaningGibbous1 => "\u{f0de}",
            MoonAltWaningGibbous2 => "\u{f0df}",
            MoonAltWaningGibbous3 => "\u{f0e0}",
            MoonAltWaningGibbous4 => "\u{f0e1}",
            MoonAltWaningGibbous5 => "\u{f0e2}",
            MoonAltWaningGibbous6 => "\u{f0e3}",
            MoonAltThirdQuarter => "\u{f0e4}",
            MoonAltWaningCrescent1 => "\u{f0e5}",
            MoonAltWaningCrescent2 => "\u{f0e6}",
            MoonAltWaningCrescent3 => "\u{f0e7}",
            MoonAltWaningCrescent4 => "\u{f0e8}",
            MoonAltWaningCrescent5 => "\u{f0e9}",
            MoonAltWaningCrescent6 => "\u{f0ea}",
        }
    }

    pub fn description(&self) -> &'static str {
        use self::Moon::*;
        match *self {
            MoonNew => "wi-moon-new",
            MoonWaxingCrescent1 => "wi-moon-waxing-crescent-1",
            MoonWaxingCrescent2 => "wi-moon-waxing-crescent-2",
            MoonWaxingCrescent3 => "wi-moon-waxing-crescent-3",
            MoonWaxingCrescent4 => "wi-moon-waxing-crescent-4",
            MoonWaxingCrescent5 => "wi-moon-waxing-crescent-5",
            MoonWaxingCrescent6 => "wi-moon-waxing-crescent-6",
            MoonFirstQuarter => "wi-moon-first-quarter",
            MoonWaxingGibbous1 => "wi-moon-waxing-gibbous-1",
            MoonWaxingGibbous2 => "wi-moon-waxing-gibbous-2",
            MoonWaxingGibbous3 => "wi-moon-waxing-gibbous-3",
            MoonWaxingGibbous4 => "wi-moon-waxing-gibbous-4",
            MoonWaxingGibbous5 => "wi-moon-waxing-gibbous-5",
            MoonWaxingGibbous6 => "wi-moon-waxing-gibbous-6",
            MoonFull => "wi-moon-full",
            MoonWaningGibbous1 => "wi-moon-waning-gibbous-1",
            MoonWaningGibbous2 => "wi-moon-waning-gibbous-2",
            MoonWaningGibbous3 => "wi-moon-waning-gibbous-3",
            MoonWaningGibbous4 => "wi-moon-waning-gibbous-4",
            MoonWaningGibbous5 => "wi-moon-waning-gibbous-5",
            MoonWaningGibbous6 => "wi-moon-waning-gibbous-6",
            MoonThirdQuarter => "wi-moon-third-quarter",
            MoonWaningCrescent1 => "wi-moon-waning-crescent-1",
            MoonWaningCrescent2 => "wi-moon-waning-crescent-2",
            MoonWaningCrescent3 => "wi-moon-waning-crescent-3",
            MoonWaningCrescent4 => "wi-moon-waning-crescent-4",
            MoonWaningCrescent5 => "wi-moon-waning-crescent-5",
            MoonWaningCrescent6 => "wi-moon-waning-crescent-6",
            MoonAltNew => "wi-moon-alt-new",
            MoonAltWaxingCrescent1 => "wi-moon-alt-waxing-crescent-1",
            MoonAltWaxingCrescent2 => "wi-moon-alt-waxing-crescent-2",
            MoonAltWaxingCrescent3 => "wi-moon-alt-waxing-crescent-3",
            MoonAltWaxingCrescent4 => "wi-moon-alt-waxing-crescent-4",
            MoonAltWaxingCrescent5 => "wi-moon-alt-waxing-crescent-5",
            MoonAltWaxingCrescent6 => "wi-moon-alt-waxing-crescent-6",
            MoonAltFirstQuarter => "wi-moon-alt-first-quarter",
            MoonAltWaxingGibbous1 => "wi-moon-alt-waxing-gibbous-1",
            MoonAltWaxingGibbous2 => "wi-moon-alt-waxing-gibbous-2",
            MoonAltWaxingGibbous3 => "wi-moon-alt-waxing-gibbous-3",
            MoonAltWaxingGibbous4 => "wi-moon-alt-waxing-gibbous-4",
            MoonAltWaxingGibbous5 => "wi-moon-alt-waxing-gibbous-5",
            MoonAltWaxingGibbous6 => "wi-moon-alt-waxing-gibbous-6",
            MoonAltFull => "wi-moon-alt-full",
            MoonAltWaningGibbous1 => "wi-moon-alt-waning-gibbous-1",
            MoonAltWaningGibbous2 => "wi-moon-alt-waning-gibbous-2",
            MoonAltWaningGibbous3 => "wi-moon-alt-waning-gibbous-3",
            MoonAltWaningGibbous4 => "wi-moon-alt-waning-gibbous-4",
            MoonAltWaningGibbous5 => "wi-moon-alt-waning-gibbous-5",
            MoonAltWaningGibbous6 => "wi-moon-alt-waning-gibbous-6",
            MoonAltThirdQuarter => "wi-moon-alt-third-quarter",
            MoonAltWaningCrescent1 => "wi-moon-alt-waning-crescent-1",
            MoonAltWaningCrescent2 => "wi-moon-alt-waning-crescent-2",
            MoonAltWaningCrescent3 => "wi-moon-alt-waning-crescent-3",
            MoonAltWaningCrescent4 => "wi-moon-alt-waning-crescent-4",
            MoonAltWaningCrescent5 => "wi-moon-alt-waning-crescent-5",
            MoonAltWaningCrescent6 => "wi-moon-alt-waning-crescent-6",
        }
    }
}

pub fn moon_phase(phase: f64) -> char {
    use std::char;
    // '\u{f095}', i.e. wi-moon-new
    let new_moon = 61589u32;
    // '\u{f0eb}', i.e. wi-moon-alt-new
    //let new_moon = 61648u32;
    if phase > 1f64 || phase < 0f64 {
        panic!("Moon phase out of bounds");
    }

    let lunar_number = match (phase * 28f64).round() as u32 {
        n @ 0...27 => n,
        28 => 0,
        _ => panic!("Moon phase out of bounds"),
    };

    char::from_u32(new_moon + lunar_number).unwrap()
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.icon())
    }
}

impl fmt::Debug for Moon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {:?}", self.description(), self.icon())
    }
}
