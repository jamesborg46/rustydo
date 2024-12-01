use crate::entities::Todo;

#[cfg_attr(test, mockall::automock)]
pub trait TodoRepo {
    fn push_todo(&self, todo: &Todo);
}
