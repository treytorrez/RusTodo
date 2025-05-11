use crossterm;
use inquire::MultiSelect;
use std::{collections::HashMap, fmt, io};
use text_io::read;

use crossterm::{
    execute,
    terminal::{Clear, ClearType},
};
use std::io::{Write, stdout};

fn clear_screen() {
    // Clears entire screen
    execute!(stdout(), Clear(ClearType::All)).unwrap();
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Task {
    name: String,
    done: bool,
}

impl Task {
    fn print(&self) {
        if self.done {
            print!("[x]    ");
        } else {
            print!("[ ]    ");
        }
        println!("{}", self.name);
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn add_todo() -> Task {
    let name = readln();
    Task { name, done: false }
}

fn readln() -> String {
    print!("Enter your text: ");
    stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin()
        .read_line(&mut buf)
        .expect("Failed to read line");
    buf.trim_end().to_string()
}

fn main() {
    println!("Welcome to RusTodo!");
    let mut tasks: HashMap<String, Task> = HashMap::new();
    let mut quit = false;
    // Main menu options
    let main_menu = vec!["View TODOs", "Add TODO", "Quit"];

    while !quit {
        clear_screen();

        // Prompt for main action
        let ans = inquire::Select::new("Select an option", main_menu.clone())
            .prompt()
            .unwrap();

        match ans {
            "View TODOs" => {
                if tasks.is_empty() {
                    let _ = inquire::Select::new("No TODOs yet—press Enter to return", vec!["OK"]) 
                        .prompt();
                } else {
                    // Build options for MultiSelect
                    let mut options: Vec<Task> = tasks.values().cloned().collect();
                    // Determine which are checked by default
                    let defaults: Vec<usize> = options.iter()
                        .enumerate()
                        .filter_map(|(i, t)| if t.done { Some(i) } else { None })
                        .collect();

                    // Show interactive checkbox menu
                    let selected: Vec<Task> = MultiSelect::new("Toggle TODOs", options)
                        .with_default(&defaults)
                        .prompt()
                        .unwrap();

                    // Update the underlying HashMap
                    let done_names: std::collections::HashSet<String> =
                        selected.into_iter().map(|t| t.name).collect();

                    for task in tasks.values_mut() {
                        task.done = done_names.contains(&task.name);
                    }
                }
            }
            "Add TODO" => {
                let new = add_todo();
                tasks.insert(new.name.clone(), new);
            }
            "Quit" => quit = true,
            _ => unreachable!("Invalid choice!"),
        }
    }
}
