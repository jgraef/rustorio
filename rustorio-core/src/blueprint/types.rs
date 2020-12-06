use std::{
    cmp::Ordering,
    str::FromStr
};

use nalgebra::{Vector2, Point2};
use serde::{Serialize, Deserialize};
use regex::Regex;
use lazy_static::lazy_static;
use parse_display::{Display, FromStr};

use crate::blueprint::Error;


// TODO: These are referred to by the Factorio Lua API as "Concepts". So maybe move this to `rustorio_core::concepts`.
//       The Position type might also just be a `Vector2<f32>`, but we need a custom serialization.


lazy_static! {
    // TODO Where does this format come from?
    static ref SIGNAL_REGEX: Regex = Regex::new(r"([a-z0-9-]+)=([a-z0-9-]+):(\d+)").unwrap();
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Point2<f32>> for Position {
    fn from(point: Point2<f32>) -> Self {
        Self {
            x: point.x,
            y: point.y
        }
    }
}

impl From<Position> for Point2<f32> {
    fn from(position: Position) -> Self {
        Point2::new(
            position.x,
            position.y
        )
    }
}

impl From<Vector2<f32>> for Position {
    fn from(vector: Vector2<f32>) -> Self {
        Self {
            x: vector.x,
            y: vector.y
        }
    }
}

impl From<Position> for Vector2<f32> {
    fn from(position: Position) -> Self {
        Vector2::new(
            position.x,
            position.y
        )
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Display, FromStr, PartialOrd, Ord)]
#[serde(rename_all="lowercase")]
#[display(style="lowercase")]
pub enum SignalType {
    Item,
    Fluid,
    Virtual,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignalID {
    pub r#type: SignalType,
    pub name: String,
}

#[derive(Debug, Error)]
#[error("Invalid signal: {0}")]
pub struct SignalIDParseError(String);

impl FromStr for SignalID {
    type Err = SignalIDParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let e = || SignalIDParseError(s.to_owned());

        let mut parts = s.splitn(2, '-');
        let r#type = parts.next().unwrap().parse().map_err(|_| e())?;
        let name = parts.next().ok_or_else(e)?.to_owned();

        Ok(Self {
            r#type,
            name,
        })
    }
}

// TODO: Isn't that the same as when we derive Ord?
impl Ord for SignalID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.r#type.cmp(&other.r#type)
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for SignalID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SignalID {
    pub fn new(r#type: SignalType, name: String) -> Self {
        Self {
            r#type,
            name,
        }
    }

    pub fn new_item(name: String) -> Self {
        Self::new(SignalType::Item, name)
    }

    pub fn new_fluid(name: String) -> Self {
        Self::new(SignalType::Fluid, name)
    }

    pub fn new_virtual(name: String) -> Self {
        Self::new(SignalType::Virtual, name)
    }

    pub fn from_letter(c: char) -> Self {
        Self::new_virtual(format!("signal-{}", c.to_ascii_uppercase()))
    }

    pub fn into_signal(self, count: i32) -> Signal {
        Signal {
            r#type: self.r#type,
            name: self.name,
            count,
        }
    }
}

#[derive(Debug, Error)]
pub enum SignalParseError {
    #[error("Failed to parse signal: {0}")]
    InvalidSignal(String),

    #[error("Failed to parse signal type: {0}")]
    InvalidType(#[from] parse_display::ParseError),

    #[error("Failed to parse signal count: {0}")]
    InvalidCount(#[from] std::num::ParseIntError),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signal {
    pub r#type: SignalType,
    pub name: String,
    pub count: i32,
}

impl FromStr for Signal {
    type Err = SignalParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let e = || SignalParseError::InvalidSignal(s.to_owned());

        let captures = SIGNAL_REGEX.captures(s).ok_or_else(e)?;
        let r#type = captures.get(1).ok_or_else(e)?.as_str().parse()?;
        let name = &captures.get(2).ok_or_else(e)?.as_str();
        let count = captures.get(3).ok_or_else(e)?.as_str().parse()?;

        Ok(Self {
            name: String::from(*name),
            r#type,
            count,
        })
    }
}

impl Signal {
    pub fn new(r#type: SignalType, name: String, count: i32) -> Self {
        Self {
            r#type,
            name,
            count
        }
    }

    pub fn new_item(name: String, count: i32) -> Self {
        Self::new(SignalType::Item, name, count)
    }

    pub fn new_fluid(name: String, count: i32) -> Self {
        Self::new(SignalType::Fluid, name, count)
    }

    pub fn new_virtual(name: String, count: i32) -> Self {
        Self::new(SignalType::Virtual, name, count)
    }

    pub fn into_signal(self) -> SignalID {
        SignalID {
            name: self.name,
            r#type: self.r#type
        }
    }
}

