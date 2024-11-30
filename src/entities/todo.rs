use std::fmt;

pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: String, completed: bool) -> Self {
        Self {
            id,
            title,
            completed,
        }
    }
}


// Implement the Display trait for Todo
impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Customize the string representation
        write!(
            f,
            "Todo #{}: '{}' [{}]",
            self.id,
            self.title,
            if self.completed { "Completed" } else { "Pending" }
        )
    }
}
