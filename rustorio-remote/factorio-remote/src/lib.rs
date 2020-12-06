pub mod request;
pub mod error;

use std::{
    collections::BTreeMap,
    convert::TryInto,
    sync::Arc,
    env
};

use rcon::Connection;
use tokio::{
    net::ToSocketAddrs,
    sync::RwLock,
};
use log::Level;

use factorio_data::types::{ChannelId, Color, SignalID, Signal, TrainSchedule, UnitNumber, RadarData, Position, Screenshot};
pub use factorio_data::types;

use crate::{
    error::Error,
    request::{
        Request, PrintRequest, GetAvailableSignalsRequest, SendSignalsRequest, ReceiveSignalsRequest,
        ImportBlueprintRequest, ImportBlueprintResult, SetTrainScheduleRequest,
        GetTrainScheduleRequest, GetRadarRequest, PacketCodec, PingRequest, GetRadarScreenshotsRequest,
    },
};


pub trait LogHandler {
    fn on_log_line(&mut self, line: &str) -> Result<(), Error>;
}


#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LoggingLogHandler {
    level: Level,
}

impl LoggingLogHandler {
    pub fn new(level: Level) -> Self {
        Self { level }
    }
}

impl LogHandler for LoggingLogHandler {
    fn on_log_line(&mut self, line: &str) -> Result<(), Error> {
        log::log!(self.level, "{}", line);
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NullLogHandler;

impl LogHandler for NullLogHandler {
    fn on_log_line(&mut self, _line: &str) -> Result<(), Error> {
        Ok(())
    }
}

struct Inner {
    connection: Connection,
    packet_codec: PacketCodec,
    //log_handler: Box<dyn LogHandler + Send>,
}


pub struct FactorioRemote {
    inner: Arc<RwLock<Inner>>,
}

impl Clone for FactorioRemote {
    fn clone(&self) -> Self {
        Self { inner: Arc::clone(&self.inner) }
    }
}

impl FactorioRemote {
    pub async fn connect_env() -> Result<Self, Error> {
        let address = env::var("RCON_ADDRESS")?;
        let password = env::var("RCON_PASSWORD")?;
        let mod_name = env::var("RCON_MOD_NAME")?;

        Self::connect(address, &password, &mod_name).await
    }

    pub async fn connect<A: ToSocketAddrs>(address: A, password: &str, mod_name: &str) -> Result<Self, Error> {
        let connection = Connection::builder()
            .enable_factorio_quirks(true)
            .connect(address, password).await?;

        let remote = Self {
            inner: Arc::new(RwLock::new(Inner {
                connection,
                //mod_name: mod_name.to_owned(),
                //log_handler: Box::new(LoggingLogHandler { level: Level::Debug }),
                packet_codec: PacketCodec::new(mod_name)
            }))
        };

        let pong = remote.ping(420).await?;
        if  pong != 420 {
            return Err(Error::IncorrectPong {
                expected: 420,
                got: pong,
            })
        }

        Ok(remote)
    }

    pub async fn send_request<R: Request>(&self, request: &R) -> Result<R::Response, Error> {
        let mut inner = self.inner.write().await;

        let command = inner.packet_codec.encode_request_to_mod(request)?;
        let response_data = inner.connection.cmd(&command).await?;

        // TODO: Use log handler
        //let _ = &self.inner.log_handler;

        let response = inner.packet_codec.decode_response_from_mod::<R>(&response_data)?;
        Ok(response)
    }

    pub async fn print(&self, message: &str, color: Option<Color>) -> Result<(), Error> {
        self.send_request(&PrintRequest { message, color }).await
    }

    pub async fn get_available_signals(&self) -> Result<Vec<SignalID>, Error> {
        self.send_request(&GetAvailableSignalsRequest {}).await
    }

    pub async fn send_signals(&self, channel: ChannelId, signals: &[Signal]) -> Result<(), Error> {
        self.send_request(&SendSignalsRequest { channel, signals }).await
    }

    pub async fn receive_signals(&self, channel: ChannelId) -> Result<Vec<Signal>, Error> {
        Ok(self.send_request(&ReceiveSignalsRequest { channel }).await?.into())
    }

    pub async fn import_blueprint(&self, channel: ChannelId, blueprint_data: &str) -> Result<ImportBlueprintResult, Error> {
        Ok(self.send_request(&ImportBlueprintRequest { channel, blueprint_data } ).await?.try_into()?)
    }

    pub async fn get_train_schedule(&self, channel: ChannelId) -> Result<BTreeMap<UnitNumber, TrainSchedule>, Error> {
        self.send_request(&GetTrainScheduleRequest { channel }).await
    }

    pub async fn set_train_schedule(&self, channel: ChannelId, train_schedule: &TrainSchedule) -> Result<(), Error> {
        self.send_request(&SetTrainScheduleRequest { channel, train_schedule }).await
    }

    pub async fn get_radar(&self, channel: ChannelId) -> Result<Vec<RadarData>, Error> {
        self.send_request(&GetRadarRequest { channel }).await
    }

    pub async fn get_radar_screenshot(&self, channel: ChannelId) -> Result<Vec<Screenshot>, Error> {
        // 22.5, 27.5

        // TODO: Move to signals connected to radar?
        let zoom = 0.8;//3 chunks,
        let offset = Position::new(-9.5 + 64., 5.5 - 64.);
        Ok(self.send_request(&GetRadarScreenshotsRequest {
            channel,
            file_name: format!("radar-%d-%s-%s.png"),
            surface: "nauvis".to_string(),
            resolution: Position{ x: 4096., y: 4096. },
            offset,
            zoom,
        }).await?.into_vec())
    }

    pub async fn ping<T>(&self, t: T) -> Result<T, Error>
        where T: serde::ser::Serialize + for<'de> serde::de::Deserialize<'de> + std::fmt::Debug
    {
        Ok(self.send_request(&PingRequest { cookie: t }).await?)
    }
}

