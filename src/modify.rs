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
    let etilte = Input::new()
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
        title: etilte,
        status: estat,
    };

    senddata(entry);
}

fn senddata(entry: Entry) -> Result<(), Box<dyn std::error::Error>> {
    /*let json = serde_json::to_string_pretty(&entry).unwrap();
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("data/store.json")
        .unwrap();
    writeln!(file, "{}", json).unwrap();*/

    let yaml_output = serde_yaml::to_string(&entry)?;
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open("data/store.yaml")
        .unwrap();
    writeln!(file, "{}", yaml_output).unwrap();
    Ok(())
}
