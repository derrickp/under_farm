use std::{fs, io::Error, str::FromStr};

use bevy::prelude::KeyCode;
use kdl::{KdlDocument, KdlError, KdlNode};

pub fn trim(value: String) -> String {
    value.replace(['\"', '\\'], "")
}

#[derive(Debug)]
pub struct InvalidPathError(pub Error);

#[derive(Debug)]
pub enum LoadError {
    InvalidPathError(InvalidPathError),
    KdlError(KdlError),
}

pub fn parse(path: &str) -> Result<Vec<KdlNode>, LoadError> {
    let content = fs::read_to_string(path);
    match content {
        Ok(it) => match KdlDocument::from_str(&it) {
            Ok(doc) => Ok(doc.nodes().to_vec()),
            Err(e) => Err(LoadError::KdlError(e)),
        },
        Err(e) => Err(LoadError::InvalidPathError(InvalidPathError(e))),
    }
}

#[derive(Debug)]
pub struct UnknownKeyCode;

#[derive(Debug)]
pub struct ReservedKeyCode;

#[derive(Debug)]
pub enum ParseKeyCodeError {
    UnknownKeyCode(UnknownKeyCode),
    ReservedKeyCode(ReservedKeyCode),
}

pub fn parse_key_code(code: &str) -> Result<KeyCode, ParseKeyCodeError> {
    match code {
        "a" => Ok(KeyCode::A),
        "h" => Ok(KeyCode::H),
        "k" => Ok(KeyCode::K),
        "m" => Ok(KeyCode::M),
        "p" => Ok(KeyCode::P),
        "t" => Ok(KeyCode::T),
        "s" => Ok(KeyCode::S),
        "i" => Err(ParseKeyCodeError::ReservedKeyCode(ReservedKeyCode)),
        _ => Err(ParseKeyCodeError::UnknownKeyCode(UnknownKeyCode)),
    }
}
