mod todo;

pub struct TodoList {
    todos: Vec<todo::Todo>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { todos: vec![] }
    }

    pub fn add(&mut self, content: String) {
        let new_todo = todo::Todo::new(content);
        self.todos.push(new_todo)
    }

    pub fn show(&self) {
        if self.todos.len() == 0 {
            println!("There is no todo in the list.")
        } else {
            for (i, todo) in self.todos.iter().enumerate() {
                println!("{}: {}", i + 1, todo.content)
            }
        }
    }
}
