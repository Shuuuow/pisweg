use crossterm::style::Stylize;
use dialoguer::Select;

mod modify;

fn main() {
    loop {
        let options = vec![
            "Add Entry".green(),
            "List Entries".green(),
            "Modify Entry".green(),
            "Delete Entry".green(),
            "Exit".on_red(),
        ];

        let selection = Select::new()
            .with_prompt("\n Select Action")
            .items(&options)
            .interact()
            .unwrap();
        match selection {
            0 => add(),
            1 => placeholder(),
            2 => placeholder(),
            3 => placeholder(),
            4 => break,
            _ => println!("Invalid selection"),
        }
    }
}

fn placeholder() {
    print!("PACEHOLDER")
}

fn add() {
    modify::addie();
}
