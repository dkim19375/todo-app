use std::fmt::Debug;
use std::io::Error;
use std::process;
use std::sync::OnceLock;

use console::style;
use dialoguer::{Confirm, Input, Select};
use dialoguer::theme::ColorfulTheme;

static THEME: OnceLock<ColorfulTheme> = OnceLock::new();

fn main() -> Result<(), Error> {
    THEME.set(ColorfulTheme::default()).ok().unwrap();
    println!("{}", style("===============    To-Do App    ===============").cyan().bold().bright());
    main_menu(vec![])
}

fn main_menu(mut items: Vec<ToDoItem>) -> Result<(), Error> {
    let options = vec![
        "View To-Do List".into(),
        "Add item".into(),
        "Remove item".into(),
        style("Exit").red().bold().to_string(),
    ];
    let option = Select::with_theme(THEME.get().unwrap())
        .items(&options)
        .default(0)
        .with_prompt("Please select an option")
        .interact()?;

    match option {
        0 => view_list(&mut items),
        1 => add_item_menu(&mut items),
        2 => remove_item_menu(&mut items),
        3 => exit(),
        _ => {
            panic!("Invalid option choice - should've been between 0 through 3");
        }
    };

    main_menu(items)
}

fn get_formatted_items(items: &[ToDoItem]) -> Vec<String> {
    items.iter().map(|x| {
        if x.completed { style(x.text.to_owned()).green().to_string() } else { x.text.to_owned() }
    }).collect::<Vec<String>>()
}

fn view_list(items: &mut Vec<ToDoItem>) {
    println!("{}", style("[9mTo-Do List Items:").magenta().bold());
    if items.is_empty() {
        println!("{}", style("Empty!").green());
        return;
    }
    let mut last_modified = 0_usize;
    loop {
        let mut options = get_formatted_items(items);
        options.push(style("Exit").red().bold().to_string());
        let option = Select::with_theme(THEME.get().unwrap())
            .items(&options)
            .default(last_modified)
            .interact()
            .unwrap();
        if option == items.len() {
            break;
        }
        last_modified = option;
        let existing = &items[option];
        let new = ToDoItem {
            text: existing.text.to_owned(),
            completed: !existing.completed,
        };
        items.remove(option);
        items.insert(option, new);
    }
}

fn add_item_menu(items: &mut Vec<ToDoItem>) {
    loop {
        let new_item: String = Input::<String>::with_theme(THEME.get().unwrap())
            .validate_with(|input: &String| {
                let existing = items.iter().find(|item| {
                    item.text.eq_ignore_ascii_case(input.to_owned().as_str())
                });
                if existing.is_some() {
                    Err(format!("That item ({}) already exists", existing.unwrap().text))
                } else {
                    Ok(())
                }
            })
            .with_prompt("Item to add")
            .interact_text()
            .unwrap()
            .trim()
            .to_string();
        if new_item.is_empty() {
            break;
        }
        items.push(ToDoItem { text: new_item, completed: false });
    }
}

fn remove_item_menu(items: &mut Vec<ToDoItem>) {
    let mut last_modified = 0_usize;
    loop {
        if items.is_empty() {
            println!("{}", style("Empty!").green());
            break;
        }
        let mut options = get_formatted_items(items);
        options.push(style("Cancel").red().bold().to_string());
        let option = Select::with_theme(THEME.get().unwrap())
            .items(&options)
            .default(last_modified.max(1) - 1)
            .interact()
            .unwrap();
        if option == items.len() {
            break;
        }
        last_modified = option;
        let removed = items.remove(option);
        println!("{} {}", style("Removed item:").yellow(), if removed.completed {
            style(removed.text).green()
        } else {
            style(removed.text).red()
        });
    }
}

fn exit() {
    let confirmed = Confirm::with_theme(THEME.get().unwrap())
        .default(false)
        .with_prompt("Are you sure you would like to exit?")
        .interact()
        .unwrap();
    if !confirmed {
        return;
    }
    println!("{}", style("Bye!").yellow().bold());
    process::exit(0);
}

#[derive(Debug)]
struct ToDoItem {
    text: String,
    completed: bool,
}