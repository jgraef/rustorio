pub mod types;

use std::{
    str::FromStr,
    collections::BTreeMap,
};

use serde::{Serialize, Deserialize};
use thiserror::Error;
use libflate::zlib;
use parse_display::{FromStr, Display};

use crate::{
    types::{Color, ItemStackIndex, ItemCountType, UnitNumber},
    blueprint::types::{Position, SignalID, Signal},
};


#[derive(Debug, Error)]
pub enum Error {
    #[error("Incorrect version prefix: {0}")]
    IncorrectVersionPrefix(String),

    #[error("Base 64 error: {0}")]
    Base64(#[from] base64::DecodeError),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Zlib error: {0}")]
    Zlib(std::io::Error),
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case", deny_unknown_fields)]
pub enum Envelope {
    Blueprint(Blueprint),
    BlueprintBook(BlueprintBook)
}

impl FromStr for Envelope {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::decode(s)
    }
}

impl Envelope {
    pub fn encode(&self) -> Result<String, Error> {
        todo!()
    }

    pub fn decode(s: &str) -> Result<Self, Error> {
        let version_prefix = &s[..1];

        if version_prefix != "0" {
            return Err(Error::IncorrectVersionPrefix(version_prefix.to_owned()));
        }

        let compressed = base64::decode(&s[1..])?;
        let zlib_decoder = zlib::Decoder::new(&compressed[..]).map_err(|e| Error::Zlib(e))?;

        //let value: serde_json::Value = serde_json::from_reader(zlib_decoder)?;
        //println!("{:#?}", value);
        //todo!();

        Ok(serde_json::from_reader(zlib_decoder)?)
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BlueprintBook {
    item: String,

    #[serde(skip_serializing_if="Option::is_none")]
    label: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    label_color: Option<Color>,

    blueprints: BTreeMap<usize, Blueprint>,

    active_index: ItemStackIndex,

    version: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Blueprint {
    item: String,

    #[serde(skip_serializing_if="Option::is_none")]
    label: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    label_color: Option<Color>,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    entities: Vec<Entity>,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    tiles: Vec<Tile>,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    icons: Vec<Icon>,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    schedules: Vec<Schedule>,

    version: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Icon {
    index: usize,
    signal: SignalID,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entity {
    pub entity_number: UnitNumber,

    pub name: String,

    pub position: Position,

    #[serde(skip_serializing_if="Option::is_none")]
    pub direction: Option<u8>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub orientation: Option<u8>,

    #[serde(default)]
    pub connections: Option<Connection>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub control_behavior: Option<ControlBehavior>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<BTreeMap<String, u32>>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub recipe: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub bar: Option<u32>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub inventory: Option<Inventory>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub infinity_settings: Option<InfinitySettings>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub r#type: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub input_priority: Option<SplitterPriority>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub output_priority: Option<SplitterPriority>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub filter: Option<String>,

    #[serde(default)]
    pub filters: Vec<ItemFilter>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub filter_mode: Option<FilterMode>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub override_stack_size: Option<u8>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub drop_position: Option<Position>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub pickup_position: Option<Position>,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub request_filters: Vec<RequestFilter>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub request_from_buffers: Option<bool>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub parameters: Option<SpeakerParameter>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub alert_parameters: Option<SpeakerAlertParameter>, // TODO

    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_launch: Option<bool>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub variation: Option<u8>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub station: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControlBehavior {
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub filters: Vec<SignalFilter>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub arithmetic_conditions: Option<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub decider_conditions: Option<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub circuit_condition: Option<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub circuit_parameters: Option<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub read_logistics: Option<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub read_robot_stats: Option<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub read_from_train: Option<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub read_stopped_train: Option<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub train_stopped_signal: Option<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub circuit_mode_of_operation: Option<u8>,
}

impl ControlBehavior {
    pub fn add_filter(&mut self, signal: Signal) {
        let last_index = self.filters
            .last()
            .map(|f| f.index)
            .unwrap_or_default();

        self.filters.push(SignalFilter {
            signal: SignalID::new(signal.r#type, signal.name),
            count: signal.count,
            index: last_index + 1,
        });
    }

    pub fn match_filters<F>(&mut self, signal_id: &SignalID, mut f: F)
        where F: FnMut(&mut ControlBehavior)
    {
        let mut matched = false;

        for filter in &self.filters {
            if &filter.signal == signal_id {
                matched = true;
                break;
            }
        }

        if matched {
            f(self)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestFilter {
    pub count: ItemCountType,
    pub index: ItemStackIndex,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Connection {
    #[serde(rename="1")]
    pub first: ConnectionPoint,
    #[serde(rename="2", skip_serializing_if="Option::is_none")]
    pub second: Option<ConnectionPoint>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectionPoint {
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    red: Vec<ConnectionData>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    green: Vec<ConnectionData>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectionData {
    entity_id: UnitNumber,
    circuit_id: Option<u32>, // TODO: What's this?
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Cables {
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub red: Vec<Connection>,
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub green: Vec<Connection>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ItemFilter {
    pub name: String,
    pub index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignalFilter {
    pub signal: SignalID,
    pub count: i32,
    pub index: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FilterMode {
    #[serde(rename="blacklist")]
    Deny,
    #[serde(rename="whitelist")]
    Allow,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="lowercase")]
pub enum SplitterPriority {
    Left,
    Right,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Inventory {
    pub filters: Vec<ItemFilter>,
    pub bar: Option<ItemStackIndex>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfinitySettings {
    pub remove_unfiltered_items: bool,
    pub filters: Vec<InfinityFilter>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InfinityFilter {
    pub name: String,
    pub count: ItemCountType,
    pub mode: InfinityFilterMode,
    pub index: ItemStackIndex,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all="kebab-case")]
pub enum InfinityFilterMode {
    AtLeast,
    AtMost,
    Exactly
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpeakerParameter {
    pub playback_volume: f64,
    #[serde(default)]
    pub playback_globally: bool,
    #[serde(default)]
    pub allow_polyphony: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpeakerAlertParameter {
    #[serde(default)]
    pub show_alert: bool,
    #[serde(default)]
    pub show_on_map: bool,
    #[serde(skip_serializing_if="Option::is_none")]
    pub icon_signal_id: Option<SignalID>,
    #[serde(default)]
    pub alert_message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tile {
    pub name: String,
    pub position: Position,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Schedule {
    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub schedule: Vec<ScheduleRecord>,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub locomotives: Vec<UnitNumber>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleRecord {
    station: String,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    wait_conditions: Vec<WaitCondition>
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WaitCondition {
    r#type: WaitConditionType,
    #[serde(skip_serializing_if="Option::is_none")]
    compare_type: Option<WaitConditionCompareType>,
    #[serde(skip_serializing_if="Option::is_none")]
    ticks: Option<u64>,
    #[serde(skip_serializing_if="Option::is_none")]
    condition: Option<serde_json::Value>, // TODO
}

#[derive(Clone, Debug, Serialize, Deserialize, FromStr, Display)]
#[serde(rename_all="snake_case")]
#[display(style="snake_case")]
pub enum WaitConditionType {
    Time,
    Inactivity,
    Full,
    Empty,
    ItemCount,
    Circuit,
    RobotsInactive,
    FluidCount,
    PassengerPresent,
    PassengerNotPresent,
}

#[derive(Clone, Debug, Serialize, Deserialize, FromStr, Display)]
#[serde(rename_all="lowercase")]
#[display(style="lowercase")]
pub enum WaitConditionCompareType {
    And,
    Or
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_decodes_blueprints() {
        let bp = "0eNqV1clqwzAQBuB3mbMcYteOY9+673tvJQTbUdsBWwqKHBqM3r1SAiWUSWFulhh9kszPaIC67eXSoLJQDoCNViso3wdY4aeq2jBnN0sJJaCVHQhQVRdGoc5WykaN7mpUldUGnABUC/kNZexmAqSyaFHuuO1gM1d9V0vjC/6FBCz1yq/VKuzvvSgZjzIBG/9VjDK/zwKNbHYFqQiGNbqd1/KrWqMH/KoPbK00B+6yRmN7P/N7il1FdBzu0Og+/Ix47zaCYZyQRsIyTknjiGWckUbKMs5JI2MZF6QxYRmXpJGzjCvSmLKMa9IoWMYNnbExC7mlEV5S72iEF9V7GuFl9YFGeGF9pBFeWp9ohBfXZxrh5fWFRniBfaURXmLf6I42djMXnD+tPWG29rg43NrDM7J9d8q9Z0rA2jf2bUEyjdO8SPJ4mqfZJHbuBxRpOX0=";

        let bp = Envelope::decode(bp).unwrap();
        println!("{:#?}", bp);
    }

    #[test]
    fn circuit_connections() {
        let bp = "0eNq1Ve1ugzAMfBf/nGhVUhAr2ptMFeLDay2RBIVQrap49znpylr6PW1/qjrEd+c7C3ZQ1B02hpSFdAdUatVC+r6DllYqr92Z3TYIKZBFCQGoXLoqN2TXEi2Vk1LLglRutYE+AFIVfkIa9sFdjApLqtBcBhD9MgBUlizhXpEvtpnqZIGGGW7hBNDollu1cuwMN4nENA5gy/+SiFl4Tmt0nRW4zjfEHXztGyfjZ5Xvbd2pc8Tmzp6Za5NNbjxFCm/gDpotN3TKZh9Gy4xU0/FVazrs9zQKywErdD8Gq+N5iCvBN8mUHVlfevOOHsfOi5VBVOPG8LRRjBrPcJesSTwmQjxKGt4mFefD9P3R2SFPcWe3LkQ6vxvpD9Qo1RZdnZ2EqxvkaD0BvMAT8T1ryMPGPhHXmDT8Jam4Es98iEdiRZ2cYM3OGM6o0TVeSmc2pBNP48fNvL7s1zYnGqS1FrGelGts7W1F8/9VFA+KDgt2Z5PDkbKKzF6Yn+5v1jAa1fPfvlUuALELDOZf7+nRFyWADZrWjyFewyhZiCRcRLPZIun7L9H0Mkw=";
        let bp = Envelope::decode(bp).unwrap();
        println!("{:#?}", bp);
    }
}
