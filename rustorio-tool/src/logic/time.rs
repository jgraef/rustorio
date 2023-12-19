use std::{
    ops::{
        Add,
        AddAssign,
        Div,
        Mul,
        Sub,
        SubAssign,
    },
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;
use serde::{
    Deserialize,
    Serialize,
};

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Instant(Time);

impl Add<Duration> for Instant {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign<Duration> for Instant {
    fn add_assign(&mut self, rhs: Duration) {
        self.0 += rhs.0;
    }
}

impl Sub<Duration> for Instant {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign<Duration> for Instant {
    fn sub_assign(&mut self, rhs: Duration) {
        self.0 -= rhs.0;
    }
}

impl Sub for Instant {
    type Output = Duration;

    fn sub(self, rhs: Self) -> Self::Output {
        Duration(self.0 - rhs.0)
    }
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize, PartialOrd, PartialEq)]
#[serde(transparent)]
pub struct Duration(Time);

impl Duration {
    pub fn from_seconds(seconds: f64) -> Self {
        Self(Time(seconds))
    }

    pub fn from_minutes(minutes: f64) -> Self {
        Self(Time(minutes * 60.))
    }

    pub fn as_seconds(&self) -> f64 {
        self.0 .0
    }

    pub fn min(&self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    pub fn max(&self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }
}

impl Add<Instant> for Duration {
    type Output = Instant;

    fn add(self, rhs: Instant) -> Instant {
        Instant(self.0 + rhs.0)
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, rhs: Duration) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl Sub<Instant> for Duration {
    type Output = Instant;

    fn sub(self, rhs: Instant) -> Instant {
        Instant(self.0 - rhs.0)
    }
}

impl Sub for Duration {
    type Output = Self;

    fn sub(self, rhs: Duration) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl Mul<f64> for Duration {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<f64> for Duration {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

lazy_static! {
    static ref TIME_REGEX: Regex = r"^(\d+):(\d{1,2}):(\d{1, 2})(\.(\d{1,3}))?$"
        .parse()
        .unwrap();
}

#[derive(Debug, Default)]
pub struct Clock {
    now: Instant,
}

impl Clock {
    pub fn new(start: Instant) -> Self {
        Self { now: start }
    }

    pub fn advance(&mut self, dt: Duration) {
        self.now += dt;
    }

    pub fn now(&self) -> Instant {
        self.now
    }
}

#[derive(Copy, Clone, Default, PartialOrd, PartialEq)]
struct Time(f64);

impl Time {
    pub fn min(&self, other: Self) -> Self {
        Self(self.0.min(other.0))
    }

    pub fn max(&self, other: Self) -> Self {
        Self(self.0.max(other.0))
    }
}

impl std::fmt::Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let h = (self.0 / 3600.) as u64;
        let m = ((self.0 / 60.) % 60.) as u64;
        let s = (self.0 % 60.) as u64;
        let ms = ((self.0 % 1.) * 1000.) as u64;
        write!(f, "{h}:{m:02}:{s:02}.{ms:03}")
    }
}

impl std::fmt::Debug for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl FromStr for Time {
    type Err = TimeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || TimeParseError(s.to_owned());
        let captures = TIME_REGEX.captures(s).ok_or_else(err)?;
        let h: u64 = captures
            .get(1)
            .ok_or_else(err)?
            .as_str()
            .parse()
            .map_err(|_| err())?;
        let m: u64 = captures
            .get(2)
            .ok_or_else(err)?
            .as_str()
            .parse()
            .map_err(|_| err())?;
        if m >= 60 {
            return Err(err());
        }
        let s: u64 = captures
            .get(3)
            .ok_or_else(err)?
            .as_str()
            .parse()
            .map_err(|_| err())?;
        if s >= 60 {
            return Err(err());
        }
        let ms: u64 = captures
            .get(5)
            .map(|m| m.as_str().parse().map_err(|_| err()))
            .transpose()?
            .unwrap_or_default();
        if ms >= 1000 {
            return Err(err());
        }
        let t = (h as f64) * 3600. + (m as f64) * 60. + s as f64 + (ms as f64) * 0.001;
        Ok(Self(t))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("invalid time: {0}")]
pub struct TimeParseError(String);

impl Serialize for Time {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.to_string().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Time {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(|e| serde::de::Error::custom(e))
    }
}

impl Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Time {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Time {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Time {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

impl Mul<f64> for Time {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl Div<f64> for Time {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self(self.0 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn it_parses_times() {
        assert_eq!(Time::from_str("1:23:45.678").unwrap().0, 5025.678);
        assert_eq!(Time::from_str("1:23:45").unwrap().0, 5025.0);
    }

    #[test]
    fn it_rejects_invalid_times() {
        assert!(Time::from_str("1:61:00.123").is_err());
        assert!(Time::from_str("1:12:61.123").is_err());
        assert!(Time::from_str("1:12:34.1234").is_err());
        assert!(Time::from_str("1:23.123").is_err());
    }

    #[test]
    fn it_formats_times() {
        assert_eq!(Time(5025.678).to_string(), "1:23:45.678");
    }
}
