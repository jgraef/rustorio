use std::{
    cmp::Ordering,
    str::FromStr,
};

#[cfg(feature = "nalgebra")]
use nalgebra::{
    Point2,
    Vector2,
};
use parse_display::{
    Display,
    FromStr,
};
use regex::Regex;
use serde::{
    Deserialize,
    Serialize,
};

use crate::error::Error;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Color {
    #[serde(default)]
    pub r: f32,
    #[serde(default)]
    pub g: f32,
    #[serde(default)]
    pub b: f32,
    #[serde(default)]
    pub a: f32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Display, FromStr)]
#[serde(rename_all = "lowercase")]
pub enum SignalType {
    Item,
    Fluid,
    Virtual,
}

impl SignalType {
    fn key(&self) -> usize {
        match self {
            SignalType::Item => 1,
            SignalType::Fluid => 2,
            SignalType::Virtual => 3,
        }
    }
}

impl Ord for SignalType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key().cmp(&other.key())
    }
}

impl PartialOrd for SignalType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignalID {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: SignalType,
}

impl Ord for SignalID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ty
            .cmp(&other.ty)
            .then_with(|| self.name.cmp(&other.name))
    }
}

impl PartialOrd for SignalID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SignalID {
    pub fn new(name: String, ty: SignalType) -> Self {
        Self { name, ty }
    }

    pub fn new_item(name: String) -> Self {
        Self::new(name, SignalType::Item)
    }

    pub fn new_fluid(name: String) -> Self {
        Self::new(name, SignalType::Fluid)
    }

    pub fn new_virtual(name: String) -> Self {
        Self::new(name, SignalType::Virtual)
    }

    pub fn from_letter(c: char) -> Self {
        Self::new_virtual(format!("signal-{}", c.to_ascii_uppercase()))
    }

    pub fn into_signal(self, count: i32) -> Signal {
        Signal {
            name: self.name,
            ty: self.ty,
            count,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Signal {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: SignalType,
    pub count: i32,
}

impl FromStr for Signal {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: make lazy_static
        let re = Regex::new(r"([a-z0-9-]+)=([a-z0-9-]+):(\d+)").unwrap();

        let e = || Error::InvalidSignal(s.to_owned());

        let captures = re.captures(s).ok_or_else(e)?;
        let ty = captures.get(1).ok_or_else(e)?.as_str().parse()?;
        let name = &captures.get(2).ok_or_else(e)?.as_str();
        let count = captures.get(3).ok_or_else(e)?.as_str().parse()?;

        Ok(Self {
            name: String::from(*name),
            ty,
            count,
        })
    }
}

impl Signal {
    pub fn new(name: String, ty: SignalType, count: i32) -> Self {
        Self { name, ty, count }
    }

    pub fn new_item(name: String, count: i32) -> Self {
        Self::new(name, SignalType::Item, count)
    }

    pub fn new_fluid(name: String, count: i32) -> Self {
        Self::new(name, SignalType::Fluid, count)
    }

    pub fn new_virtual(name: String, count: i32) -> Self {
        Self::new(name, SignalType::Virtual, count)
    }

    pub fn into_signal(self) -> SignalID {
        SignalID {
            name: self.name,
            ty: self.ty,
        }
    }
}

pub type ChannelId = i32;

pub type UnitNumber = u32;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainSchedule {
    pub current: u32,
    pub records: Vec<TrainScheduleRecord>,
}

impl Default for TrainSchedule {
    fn default() -> Self {
        Self {
            current: 1,
            records: vec![],
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrainScheduleRecord {
    pub station: Option<String>,

    // TODO: Handling is not implemented, so always set this to None
    pub rail: Option<[f32; 2]>,

    #[serde(default)]
    pub wait_conditions: Vec<WaitCondition>,

    #[serde(default)]
    pub temporary: bool,
}

impl TrainScheduleRecord {
    pub fn station(name: &str, wait_conditions: Vec<WaitCondition>, temporary: bool) -> Self {
        Self {
            station: Some(name.to_owned()),
            rail: None,
            wait_conditions,
            temporary,
        }
    }

    pub fn rail(x: f32, y: f32, wait_conditions: Vec<WaitCondition>, temporary: bool) -> Self {
        Self {
            station: None,
            rail: Some([x, y]),
            wait_conditions,
            temporary,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WaitCondition {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub ty: WaitConditionType,
    pub compare_type: CompareType,
}

impl WaitCondition {
    pub fn single(ty: WaitConditionType) -> Vec<Self> {
        vec![Self {
            ty,
            compare_type: CompareType::Or,
        }]
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompareType {
    And,
    Or,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CircuitCondition {
    pub comparator: String,
    pub first_signal: SignalID,
    pub second_signal: Option<SignalID>,
    pub constant: Option<i32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WaitConditionType {
    Time { ticks: usize },
    Inactivity { ticks: usize },
    Full,
    Empty,
    ItemCount { condition: CircuitCondition },
    Circuit { condition: () },
    RobotsInactive,
    FluidCount { condition: () },
    PassengerPresent,
    PassengerNotPresent,
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

#[cfg(feature = "nalgebra")]
impl From<Point2<f32>> for Position {
    fn from(point: Point2<f32>) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

#[cfg(feature = "nalgebra")]
impl From<Position> for Point2<f32> {
    fn from(position: Position) -> Self {
        Point2::new(position.x, position.y)
    }
}

#[cfg(feature = "nalgebra")]
impl From<Vector2<f32>> for Position {
    fn from(vector: Vector2<f32>) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
        }
    }
}

#[cfg(feature = "nalgebra")]
impl From<Position> for Vector2<f32> {
    fn from(position: Position) -> Self {
        Vector2::new(position.x, position.y)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct Entity {
    pub unit_number: Option<UnitNumber>,
    pub name: String,
    pub position: Position,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RadarData {
    pub entities: Vec<Entity>,
    pub origin: Position,
    pub radar: UnitNumber,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Screenshot {
    pub position: Position,
    pub file_name: String,
    pub tick: u32,
}
