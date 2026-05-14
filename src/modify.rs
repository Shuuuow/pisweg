use std::fs;
use crossterm::style::Stylize;
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Entry {
    title: String,
    status: bool,
}

pub fn addie() {
    let tilte = Input::new()
        .with_prompt("Enter title")
        .interact_text()
        .unwrap();

    let estat = Select::new()
        .with_prompt("Enter status")
        .item("Done".green().bold())
        .item("In Progress".red().bold())
        .interact()
        .unwrap()
        == 0;

    let entry = Entry {
        title: tilte,
        status: estat,
    };

    insert_data(entry);
}

fn insert_data(entry: Entry) {
    let filename = "data/store.yaml";

    let mut udata: Vec<Entry> = if let Ok(content) = fs::read_to_string(filename) {
        if content.is_empty() {
            Vec::new()
        } else {
            serde_yaml::from_str(&content).unwrap_or_else(|_| Vec::new())
        }
    } else {
        Vec::new()
    };

    udata.push(entry);

    let updated_yaml = serde_yaml::to_string(&udata).expect("failed to serialize entry");
    fs::write(filename, updated_yaml).expect("Data insertion failed.");

    println!("data inseted correctly, number of enteries now {} ", udata.len());
}