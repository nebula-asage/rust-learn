use crate::models::user::User;
use serde_json;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait UserRepositoryTrait {
    fn save(&self, user: &User) -> Result<(), String>;
    fn find_by_email(&self, email: &str) -> Result<Option<User>, String>;
    fn find_all(&self) -> Result<Vec<User>, String>;
    fn delete(&self, email: &str) -> Result<bool, String>;
}

pub struct UserRepository {
    file_path: String,
}

impl Default for UserRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl UserRepository {
    pub fn new() -> Self {
        let file_path = env::var("USER_DATA_FILE").unwrap_or_else(|_| "userdata.json".to_string());
        Self { file_path }
    }

    fn read_users(&self) -> Result<HashMap<String, User>, String> {
        if !Path::new(&self.file_path).exists() {
            return Ok(HashMap::new());
        }

        let content = fs::read_to_string(&self.file_path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        if content.is_empty() {
            return Ok(HashMap::new());
        }

        serde_json::from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e))
    }

    fn write_users(&self, users: &HashMap<String, User>) -> Result<(), String> {
        let content = serde_json::to_string_pretty(users)
            .map_err(|e| format!("Failed to serialize JSON: {}", e))?;

        fs::write(&self.file_path, content).map_err(|e| format!("Failed to write file: {}", e))
    }
}

impl UserRepositoryTrait for UserRepository {
    fn save(&self, user: &User) -> Result<(), String> {
        let mut users = self.read_users()?;
        users.insert(user.email.clone(), user.clone());
        self.write_users(&users)
    }

    fn find_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let users = self.read_users()?;
        Ok(users.get(email).cloned())
    }

    fn find_all(&self) -> Result<Vec<User>, String> {
        let users = self.read_users()?;
        Ok(users.values().cloned().collect())
    }

    fn delete(&self, email: &str) -> Result<bool, String> {
        let mut users = self.read_users()?;
        let existed = users.remove(email).is_some();
        self.write_users(&users)?;
        Ok(existed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn create_test_user() -> User {
        User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            phone: "1234567890".to_string(),
            age: 25,
        }
    }

    #[test]
    fn test_save_and_find_user() {
        let temp_file = NamedTempFile::new().unwrap();
        unsafe {
            env::set_var("USER_DATA_FILE", temp_file.path().to_str().unwrap());
        }

        let repo = UserRepository::new();
        let user = create_test_user();

        // Test save
        repo.save(&user).unwrap();

        // Test find
        let found = repo.find_by_email(&user.email).unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap(), user);
    }

    #[test]
    fn test_find_all_users() {
        let temp_file = NamedTempFile::new().unwrap();
        unsafe {
            env::set_var("USER_DATA_FILE", temp_file.path().to_str().unwrap());
        }

        let repo = UserRepository::new();
        let user1 = create_test_user();
        let mut user2 = create_test_user();
        user2.email = "test2@example.com".to_string();

        repo.save(&user1).unwrap();
        repo.save(&user2).unwrap();

        let users = repo.find_all().unwrap();
        assert_eq!(users.len(), 2);
    }

    #[test]
    fn test_delete_user() {
        let temp_file = NamedTempFile::new().unwrap();
        unsafe {
            env::set_var("USER_DATA_FILE", temp_file.path().to_str().unwrap());
        }

        let repo = UserRepository::new();
        let user = create_test_user();

        repo.save(&user).unwrap();
        assert!(repo.delete(&user.email).unwrap());
        assert!(repo.find_by_email(&user.email).unwrap().is_none());
    }
}
