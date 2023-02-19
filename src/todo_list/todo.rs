pub struct Todo {
    pub content: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(input: String) -> Todo {
        Todo {
            content: input,
            completed: false,
        }
    }
}
