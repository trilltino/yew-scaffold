use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// User role enum for role-based access control (RBAC)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type, Default)]
#[sqlx(type_name = "text", rename_all = "lowercase")]
pub enum UserRole {
    #[serde(rename = "user")]
    #[default]
    User,
    #[serde(rename = "admin")]
    Admin,
    #[serde(rename = "chapter_lead")]
    ChapterLead,
}

impl UserRole {
    pub fn as_str(&self) -> &str {
        match self {
            UserRole::User => "user",
            UserRole::Admin => "admin",
            UserRole::ChapterLead => "chapter_lead",
        }
    }
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "admin" => UserRole::Admin,
            "chapter_lead" => UserRole::ChapterLead,
            _ => UserRole::User,
        }
    }
}

impl From<UserRole> for String {
    fn from(role: UserRole) -> Self {
        role.as_str().to_string()
    }
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub wallet_address: String,
    pub created_at: Option<DateTime<Utc>>,

    // Authentication fields (added for email+password auth)
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub role: String,
    pub email_verified: bool,
    pub last_login: Option<DateTime<Utc>>,
}

impl User {
    /// Create a new wallet-only user (legacy)
    pub fn new(username: String, wallet_address: String) -> Self {
        Self {
            id: 0,
            username,
            wallet_address,
            created_at: Some(Utc::now()),
            email: None,
            password_hash: None,
            role: UserRole::User.as_str().to_string(),
            email_verified: false,
            last_login: None,
        }
    }

    /// Create a new user with email and password
    pub fn new_with_password(username: String, email: String, password_hash: String) -> Self {
        Self {
            id: 0,
            username,
            wallet_address: String::new(), // Will be linked later
            created_at: Some(Utc::now()),
            email: Some(email),
            password_hash: Some(password_hash),
            role: UserRole::User.as_str().to_string(),
            email_verified: false,
            last_login: None,
        }
    }

    /// Get user role as enum
    pub fn get_role(&self) -> UserRole {
        UserRole::from(self.role.clone())
    }

    /// Check if user has a specific role
    pub fn has_role(&self, role: UserRole) -> bool {
        self.get_role() == role
    }

    /// Check if user is admin
    pub fn is_admin(&self) -> bool {
        self.has_role(UserRole::Admin)
    }

    /// Check if user is chapter lead
    pub fn is_chapter_lead(&self) -> bool {
        self.has_role(UserRole::ChapterLead)
    }
}