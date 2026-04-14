use crossterm::style::Stylize;
use dialoguer::{Input, Select};

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

    println!("{} {}", entry.title, entry.status);
}
