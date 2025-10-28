use crate::models::user::User;
use crate::repositories::user_repository::UserRepositoryTrait;
use regex::Regex;

pub struct UserService<T: UserRepositoryTrait> {
    repository: T,
}

#[derive(Debug)]
#[allow(dead_code)] // 全てのバリアントがテストで使用されるため
pub enum UserError {
    InvalidEmail(String),
    InvalidUsername(String),
    InvalidPhone(String),
    InvalidAge(String),
    UserNotFound(String),
    RepositoryError(String),
    UserAlreadyExists(String),
}

impl From<String> for UserError {
    fn from(error: String) -> Self {
        UserError::RepositoryError(error)
    }
}

impl<T: UserRepositoryTrait> UserService<T> {
    pub fn new(repository: T) -> Self {
        Self { repository }
    }

    pub fn create_user(
        &self,
        email: String,
        username: String,
        phone: String,
        age: u32,
    ) -> Result<User, UserError> {
        self.validate_email(&email)?;
        self.validate_username(&username)?;
        self.validate_phone(&phone)?;
        self.validate_age(age)?;

        // Check if user already exists
        if let Ok(Some(_)) = self.repository.find_by_email(&email) {
            return Err(UserError::UserAlreadyExists(format!(
                "User with email {} already exists",
                email
            )));
        }

        let user = User {
            email,
            username,
            phone,
            age,
        };

        self.repository
            .save(&user)
            .map_err(UserError::RepositoryError)?;

        Ok(user)
    }

    pub fn update_user(
        &self,
        email: String,
        username: String,
        phone: String,
        age: u32,
    ) -> Result<User, UserError> {
        self.validate_username(&username)?;
        self.validate_phone(&phone)?;
        self.validate_age(age)?;

        // Check if user exists
        if self.repository.find_by_email(&email)?.is_none() {
            return Err(UserError::UserNotFound(format!(
                "User with email {} not found",
                email
            )));
        }

        let user = User {
            email,
            username,
            phone,
            age,
        };

        self.repository
            .save(&user)
            .map_err(UserError::RepositoryError)?;

        Ok(user)
    }

    pub fn get_user(&self, email: &str) -> Result<User, UserError> {
        self.repository
            .find_by_email(email)?
            .ok_or_else(|| UserError::UserNotFound(format!("User with email {} not found", email)))
    }

    pub fn list_users(&self) -> Result<Vec<User>, UserError> {
        self.repository
            .find_all()
            .map_err(UserError::RepositoryError)
    }

    pub fn delete_user(&self, email: &str) -> Result<(), UserError> {
        if !self
            .repository
            .delete(email)
            .map_err(UserError::RepositoryError)?
        {
            return Err(UserError::UserNotFound(format!(
                "User with email {} not found",
                email
            )));
        }
        Ok(())
    }

    fn validate_email(&self, email: &str) -> Result<(), UserError> {
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(email) {
            return Err(UserError::InvalidEmail(format!(
                "Invalid email format: {}",
                email
            )));
        }
        Ok(())
    }

    fn validate_username(&self, username: &str) -> Result<(), UserError> {
        if username.trim().is_empty() || username.len() < 3 {
            return Err(UserError::InvalidUsername(
                "Username must be at least 3 characters long".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_phone(&self, phone: &str) -> Result<(), UserError> {
        let phone_regex = Regex::new(r"^\d{10,}$").unwrap();
        if !phone_regex.is_match(phone) {
            return Err(UserError::InvalidPhone(
                "Phone number must be at least 10 digits".to_string(),
            ));
        }
        Ok(())
    }

    fn validate_age(&self, age: u32) -> Result<(), UserError> {
        if age > 150 {
            return Err(UserError::InvalidAge(
                "Age must be between 0 and 150".to_string(),
            ));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::user_repository::MockUserRepositoryTrait;

    fn create_mock_repository() -> MockUserRepositoryTrait {
        MockUserRepositoryTrait::new()
    }

    #[test]
    fn test_create_user_success() {
        let mut mock_repo = create_mock_repository();
        mock_repo.expect_find_by_email().return_once(|_| Ok(None));
        mock_repo.expect_save().return_once(|_| Ok(()));

        let service = UserService::new(mock_repo);
        let result = service.create_user(
            "test@example.com".to_string(),
            "testuser".to_string(),
            "1234567890".to_string(),
            25,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_create_user_invalid_email() {
        let mock_repo = create_mock_repository();
        let service = UserService::new(mock_repo);
        let result = service.create_user(
            "invalid-email".to_string(),
            "testuser".to_string(),
            "1234567890".to_string(),
            25,
        );

        assert!(matches!(result, Err(UserError::InvalidEmail(_))));
    }

    #[test]
    fn test_update_user_not_found() {
        let mut mock_repo = create_mock_repository();
        mock_repo.expect_find_by_email().return_once(|_| Ok(None));

        let service = UserService::new(mock_repo);
        let result = service.update_user(
            "test@example.com".to_string(),
            "testuser".to_string(),
            "1234567890".to_string(),
            25,
        );

        assert!(matches!(result, Err(UserError::UserNotFound(_))));
    }

    #[test]
    fn test_delete_user_success() {
        let mut mock_repo = create_mock_repository();
        mock_repo.expect_delete().return_once(|_| Ok(true));

        let service = UserService::new(mock_repo);
        let result = service.delete_user("test@example.com");

        assert!(result.is_ok());
    }
}
