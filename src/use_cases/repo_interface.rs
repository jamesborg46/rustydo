use crate::entities::Todo;

#[cfg_attr(test, mockall::automock)]
pub trait TodoRepo {
    fn push_todo(&mut self, todo: &Todo) -> Result<(), Box<dyn std::error::Error>>;
    fn list_todos(&self) -> Vec<Todo>;
}
