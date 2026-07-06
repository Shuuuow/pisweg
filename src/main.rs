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
            "Search Entries".green(),
            "Exit".on_red(),
        ];

        let selection = Select::new()
            .with_prompt("\n Select Action")
            .items(&options)
            .interact()
            .unwrap();
        match selection {
            0 => modify::addie(),
            1 => modify::list_entries(),
            2 => modify::modify_entry(),
            3 => modify::del(),
            4 => modify::search_entries(),
            5 => break,
            _ => println!("Invalid selection"),
        }
    }
}

