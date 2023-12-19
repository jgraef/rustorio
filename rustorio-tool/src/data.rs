use std::{
    collections::HashMap,
    sync::Arc,
};

use gloo_net::http::Request;
use rustorio_prototype::Prototypes;
use serde::Deserialize;

use crate::error::Error;

#[derive(Clone, Debug, Deserialize)]
pub struct GameData {
    pub prototypes: Prototypes,
    pub icons: HashMap<String, String>,
}

impl GameData {
    pub fn icon(&self, path: &str) -> Option<&str> {
        self.icons.get(path).map(|s| s.as_str())
    }
}

#[derive(Debug)]
struct AppDataInner {
    game_data: GameData,
}

#[derive(Clone, Debug, Default)]
pub struct AppData(Option<Arc<AppDataInner>>);

impl PartialEq for AppData {
    fn eq(&self, other: &Self) -> bool {
        match (&self.0, &other.0) {
            (Some(a), Some(b)) => Arc::ptr_eq(a, b),
            (None, None) => true,
            _ => false,
        }
    }
}

impl AppData {
    pub async fn fetch() -> Result<Self, Error> {
        let game_data = Request::get("/data/data.json")
            .send()
            .await?
            .json::<GameData>()
            .await?;
        Ok(Self(Some(Arc::new(AppDataInner { game_data }))))
    }

    pub fn is_loaded(&self) -> bool {
        self.0.is_some()
    }

    pub fn game_data(&self) -> &GameData {
        &self.0.as_ref().unwrap().game_data
    }
}
