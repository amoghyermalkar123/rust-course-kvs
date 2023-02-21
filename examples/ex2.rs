use serde::__private::de::IdentifierDeserializer;
use serde_json::*;
use std::error::Error;
use std::fs::{File, read_to_string};
use std::io::{Write, self};
use std::path::Path;
use std::str::from_utf8;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Move {
    direction: String,
}

fn main() {
    let d = Move{direction: String::from("DOWN")};
    let s = ron::to_string(&d).unwrap_or("NULL".to_owned());

    let vect = Vec::from(s);
    println!("{:?}", from_utf8(vect.as_ref()).unwrap())
}
