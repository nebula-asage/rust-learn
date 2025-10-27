use crate::models::user::User;
use crate::repositories::user_repository::UserRepository;
use crate::services::user_service::UserService;

pub struct UserCommand {
    service: UserService<UserRepository>,
}

impl UserCommand {
    pub fn new() -> Self {
        let repository = UserRepository::new();
        let service = UserService::new(repository);
        Self { service }
    }

    pub fn create(&self, args: &[String]) -> Result<(), String> {
        if args.len() != 4 {
            return Err("Usage: create <email> <username> <phone> <age>".to_string());
        }

        let email = &args[0];
        let username = &args[1];
        let phone = &args[2];
        let age = args[3].parse::<u32>().map_err(|_| "Invalid age format")?;

        match self.service.create_user(
            email.to_string(),
            username.to_string(),
            phone.to_string(),
            age,
        ) {
            Ok(user) => {
                println!("User created successfully:");
                self.print_user(&user);
                Ok(())
            }
            Err(e) => Err(format!("Failed to create user: {:?}", e)),
        }
    }

    pub fn update(&self, args: &[String]) -> Result<(), String> {
        if args.len() != 4 {
            return Err("Usage: update <email> <username> <phone> <age>".to_string());
        }

        let email = &args[0];
        let username = &args[1];
        let phone = &args[2];
        let age = args[3].parse::<u32>().map_err(|_| "Invalid age format")?;

        match self.service.update_user(
            email.to_string(),
            username.to_string(),
            phone.to_string(),
            age,
        ) {
            Ok(user) => {
                println!("User updated successfully:");
                self.print_user(&user);
                Ok(())
            }
            Err(e) => Err(format!("Failed to update user: {:?}", e)),
        }
    }

    pub fn list(&self) -> Result<(), String> {
        match self.service.list_users() {
            Ok(users) => {
                println!("User list:");
                println!("Email\t\tUsername");
                println!("------------------------");
                for user in users {
                    println!("{}\t{}", user.email, user.username);
                }
                Ok(())
            }
            Err(e) => Err(format!("Failed to list users: {:?}", e)),
        }
    }

    pub fn get(&self, args: &[String]) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Usage: get <email>".to_string());
        }

        let email = &args[0];
        match self.service.get_user(email) {
            Ok(user) => {
                self.print_user(&user);
                Ok(())
            }
            Err(e) => Err(format!("Failed to get user: {:?}", e)),
        }
    }

    pub fn delete(&self, args: &[String]) -> Result<(), String> {
        if args.len() != 1 {
            return Err("Usage: delete <email>".to_string());
        }

        let email = &args[0];
        match self.service.delete_user(email) {
            Ok(()) => {
                println!("User deleted successfully");
                Ok(())
            }
            Err(e) => Err(format!("Failed to delete user: {:?}", e)),
        }
    }

    fn print_user(&self, user: &User) {
        println!("Email: {}", user.email);
        println!("Username: {}", user.username);
        println!("Phone: {}", user.phone);
        println!("Age: {}", user.age);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::NamedTempFile;

    fn setup() -> UserCommand {
        let temp_file = NamedTempFile::new().unwrap();
        unsafe {
            env::set_var("USER_DATA_FILE", temp_file.path().to_str().unwrap());
        }
        UserCommand::new()
    }

    #[test]
    fn test_create_user_command() {
        let command = setup();
        let args = vec![
            "test@example.com".to_string(),
            "testuser".to_string(),
            "1234567890".to_string(),
            "25".to_string(),
        ];

        let result = command.create(&args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_create_user_invalid_args() {
        let command = setup();
        let args = vec!["test@example.com".to_string()];

        let result = command.create(&args);
        assert!(result.is_err());
    }

    #[test]
    fn test_update_user_command() {
        let command = setup();
        let create_args = vec![
            "test@example.com".to_string(),
            "testuser".to_string(),
            "1234567890".to_string(),
            "25".to_string(),
        ];
        command.create(&create_args).unwrap();

        let update_args = vec![
            "test@example.com".to_string(),
            "newuser".to_string(),
            "0987654321".to_string(),
            "30".to_string(),
        ];
        let result = command.update(&update_args);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_user_command() {
        let command = setup();
        let create_args = vec![
            "test@example.com".to_string(),
            "testuser".to_string(),
            "1234567890".to_string(),
            "25".to_string(),
        ];
        command.create(&create_args).unwrap();

        let delete_args = vec!["test@example.com".to_string()];
        let result = command.delete(&delete_args);
        assert!(result.is_ok());
    }
}
