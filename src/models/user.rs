use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub email: String,
    pub username: String,
    pub phone: String,
    pub age: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        let user = User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            phone: "1234567890".to_string(),
            age: 25,
        };

        assert_eq!(user.email, "test@example.com");
        assert_eq!(user.username, "testuser");
        assert_eq!(user.phone, "1234567890");
        assert_eq!(user.age, 25);
    }

    #[test]
    fn test_user_serialization() {
        let user = User {
            email: "test@example.com".to_string(),
            username: "testuser".to_string(),
            phone: "1234567890".to_string(),
            age: 25,
        };

        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&serialized).unwrap();

        assert_eq!(user, deserialized);
    }
}
