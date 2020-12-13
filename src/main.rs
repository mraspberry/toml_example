use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs::read_to_string;
use toml;

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    player: PlayerInfo,
}

#[derive(Serialize, Deserialize, Debug)]
struct PlayerInfo {
    name: String,
    rank: u32,
}

fn main() {
    let contents = read_to_string("test.toml").unwrap();
    let config: Config = toml::from_str(&contents).unwrap();
    // dbg!(config);
    println!("{}", serde_json::to_string_pretty(&config).unwrap());
}
