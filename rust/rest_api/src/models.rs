use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
    Moderator,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::User => write!(f, "user"),
            Role::Moderator => write!(f, "moderator"),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub email: String,
    #[serde(default = "default_role")]
    pub role: Role,
}

fn default_role() -> Role {
    Role::User
}

#[derive(Debug, Deserialize)]
pub struct UserUpdate {
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<Role>,
}

#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub role: Role,
    pub post_count: usize,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PostCreate {
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub published: bool,
}

#[derive(Debug, Deserialize)]
pub struct PostUpdate {
    pub title: Option<String>,
    pub body: Option<String>,
    pub published: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub id: u64,
    pub author_id: u64,
    pub title: String,
    pub body: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: u64,
    pub per_page: u64,
    pub pages: u64,
}

#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    #[serde(default = "default_page")]
    pub page: u64,
    #[serde(default = "default_per_page")]
    pub per_page: u64,
}

fn default_page() -> u64 {
    1
}

fn default_per_page() -> u64 {
    20
}

#[derive(Debug, Deserialize)]
pub struct UserListParams {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PostListParams {
    #[serde(flatten)]
    pub pagination: PaginationParams,
    pub author_id: Option<u64>,
    #[serde(default)]
    pub published: bool,
}
