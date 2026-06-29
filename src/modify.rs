use crossterm::style::Stylize;
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Entry {
    title: String,
    status: bool,
    id: [u32; 2],
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

    let id_: [u32; 2] = [if estat { 0 } else { 1 }, rand::random_range(0..9999)];

    let entry = Entry {
        title: tilte,
        status: estat,
        id: id_,
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

    println!(
        "data inserted correctly, number of entries now {} ",
        udata.len()
    );
}

pub fn list_entries() {
    let files = fs::read_to_string("data/store.yaml").expect("Can't read file");
    let items: Vec<Entry> = serde_yaml::from_str(&files).expect("Invalid YAML");
    for item in items {
        println!("{}: {}", item.title, item.status);
    }
}

#[allow(dead_code)]
pub fn del() {}
