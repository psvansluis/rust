#[derive(Debug)]
pub struct Duration {
    years: f64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;
        Duration {
            years: s as f64 / EARTH_YEAR_IN_SECONDS,
        }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD_IN_EARTH_YEARS: f64;
    fn years_during(d: &Duration) -> f64 {
        d.years / Self::ORBITAL_PERIOD_IN_EARTH_YEARS
    }
}

macro_rules! planet {
    ($name:ident, $period:expr) => {
        pub struct $name;
        impl Planet for $name {
            const ORBITAL_PERIOD_IN_EARTH_YEARS: f64 = $period;
        }
    };
}

planet!(Mercury, 0.2408467);
planet!(Venus, 0.61519726);
planet!(Earth, 1.0);
planet!(Mars, 1.8808158);
planet!(Jupiter, 11.862615);
planet!(Saturn, 29.447498);
planet!(Uranus, 84.016846);
planet!(Neptune, 164.79132);
