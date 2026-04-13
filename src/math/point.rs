use serde::{Deserialize, Serialize};
use std::{
    fmt::Display,
    ops::{Add, Mul, Sub},
};

/// The radius of the earth in meters.
pub const R: f64 = 6_371_200.0;

/// The arc-distance within which coordinates are considered equal. In degrees longitude at the
/// equator this is roughly 11 centimeters.
pub const EPSILON: f64 = 1e-6;

/// The conversion factor from degrees to radians.
const DEG2RAD: f64 = 1f64.to_radians();

/// The conversion factor from radians to degrees.
const RAD2DEG: f64 = 1f64.to_degrees();

/// A geographic coordinate.
#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Point {
    /// The latitude of a coordinate in degrees north of the equator [-90 to 90].
    pub lat: f64,
    /// The longitude of a coordinate in degrees east of the prime meridian (-180 to 180].
    pub lon: f64,
}

impl Point {
    /// Create a new point.
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
    }

    /// Guarantee that the coordinates fall within the valid range. Returns true if the point is modified.
    pub fn normalize(&mut self) -> bool {
        if (-90.0..=90.0).contains(&self.lat)
            && (-180.0..=180.0).contains(&self.lon)
        {
            return false;
        }

        let rads = *self * DEG2RAD;
        let x = rads.lat.cos() * rads.lon.sin();
        let y = rads.lat.sin();
        let z = rads.lat.cos() * rads.lon.cos();
        let rads = Self {
            lat: y.asin(),
            lon: x.atan2(z),
        };

        *self = rads * RAD2DEG;
        true
    }

    /// Returns a normalized copy of this point.
    pub fn normalized(&self) -> Self {
        let mut new = self.clone();
        new.normalize();
        new
    }

    /// Calculate the haversine distance between two points.
    pub fn distance_to(&self, rhs: Point) -> f64 {
        let p1 = *self * DEG2RAD;
        let p2 = rhs * DEG2RAD;
        let diff = Point {
            lat: (p1.lat - p2.lat).abs(),
            lon: (p1.lon - p2.lon).abs(),
        };

        let sinlat2 = (diff.lat / 2.).sin();
        let sinlon2 = (diff.lon / 2.).sin();
        let a = (sinlat2 * sinlat2)
            + (p1.lat.cos() * p2.lat.cos() * sinlon2 * sinlon2);
        a.sqrt().asin() * 2. * R
    }
}

/// Whether two coordinate pairs are approximately equal, i.e. both coordinates are within
/// [EPSILON] of each other.
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.lat - other.lat).abs() < EPSILON
            && (self.lon - other.lon).abs() < EPSILON
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            lat: self.lat - rhs.lat,
            lon: self.lon - rhs.lon,
        }
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            lat: self.lat + rhs.lat,
            lon: self.lon + rhs.lon,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Point;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            lat: self.lat * rhs,
            lon: self.lon * rhs,
        }
    }
}

impl Mul for Point {
    type Output = Point;
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            lat: self.lat * rhs.lat,
            lon: self.lon * rhs.lon,
        }
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.lat >= 0. {
            write!(f, "({:.3}° N, ", self.lat)?;
        } else {
            write!(f, "({:.3}° S, ", self.lat.abs())?;
        }

        if self.lon >= 0. {
            write!(f, "{:.3}° E)", self.lon)
        } else {
            write!(f, "{:.3}° W)", self.lon.abs())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn distance() {
        let white_house = Point::new(38.898, -77.037);
        let eiffel_tower = Point::new(48.858, 2.294);
        let dist = white_house.distance_to(eiffel_tower);

        // distance should be approx 6,161.6km
        assert!((dist - 6_161_600.).abs() < 100.);
    }

    #[test]
    fn normalize() {
        let actual = Point::new(170., 330.).normalized();
        let expected = Point::new(10., 150.);
        assert_eq!(actual, expected);
    }
}
