use std::{
    marker::PhantomData,
    convert::TryFrom,
    collections::BTreeMap,
    fmt,
};

use serde::{Serialize, Deserialize};

use factorio_data::types::{ChannelId, Color, SignalID, Signal, TrainSchedule, UnitNumber, RadarData, Position, Screenshot};

use crate::error::{Error, LuaError};


/// Wrapper of `Vec<T>` to customize parsing of lists that are serialized to Json from Lua. Lua only has tables, so it
/// can't distinguish between an empty table and empty list. Factorio's `game.table_to_json` serialiizes an empty list
/// as an empty dictionary `{}`, so we need to accept that for an empty list.
pub struct LuaList<T>(Vec<T>);

impl<T> Default for LuaList<T> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T> LuaList<T> {
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
}

impl<T> From<LuaList<T>> for Vec<T> {
    fn from(list: LuaList<T>) -> Self {
        list.0
    }
}

impl<'de, T> Deserialize<'de> for LuaList<T>
    where T: serde::de::Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<LuaList<T>, D::Error>
        where
            D: serde::de::Deserializer<'de>,
    {
        deserializer.deserialize_any(LuaListVisitor { marker: PhantomData })
    }
}

struct LuaListVisitor<T> {
    marker: PhantomData<T>,
}

impl<'de, T> serde::de::Visitor<'de> for LuaListVisitor<T>
    where T: serde::de::Deserialize<'de>
{
    type Value = LuaList<T>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a list or empty dict")
    }

    fn visit_seq<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where A: serde::de::SeqAccess<'de>,
    {
        let mut list = vec![];

        while let Some(value) = access.next_element()? {
            list.push(value)
        }

        Ok(LuaList(list))
    }

    fn visit_map<A>(self, mut access: A) -> Result<Self::Value, A::Error>
        where A: serde::de::MapAccess<'de>,
    {
        let opt: Option<((), ())> = access.next_entry()?;
        if opt.is_some() {
            use serde::de::Error;
            Err(A::Error::custom("Expecting list, but got non-empty dict."))
        }
        else {
            Ok(LuaList(vec![]))
        }
    }
}


pub struct PacketCodec {
    pub mod_name: String,
    pub request_prefix: String,
    pub response_prefix: String,
    pub heartbeat: String,
}

impl PacketCodec {
    pub fn new(mod_name: &str) -> Self {
        let request_prefix= format!("janosch-remote-{}-resp:", mod_name);
        let response_prefix= format!("janosch-remote-{}-resp:", mod_name);
        let heartbeat = format!("/janosch-remote-{}-hb", mod_name);

        return Self {
            mod_name: mod_name.to_owned(),
            request_prefix,
            response_prefix,
            heartbeat,
        }
    }

    pub fn encode_request_to_mod<R>(&self, request: &R) -> Result<String, Error>
        where R: Request
    {
        let request_data = RequestData {
            request_type: R::REQUEST_TYPE,
            body: request,
        };

        log::trace!("Sending request: {:?}", request);

        let encoded = format!("/janosch-remote-{}-req {}", self.mod_name, &serde_json::to_string(&request_data)?);

        Ok(encoded)
    }

    pub fn decode_request_from_mod<R>(&self, _encoded: &str) -> Result<R, Error>
        where R: Request,
    {
        /*let request_data = encoded.splitn(2, ':').last()
            .ok_or_else(|| Error::MalformedPacket(encoded.to_owned()))?;

        let request: R = serde_json::from_str(request_data)?;

        Ok(request)*/
        unimplemented!()
    }

    pub fn encode_response_to_mod<Q>(&self, _response_data: &ResponseData<Q>)
        where Q: for<'de> serde::de::Deserialize<'de>,
    {
        /*let mut encoded = format!("/janosch-remote-{}-resp {}", self.mod_name, &serde_json::to_string(&response_data)?);

        Ok(encoded)*/
        unimplemented!()
    }

    pub fn decode_response_from_mod<R>(&self, encoded: &str) -> Result<R::Response, Error>
        where R: Request
    {
        for line in encoded.lines() {
            if line.starts_with(&self.response_prefix) {
                log::trace!("Response: {}", line);

                let response_data = line.splitn(2, ':').last()
                    .ok_or_else(|| Error::MalformedPacket(encoded.to_owned()))?;
                log::trace!("response_data: {}", response_data);
                let response_data: ResponseData<R::Response> = serde_json::from_str(response_data)?;

                if response_data.status == "success" {
                    if let Some(value) = response_data.value {
                        return Ok(value);
                    } else {
                        return R::on_missing_value();
                    }
                }
                else if response_data.status == "error" {
                    return Err(response_data.error.unwrap_or_default().into());
                }
                else {
                    return Err(Error::MalformedPacket(line.to_owned()));
                }
            } else {
                log::debug!("RCON: {}", line);
                //log_handler.on_log_line(line)?;
            }
        }

        Err(Error::MissingResponse)
    }
}


/// A request that can be send to the remote-io mod.
pub trait Request: serde::ser::Serialize + std::fmt::Debug {
    /// The expected response type.
    type Response: for<'de> serde::de::Deserialize<'de>;

    /// The name of the request type.
    const REQUEST_TYPE: &'static str;

    /// What to do when the request was successful, but no return value was send in the response.
    ///
    /// The default behavior is to return an error. But since in Lua having a `nil` value is equivalent to having
    /// the field not set in a table, if a request returns `nil`, the `value` will be missing, even if the response
    /// was successful. In that case the `Request` implementation should override this by returning the default value.
    fn on_missing_value() -> Result<Self::Response, Error> {
        Err(Error::MissingResponse)
    }
}


#[derive(Clone, Debug, Serialize)]
pub struct RequestData<'t, 'r, R>
    where R: serde::ser::Serialize
{
    request_type: &'t str,
    #[serde(flatten)]
    body: &'r R
}

#[derive(Clone, Debug, Deserialize)]
pub struct ResponseData<Q>
{
    status: String,
    value: Option<Q>,
    error: Option<LuaError>,
    request_id: Option<i32>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ListRequestTypes;

impl Request for ListRequestTypes {
    type Response = LuaList<String>;
    const REQUEST_TYPE: &'static str = "list_request_types";
}

#[derive(Clone, Debug, Serialize)]
pub struct PingRequest<T> {
    pub cookie: T,
}

impl<T> Request for PingRequest<T>
    where T: serde::ser::Serialize + for<'de> serde::de::Deserialize<'de> + std::fmt::Debug,
{
    type Response = T;
    const REQUEST_TYPE: &'static str = "ping";
}

#[derive(Clone, Debug, Serialize)]
pub struct PrintRequest<'m> {
    pub message: &'m str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
}

impl<'m> Request for PrintRequest<'m> {
    type Response = ();
    const REQUEST_TYPE: &'static str = "print";

    fn on_missing_value() -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct RunCodeRequest<'c> {
    pub code: &'c str,
}

impl<'c> Request for RunCodeRequest<'c> {
    type Response = serde_json::Value;
    const REQUEST_TYPE: &'static str = "run_code";
}

#[derive(Clone, Debug, Serialize)]
pub struct GetAvailableSignalsRequest {}

impl Request for GetAvailableSignalsRequest {
    type Response = Vec<SignalID>;
    const REQUEST_TYPE: &'static str = "get_available_signals";
}

#[derive(Clone, Debug, Serialize)]
pub struct SendSignalsRequest<'s> {
    pub channel: ChannelId,
    pub signals: &'s[Signal],
}

impl<'s> Request for SendSignalsRequest<'s> {
    type Response = ();
    const REQUEST_TYPE: &'static str = "send_signals";

    fn on_missing_value() -> Result<(), Error> {
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ReceiveSignalsRequest {
    pub channel: ChannelId,
}

impl Request for ReceiveSignalsRequest {
    type Response = LuaList<Signal>;
    const REQUEST_TYPE: &'static str = "receive_signals";
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ImportBlueprintResult {
    Success,
    SuccessWithErrors,
    Failed
}

impl TryFrom<i32> for ImportBlueprintResult {
    type Error = Error;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ImportBlueprintResult::Success),
            -1 => Ok(ImportBlueprintResult::SuccessWithErrors),
            1 => Ok(ImportBlueprintResult::Failed),
            _ => Err(Error::InvalidImportBlueprintResult(value)),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ImportBlueprintRequest<'b> {
    pub channel: ChannelId,
    pub blueprint_data: &'b str,
}

impl<'b> Request for ImportBlueprintRequest<'b> {
    type Response = i32;
    const REQUEST_TYPE: &'static str = "import_blueprint";
}

#[derive(Clone, Debug, Serialize)]
pub struct GlobalRequest {}

impl Request for GlobalRequest {
    type Response = serde_json::Value;
    const REQUEST_TYPE: &'static str = "global";

    fn on_missing_value() -> Result<serde_json::Value, Error> {
        Ok(serde_json::Value::Object(Default::default()))
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GetTrainScheduleRequest {
    pub channel: ChannelId,
}

impl Request for GetTrainScheduleRequest {
    type Response = BTreeMap<UnitNumber, TrainSchedule>;
    const REQUEST_TYPE: &'static str = "get_train_schedule";

    fn on_missing_value() -> Result<BTreeMap<UnitNumber, TrainSchedule>, Error> {
        Ok(Default::default())
    }
}


#[derive(Clone, Debug, Serialize)]
pub struct SetTrainScheduleRequest<'t> {
    pub channel: ChannelId,
    pub train_schedule: &'t TrainSchedule,
}

impl<'t> Request for SetTrainScheduleRequest<'t> {
    type Response = ();
    const REQUEST_TYPE: &'static str = "set_train_schedule";

    fn on_missing_value() -> Result<(), Error> {
        Ok(())
    }
}


#[derive(Clone, Debug, Serialize)]
pub struct GetRadarRequest {
    pub channel: ChannelId,
}

impl Request for GetRadarRequest {
    type Response = Vec<RadarData>;
    const REQUEST_TYPE: &'static str = "get_radar";

    fn on_missing_value() -> Result<Self::Response, Error> {
        Ok(Default::default())
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GetRadarScreenshotsRequest {
    pub channel: ChannelId,
    pub file_name: String,
    pub surface: String,
    pub resolution: Position,
    pub zoom: f64,
    pub offset: Position,
}

impl Request for GetRadarScreenshotsRequest {
    type Response = LuaList<Screenshot>;
    const REQUEST_TYPE: &'static str = "get_radar_screenshot";

    fn on_missing_value() -> Result<Self::Response, Error> {
        Ok(Default::default())
    }
}
