use crossterm::style::Stylize;
use dialoguer::Select;

mod modify;

fn main() {
    let options = vec![
        "Add Entry".green(),
        "Modify Entry".yellow(),
        "Delete Entry".red(),
    ];

    let selection = Select::new()
        .with_prompt("")
        .items(&options)
        .interact()
        .unwrap();
    match selection {
        0 => placeholder(),
        1 => placeholder(),
        2 => placeholder(),
        _ => println!("Invalid selection"),
    }
}

fn placeholder() {
    print!("PACEHOLDER")
}
