use std::ops::{Add, Mul, Sub};
use serde::{Deserialize, Serialize};

/// The radius of the earth in meters.
const R: f64 = 6_371_200.0;

const DEG2RAD: f64 = 3.141592658589 / 180.0;

/// A geographic coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Point {
    /// The latitude of a coordinate in degrees north of the equator (-90 to 90).
    pub lat: f64,
    /// The longitude of a coordinate in degrees east of the prime meridian (-180 to 180).
    pub lon: f64,
}

impl Point {
    /// Create a new point.
    pub fn new(lat: f64, lon: f64) -> Self {
        Self { lat, lon }
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
        let a = (sinlat2 * sinlat2) + (p1.lat.cos() * p2.lat.cos() * sinlon2 * sinlon2);
        a.sqrt().asin() * 2. * R
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
}