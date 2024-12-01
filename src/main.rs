mod entities;
mod repos;
mod use_cases;
use std::path::Path;

use chrono::NaiveDate;
use entities::Todo;
use repos::JsonRepo;

fn main() {
    let todos_path = Path::new("todos.json");
    let mut repo = JsonRepo::new(todos_path.to_path_buf()).expect("Failed to create repo");

    let todo = Todo::new(
        1,
        "Buy milk".to_string(),
        false,
        NaiveDate::from_ymd_opt(2021, 1, 1).expect("Invalid date"),
    );

    use_cases::add_todo(&mut repo, &todo).expect("Failed to add todo");

    let todos = use_cases::list_todos(&repo).expect("Failed to list todos");
    for todo in todos {
        println!("{:?}", todo);
    }
}
