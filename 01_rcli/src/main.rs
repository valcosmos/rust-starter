use anyhow::{Ok, Result};
use serde::Deserialize;
use std::fs::File;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Player {
    #[serde(rename="ID")]
    id: u8,
    #[serde(rename="Name")]
    name: String,
    #[serde(rename="Age")]
    age: u8,
    #[serde(rename="Country")]
    country: String,
}

fn main() -> Result<()> {
    // let content = read_file("assets/demo.csv")?;
    let file = File::open("assets/demo.csv")?;
    // let mut reader = csv::Reader::from_reader(file);
    let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(file);
    for result in reader.deserialize() {
        let player: Player = result?;
        // println!("{:?}", player.ID);
        println!("Id:{}, Name: {}, Age: {}, Country: {}", player.id, player.name, player.age, player.country)
    }
    Ok(())
}

// fn read_file(file_path: &str) -> Result<String> {
// //    let content =  fs::read_to_string(file_path).expect("Error reading file");
//    let content =  fs::read_to_string(file_path)?;
//    Ok(content)
// }
