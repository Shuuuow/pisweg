use crossterm::style::Stylize;
use dialoguer::{Input, Select};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Default)]
struct Entry {
    title: String,
    status: bool,
    id: u32,
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

    let mut entries = read_entries();
    let id_ = entries.iter().map(|e| e.id).max().unwrap_or(0) + 1;

    let entry = Entry {
        title: tilte,
        status: estat,
        id: id_,
    };

    entries.push(entry);
    write_entries(&entries);

    println!(
        "data inserted correctly, number of entries now {} ",
        entries.len()
    );
}

fn read_entries() -> Vec<Entry> {
    let filename = "data/store.yaml";
    if let Ok(content) = fs::read_to_string(filename) {
        if content.is_empty() {
            Vec::new()
        } else {
            serde_yaml::from_str(&content).unwrap_or_else(|_| Vec::new())
        }
    } else {
        Vec::new()
    }
}

fn write_entries(entries: &[Entry]) {
    let filename = "data/store.yaml";
    let yaml = serde_yaml::to_string(entries).expect("failed to serialize entries");
    fs::write(filename, yaml).expect("failed to write data");
}

fn status_str(status: bool) -> &'static str {
    if status { "Done" } else { "In Progress" }
}

pub fn list_entries() {
    let items = read_entries();
    for item in &items {
        println!("[{}] {} - {}", item.id, item.title, status_str(item.status));
    }
}

pub fn del() {
    let mut entries = read_entries();
    if entries.is_empty() {
        println!("No entries to delete.");
        return;
    }

    let selections: Vec<String> = entries
        .iter()
        .map(|e| format!("[{}] {} ({})", e.id, e.title, status_str(e.status)))
        .collect();

    let idx = Select::new()
        .with_prompt("Select entry to delete")
        .items(&selections)
        .interact()
        .unwrap();

    entries.remove(idx);
    write_entries(&entries);
    println!("Entry deleted.");
}

pub fn modify_entry() {
    let mut entries = read_entries();
    if entries.is_empty() {
        println!("No entries to modify.");
        return;
    }

    let selections: Vec<String> = entries
        .iter()
        .map(|e| format!("[{}] {} ({})", e.id, e.title, status_str(e.status)))
        .collect();

    let idx = Select::new()
        .with_prompt("Select entry to modify")
        .items(&selections)
        .interact()
        .unwrap();

    let new_status = Select::new()
        .with_prompt("Select new status")
        .item("Done".green().bold())
        .item("In Progress".red().bold())
        .interact()
        .unwrap()
        == 0;

    entries[idx].status = new_status;
    write_entries(&entries);
    println!("Entry status updated.");
}

pub fn search_entries() {
    let items = read_entries();
    if items.is_empty() {
        println!("No entries to search.");
        return;
    }

    let query = Input::<String>::new()
        .with_prompt("Enter search term (title or ID)")
        .interact_text()
        .unwrap();

    let query_lower = query.to_lowercase();
    let results: Vec<&Entry> = items
        .iter()
        .filter(|e| {
            e.title.to_lowercase().contains(&query_lower) || e.id.to_string().contains(&query)
        })
        .collect();

    if results.is_empty() {
        println!("No matches found.");
    } else {
        for e in &results {
            println!("[{}] {} - {}", e.id, e.title, status_str(e.status));
        }
    }
}
