// src/lib.rs

#[derive(Debug)]
pub struct Duration(u64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64;
}

const SECONDS_PER_EARTH_YEAR: f64 = 31_557_600.0;

macro_rules! impl_planet {
    ($planet:ident, $orbital_period:expr) => {
        pub struct $planet;

        impl Planet for $planet {
            fn years_during(d: &Duration) -> f64 {
                (d.0 as f64) / (SECONDS_PER_EARTH_YEAR * $orbital_period)
            }
        }
    };
}

// Create each planet with its orbital period constant
impl_planet!(Mercury, 0.2408467);
impl_planet!(Venus, 0.61519726);
impl_planet!(Earth, 1.0);
impl_planet!(Mars, 1.8808158);
impl_planet!(Jupiter, 11.862615);
impl_planet!(Saturn, 29.447498);
impl_planet!(Uranus, 84.016846);
impl_planet!(Neptune, 164.79132);
