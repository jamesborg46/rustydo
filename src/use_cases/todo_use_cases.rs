use crate::entities::Todo;
use crate::use_cases::repo_interface::TodoRepo;
use std::error::Error;

pub fn add_todo(repo: &mut impl TodoRepo, todo: &Todo) -> Result<(), Box<dyn Error>> {
    repo.push_todo(todo)?;
    Ok(())
}

pub fn list_todos(repo: &impl TodoRepo) -> Result<Vec<Todo>, Box<dyn Error>> {
    let todos = repo.list_todos();
    Ok(todos)
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
            .returning(|_| (Ok(())));

        let result = add_todo(&mut mock_repo, &todo);
        assert!(result.is_ok());
    }

    #[test]
    fn test_list_todos_success() {
        let mut mock_repo = MockTodoRepo::new();

        let todo = Todo::new(
            1,
            "Buy milk".to_string(),
            false,
            chrono::NaiveDate::from_ymd_opt(2021, 1, 1).expect("Invalid date"),
        );

        mock_repo
            .expect_list_todos()
            .times(1)
            .return_const(vec![todo.clone()]);

        let result = list_todos(&mock_repo);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![todo]);
    }
}
