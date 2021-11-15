use std::{fs, io::Error};

use kdl::{KdlError, KdlNode};

pub fn trim(value: String) -> String {
    value.replace("\"", "").replace("\\", "")
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
        Ok(it) => match kdl::parse_document(it) {
            Ok(nodes) => Ok(nodes),
            Err(e) => Err(LoadError::KdlError(e)),
        },
        Err(e) => Err(LoadError::InvalidPathError(InvalidPathError(e))),
    }
}
