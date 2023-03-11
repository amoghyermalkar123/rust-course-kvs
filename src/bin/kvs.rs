#![allow(dead_code)]
use clap::{Arg, Command};

fn main() {
    let about = "use --help for more info";

    let m = Command::new("kvs")
        .author("Me, me@mail.com")
        .version(env!("CARGO_PKG_VERSION"))
        .about(about)
        .subcommand(
            Command::new("set")
                .arg(Arg::new("key"))
                .arg(Arg::new("value")),
        )
        .subcommand(Command::new("get").arg(Arg::new("key")))
        .subcommand(Command::new("rm").arg(Arg::new("key")))
        .get_matches();

    let mut kvs = kvs::KvStore::new().expect("db creation failed");

    match m.subcommand() {
        Some(("set", cmd)) => {
            if let Some(key) = cmd.get_one::<String>("key") {
                if let Some(val) = cmd.get_one::<String>("value") {
                    match kvs.set(key.to_owned(), val.to_owned()) {
                        Ok(_) => {
                            println!("record inserted successfully")
                        },
                        Err(e) => {
                            println!("encountered following error: {}", e.root_cause())
                        }
                    }
                } else {
                    println!("{}", about)
                }
            }
        }
        Some(("get", cmd)) => {
            if let Some(val) = cmd.get_one::<String>("key") {
                if let Some(val) = kvs.get(val.to_owned()) {
                    println!("value: {}", val);
                } else {
                    println!("empty")
                }
            }
        }
        Some(("rm", cmd)) => {
            if let Some(key) = cmd.get_one::<String>("key") {
                if let Ok(_) = kvs.remove(key.to_owned()) {
                    println!("key deleted");
                } else {
                    println!("failed to delete key")
                }
            }
        }
        _ => {
            println!("{}", about)
        }
    }
}
