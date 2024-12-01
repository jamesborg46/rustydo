use crate::entities::Todo;
use crate::use_cases::repo_interface::TodoRepo;
use std::error::Error;

pub fn add_todo(repo: &impl TodoRepo, todo: &Todo) -> Result<(), Box<dyn Error>> {
    repo.push_todo(todo);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::repo_interface::MockTodoRepo;
    use mockall::predicate;

    #[test]
    fn test_add_todo_success() {
        let mut mock_repo = MockTodoRepo::new();

        let todo = Todo::new(
            1,
            "Buy milk".to_string(),
            false,
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).expect("Invalid date"),
        );

        mock_repo
            .expect_push_todo()
            .with(predicate::eq(todo.clone()))
            .times(1)
            .returning(|_| ());

        let result = add_todo(&mock_repo, &todo);
        assert!(result.is_ok());
    }
}
