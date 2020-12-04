#![allow(dead_code)]
#![feature(associated_type_defaults)]

pub mod config;
pub mod error;
pub mod lua_api;
pub mod lua_utils;
pub mod mod_loader;
pub mod version;

// TODO: Split up types, such that it can be used without prototypes
//#[cfg(feature="prototypes")]
pub mod types;

//#[cfg(feature="prototypes")]
pub mod prototypes;

#[cfg(feature="blueprint")]
pub mod blueprint;

pub use rustorio_data as data;
