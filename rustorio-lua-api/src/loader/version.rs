use std::{
    fmt::{
        Display,
        Formatter,
        Result as FormatResult,
    },
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

#[cfg(test)]
mod tests {
    use super::Version;

    
}
