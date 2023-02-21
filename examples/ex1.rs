use serde_json::*;
use std::error::Error;
use std::fs::{File, read_to_string};
use std::io::{Write, self};
use std::path::Path;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Move {
    direction: String,
}

fn main() -> Result<()> {
    let d = Move{direction: String::from("DOWN")};
    let s = serde_json::to_string(&d).unwrap_or("NULL".to_owned());

    let path = Path::new("hello.txt");
    let mut file = File::create(&path).unwrap(); 
    
    file.write_all(s.as_bytes());

    let content = read_to_string(path).unwrap();

    let b: Move = serde_json::from_str(content.as_str())?;
    println!("{:?}", b);
    Ok(())
}
