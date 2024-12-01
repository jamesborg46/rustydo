use std::fmt;

pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new(id: u32, title: String, completed: bool) -> Self {
        Todo {
            id,
            title,
            completed,
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Todo #{}: '{}' [{}]",
            self.id,
            self.title,
            if self.completed {
                "Completed"
            } else {
                "Pending"
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_creation() {
        let todo = Todo::new(1, String::from("Test Todo"), false);
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Test Todo");
        assert!(!todo.completed);
    }
}
