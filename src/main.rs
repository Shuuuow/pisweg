use crossterm::style::Stylize;
use dialoguer::Select;

mod modify;

fn main() {
    let options = vec![
        "Add Entry".green(),
        "List Entries".blue(),
        "Modify Entry".yellow(),
        "Delete Entry".red(),
    ];

    let selection = Select::new()
        .with_prompt("Select Action")
        .items(&options)
        .interact()
        .unwrap();
    match selection {
        0 => add(),
        1 => placeholder(),
        2 => placeholder(),
        3 => placeholder(),
        _ => println!("Invalid selection"),
    }
}

fn placeholder() {
    print!("PACEHOLDER")
}

fn add() {
    modify::addie();
}
