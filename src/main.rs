use std::io;

mod todo_list;

fn main() {
    let mut todos = todo_list::TodoList::new();
    println!("Welcome to Terry's Rust Todo List.");
    loop {
        print_menu();
        let mut option = String::new();
        io::stdin().read_line(&mut option).unwrap();
        let answer = &option.trim().parse().unwrap();
        match answer {
            1 => todos.show(),
            2 => add_todo_handler(&mut todos),
            3 => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid Input. Please enter again."),
        }
    }
}

fn print_menu() {
    println!("");
    println!("What do you want to do?");
    println!("1 - Shows all todos");
    println!("2 - Add a new todo");
    println!("3 - Quit");
    println!("");
}

fn add_todo_handler(todos: &mut todo_list::TodoList) {
    println!("");
    let mut content = String::new();
    println!("Please enter the content of your new todo.");
    io::stdin().read_line(&mut content).unwrap();
    todos.add(content)
}
