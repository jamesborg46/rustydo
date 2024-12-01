use chrono::NaiveDate;
use std::fmt;

pub struct Todo {
    id: u32,
    title: String,
    completed: bool,
    date: NaiveDate,
}

impl Todo {
    pub fn new(id: u32, title: String, completed: bool, date: NaiveDate) -> Self {
        Todo {
            id,
            title,
            completed,
            date,
        }
    }
}

impl fmt::Display for Todo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Todo #{}: '{}' [{}] by {}",
            self.id,
            self.title,
            if self.completed {
                "Completed"
            } else {
                "Pending"
            },
            self.date
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo_creation() {
        let todo = Todo::new(
            1,
            String::from("Test Todo"),
            false,
            NaiveDate::from_ymd_opt(2021, 1, 1).expect("Invalid date"),
        );
        assert_eq!(todo.id, 1);
        assert_eq!(todo.title, "Test Todo");
        assert!(!todo.completed);
        assert_eq!(
            todo.date,
            NaiveDate::from_ymd_opt(2021, 1, 1).expect("Invalid date")
        );
    }
}
