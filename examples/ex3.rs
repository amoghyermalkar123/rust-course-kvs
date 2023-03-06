use std::fs::{read_to_string, File};
use std::io::{self, BufReader, Read, Write};
use std::path::Path;
use std::str::from_utf8;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Move {
    id: u64,
    direction: String,
    other: u64,
    elsee: bool,
}

fn main() {
    let path = Path::new("some.db");
    let mut file = File::create(&path).unwrap();

    for i in 0..10 {
        let m = Move {
            id: i,
            direction: String::from("UP"),
            other: i + 10,
            elsee: false,
        };
        let moveval = bson::to_vec(&m).unwrap();
        let buf: &[u8] = moveval.as_ref();
        println!("buf {:?}", buf.len());
        file.write_all(buf);
    }
    println!("writing done");

    const BUFFER_LEN: usize = 58;
    let mut reader = File::open(path).unwrap();

    for i in 0..10 {
        let mut buffer = [0u8; BUFFER_LEN];
        let read_count = reader.read(&mut buffer).unwrap();

        if read_count != BUFFER_LEN {
            return;
        }
        let deser: Move = bson::from_slice(&buffer).unwrap();
        println!("{:?}", deser);
    }

    let mut all = Vec::new();

    for i in 0..10 {
        let m = Move {
            id: i,
            direction: String::from("UP"),
            other: i + 10,
            elsee: false,
        };
        let moveval = bson::to_vec(&m).unwrap();
        let buf = moveval.as_ref();
        all.write(buf);
    }

    let mut pointer: usize = 0;
    let mut offset = BUFFER_LEN;

    loop {
        if offset > all.len() {
            break
        }
        let buffer = &all[pointer..offset];
        let deser: Move = bson::from_slice(&buffer).unwrap();
        println!("{:?}", deser);
        pointer = offset;
        offset = offset + BUFFER_LEN;
    }
}
