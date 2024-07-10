use rocket::serde::json::Json;
use std::io::{BufReader, Write};
use serde::de::DeserializeOwned;
use serde::Serialize;
use crate::error::error::{ApplicationError, Error};
use crate::error::error::FileError::*;
use crate::error::error::JsonError::*;

pub fn rewrite_single_json<T: Serialize + Clone>(data: &Json<T>, path: &str) -> Result<(), Error> {
    let mut file = match std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
    {
        Ok(file) => file,
        Err(err) => return Err(Error::new(ApplicationError::FileError(FileOpenError), err.to_string()))
    };

    let json_string = match serde_json::to_string(&data.clone().into_inner()) {
        Ok(str) => str,
        Err(err) => return Err(Error::new(ApplicationError::JsonError(JsonSerializeError), err.to_string()))
    };

    match writeln!(file, "{}", json_string) {
        Ok(_) => Ok(()),
        Err(err) => return Err(Error::new(ApplicationError::FileError(FileWriteError), err.to_string()))
    }
}

pub fn read_single_json<T: DeserializeOwned>(path: &str) -> T {
    let file: std::fs::File = std::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Can't open json file");

    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader).expect("Can't deserialize json data");

    data
}

pub fn rewrite_json<T: Serialize + Clone>(data: &Json<Vec<T>>, path: &str) {
    let mut file: std::fs::File = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)
        .expect("Can't open json file");

    let json_string = serde_json::to_string(&data.clone().into_inner())
        .expect("Failed to serialize content");

    writeln!(file, "{}", json_string)
        .expect("Failed to write to file");
}

pub fn read_json<T: DeserializeOwned>(path: &str) -> Vec<T> {
    let file: std::fs::File = std::fs::OpenOptions::new()
        .read(true)
        .open(path)
        .expect("Can't open json file");

    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader).expect("Can't deserialize json data");

    data
}