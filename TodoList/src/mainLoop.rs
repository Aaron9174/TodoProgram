
use std::io::{self, Write};
use std::convert::TryFrom;
use std::result::Result;

use crate::taskData;

#[repr(u8)]
#[derive(PartialEq)]
enum MainMenuOptions {
    Unknown = 0,
    DisplayTasks = 1,
    AddTask = 2,
    CompleteTask = 3,
    DeleteTask = 4,
    Exit = 5,
}

#[repr(u8)]
#[derive(PartialEq)]
enum DisplayTasksMenuOptions
{
    Unknown = 0,
}

pub struct InvalidMainMenuOption(u8);

// Implement the TryFrom trait for your enum
impl TryFrom<u8> for MainMenuOptions {
    type Error = InvalidMainMenuOption;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MainMenuOptions::Unknown),
            1 => Ok(MainMenuOptions::DisplayTasks),
            2 => Ok(MainMenuOptions::AddTask),
            3 => Ok(MainMenuOptions::CompleteTask),
            4 => Ok(MainMenuOptions::DeleteTask),
            5 => Ok(MainMenuOptions::Exit),
            _ => Err(InvalidMainMenuOption(value)),
        }
    }
}

pub fn main_loop() {
    let mut selected_option = MainMenuOptions::Unknown;

    while selected_option != MainMenuOptions::Exit
    {
        display_main_menu();
        display_user_input();
        let res = get_user_input(); 
        selected_option = handle_main_menu_response(res);
    }
}

fn display_user_input() {
    print!("USER_INPUT> ");
    io::stdout().flush();
}

fn display_main_menu() {
    let menu_str = r#"
    #######################
    #   Todo Task Flask   #
    #######################
    # 1) Display tasks    #
    # 2) Add a task       #
    # 3) Complete a task  #
    # 4) Delete a task    #
    # 5) Exit             #
    #######################
    "#;
    println!("{}", menu_str);
}

fn get_user_input() -> u8 {
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    return input.trim().parse().unwrap();
}

fn handle_main_menu_response(res: u8) -> MainMenuOptions {
   match MainMenuOptions::try_from(res) {
        Ok(MainMenuOptions::DisplayTasks) => {
            display_task_submenu();
            return MainMenuOptions::DisplayTasks;
        }
        Ok(MainMenuOptions::AddTask) => {
            display_add_task_submenu();
            return MainMenuOptions::AddTask;
        }
        Ok(MainMenuOptions::Exit) => { return MainMenuOptions::Exit; }
        _ => { println!("Unrecognized response, please reenter.\n"); display_main_menu(); return MainMenuOptions::Unknown; }
   } 
}

fn display_task_submenu() {
    let submenu = r#"
    #############################
    #      Display Options      #
    #############################
    # 1) Show all tasks         #
    # 2) Show completed tasks   #
    # 3) Show uncompleted tasks #
    # 4) Back                   #
    #############################
    "#;

    println!("{}", submenu);
}

fn handle_display_task_submenu(res: u8)
{
    // match res {
    //     1 => 
    // }
}

fn display_tasks() {
    let submenu = r#"
    ###########################
    #      Display Tasks      #
    ###########################
    "#;

    println!("{}", submenu);
}

fn display_add_task_submenu() {
    let submenu = r#"
    ###########################
    #     Add Task Options    #
    ###########################
    # 1) Create a new task    #
    # 2) Back                 #
    ###########################
    "#;
    println!("{}", submenu);
}

fn create_new_task() {
    println!("Enter the task name: ");
    let mut name = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    println!("Enter the task description: ");
    let mut descr = String::new();
    std::io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let task = Task {
        id: get_todo_list_size(),
        name: name,
        description: descr,
        completed: false,
    };

    add_task(task);
}

