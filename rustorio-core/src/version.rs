use std::{
    fmt::{Display, Formatter, Result as FormatResult},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"(\d+)\.(\d+)(\.(\d+))?").unwrap();
}

#[derive(Debug, Error)]
#[error("Could not parse version: {0}")]
pub struct VersionParseError(String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Version {
    pub major: u16,
    pub minor: u16,
    pub patch: u16,
}

impl Version {
    pub fn new(major: u16, minor: u16, patch: u16) -> Self {
        Self { major, minor, patch }
    }
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || VersionParseError(s.to_owned());

        if let Some(captures) = REGEX.captures(s) {
            let major = captures.get(1).ok_or_else(err)?.as_str().parse().map_err(|_| err())?;
            let minor = captures.get(2).ok_or_else(err)?.as_str().parse().map_err(|_| err())?;
            let patch = captures.get(4).map(|m| m.as_str().parse()).transpose().map_err(|_| err())?.unwrap_or_default();
            Ok(Version::new(major, minor, patch))
        } else {
            Err(err())
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn it_parses() {
        let v: Version = "1.2.3".parse().unwrap();
        assert_eq!(v.major, 1);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 3);

        let v: Version = "0.2".parse().unwrap();
        assert_eq!(v.major, 0);
        assert_eq!(v.minor, 2);
        assert_eq!(v.patch, 0);
    }
}
