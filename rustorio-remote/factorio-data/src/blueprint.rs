use std::{
    io::Cursor,
    str::FromStr,
    collections::HashMap,
};

use libflate::zlib;
use serde::{Serialize, Deserialize};

use crate::{
    error::Error,
    types::{UnitNumber, Position, SignalID, Signal, Color, SignalType},
};


#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct BlueprintEnvelope {
    pub blueprint: Blueprint,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestFilter {
    pub count: i32,
    pub index: usize,
    pub name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Connection {
    entity_id: UnitNumber,
    #[serde(default, skip_serializing_if="Option::is_none")]
    circuit_id: Option<UnitNumber>,
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
            signal: SignalID::new(signal.name, signal.ty),
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


/// See [1]
///
/// [1] https://wiki.factorio.com/Blueprint_string_format#Entity_object
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Entity {
    pub entity_number: UnitNumber,

    pub name: String,

    pub position: Position,

    #[serde(skip_serializing_if="Option::is_none")]
    pub direction: Option<u32>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub orientation: Option<f32>,

    #[serde(default)]
    pub connections: HashMap<String, Cables>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub control_behavior: Option<ControlBehavior>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub items: Option<serde_json::Value>, // TODO

    #[serde(skip_serializing_if="Option::is_none")]
    pub recipe: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub bar: Option<u32>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub inventory: Option<serde_json::Value>, // TODO

    #[serde(skip_serializing_if="Option::is_none")]
    pub infinity_settings: Option<serde_json::Value>, // TODO

    #[serde(rename="type", skip_serializing_if="Option::is_none")]
    pub ty: Option<String>,

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
    pub parameters: Option<serde_json::Value>, // TODO

    #[serde(skip_serializing_if="Option::is_none")]
    pub alert_parameters: Option<serde_json::Value>, // TODO

    #[serde(skip_serializing_if="Option::is_none")]
    pub auto_launch: Option<bool>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub variation: Option<u8>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub color: Option<Color>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub station: Option<String>,
}

impl Entity {
    pub fn match_request_filters<F>(&mut self, signal_id: &SignalID, mut f: F)
        where F: FnMut(&mut Vec<RequestFilter>)
    {
        if signal_id.ty == SignalType::Item {
            let mut matched = false;

            for filter in &self.request_filters {
                if filter.name == signal_id.name {
                    matched = true;
                    break;
                }
            }

            if matched {
                f(&mut self.request_filters)
            }
        }
    }
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Icon {
    index: usize,
    signal: SignalID,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all="kebab-case")]
pub struct Blueprint {
    pub item: String, // must be "blueprint"

    pub version: u64,

    #[serde(skip_serializing_if="Option::is_none")]
    pub label: Option<String>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub label_color: Option<Color>,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub entities: Vec<Entity>,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub tiles: Vec<serde_json::Value>,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub icons: Vec<Icon>,

    #[serde(default, skip_serializing_if="Vec::is_empty")]
    pub schedules: Vec<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub snap_to_grid: Option<serde_json::Value>,

    #[serde(skip_serializing_if="Option::is_none")]
    pub absolute_snapping: Option<serde_json::Value>,
}

impl FromStr for Blueprint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::decode(s)
    }
}

impl Blueprint {
    pub fn decode(s: &str) -> Result<Self, Error> {
        let s = s.trim();

        if !s.starts_with('0') {
            return Err(Error::InvalidBlueprintString(s.to_owned()))
        }

        let compressed_data = base64::decode(&s[1..])?;
        let mut decoder = zlib::Decoder::new(Cursor::new(compressed_data)).unwrap();
        let envelope: BlueprintEnvelope = serde_json::from_reader(&mut decoder)?;

        Ok(envelope.blueprint)
    }

    pub fn encode(self) -> Result<String, Error> {
        let mut encoder = zlib::Encoder::new(Vec::new()).unwrap();

        serde_json::to_writer(&mut encoder, &BlueprintEnvelope { blueprint: self })?;
        let compressed_data = encoder.finish().into_result()?;

        let config = base64::Config::new(base64::CharacterSet::Standard, true);
        let mut buf = "0".to_owned();
        base64::encode_config_buf(&compressed_data, config, &mut buf);

        Ok(buf)
    }

    pub fn bill(&self) -> Vec<Signal> {
        let mut bill = HashMap::new();

        self.entities
            .iter()
            .for_each(|entity| {
                if let Some(count) = bill.get_mut(&entity.name) {
                    *count += 1;
                }
                else {
                    bill.insert(entity.name.clone(), 1);
                }
            });

        // TODO: Modules, trains, etc.

        bill.into_iter()
            .map(|(name, count )| Signal::new_item(name, count))
            .collect()
    }
}