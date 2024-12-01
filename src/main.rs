mod entities;
mod use_cases;

use chrono::NaiveDate;
use entities::Todo;

fn main() {
    let todo = Todo::new(
        1,
        "Buy milk".to_string(),
        false,
        NaiveDate::from_ymd_opt(2021, 1, 1).expect("Invalid date"),
    );
    println!("Todo {}", todo);
}
