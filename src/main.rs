mod entities;

use entities::Todo;

fn main() {
    let todo = Todo::new(1, "Buy milk".to_string(), false);
    println!("Todo {}", todo);
}
