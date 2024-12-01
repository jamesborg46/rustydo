use crate::entities::Todo;

#[cfg_attr(test, mockall::automock)]
pub trait TodoRepo {
    fn push_todo(&self, todo: &Todo);
    fn list_todos(&self) -> Vec<Todo>;
}
