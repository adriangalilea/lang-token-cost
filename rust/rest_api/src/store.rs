use chrono::Utc;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::error::AppError;
use crate::models::*;

pub type SharedState = Arc<Mutex<Store>>;

pub struct Store {
    users: HashMap<u64, User>,
    posts: HashMap<u64, Post>,
    next_user_id: u64,
    next_post_id: u64,
}

impl Store {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            posts: HashMap::new(),
            next_user_id: 1,
            next_post_id: 1,
        }
    }

    pub fn create_user(&mut self, data: UserCreate) -> User {
        let now = Utc::now();
        let id = self.next_user_id;
        self.next_user_id += 1;
        let user = User {
            id,
            name: data.name,
            email: data.email,
            role: data.role,
            created_at: now,
            updated_at: now,
        };
        self.users.insert(id, user.clone());
        user
    }

    pub fn get_user(&self, id: u64) -> Result<&User, AppError> {
        self.users
            .get(&id)
            .ok_or_else(|| AppError::NotFound(format!("User {id} not found")))
    }

    pub fn list_users(&self, params: &UserListParams) -> PaginatedResponse<User> {
        let mut items: Vec<&User> = self.users.values().collect();

        if let Some(ref search) = params.search {
            let query = search.to_lowercase();
            items.retain(|u| {
                u.name.to_lowercase().contains(&query) || u.email.to_lowercase().contains(&query)
            });
        }

        let total = items.len();
        let page = params.pagination.page;
        let per_page = params.pagination.per_page;
        let start = ((page - 1) * per_page) as usize;
        let paged: Vec<User> = items
            .into_iter()
            .skip(start)
            .take(per_page as usize)
            .cloned()
            .collect();
        let pages = (total as u64 + per_page - 1) / per_page;

        PaginatedResponse {
            items: paged,
            total,
            page,
            per_page,
            pages,
        }
    }

    pub fn update_user(&mut self, id: u64, data: UserUpdate) -> Result<User, AppError> {
        let user = self
            .users
            .get_mut(&id)
            .ok_or_else(|| AppError::NotFound(format!("User {id} not found")))?;

        if let Some(name) = data.name {
            user.name = name;
        }
        if let Some(email) = data.email {
            user.email = email;
        }
        if let Some(role) = data.role {
            user.role = role;
        }
        user.updated_at = Utc::now();

        Ok(user.clone())
    }

    pub fn delete_user(&mut self, id: u64) -> Result<(), AppError> {
        self.users
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| AppError::NotFound(format!("User {id} not found")))
    }

    pub fn create_post(&mut self, author_id: u64, data: PostCreate) -> Result<Post, AppError> {
        if !self.users.contains_key(&author_id) {
            return Err(AppError::BadRequest(format!(
                "Author {author_id} not found"
            )));
        }

        let now = Utc::now();
        let id = self.next_post_id;
        self.next_post_id += 1;
        let post = Post {
            id,
            author_id,
            title: data.title,
            body: data.body,
            published: data.published,
            created_at: now,
            updated_at: now,
        };
        self.posts.insert(id, post.clone());
        Ok(post)
    }

    pub fn get_post(&self, id: u64) -> Result<&Post, AppError> {
        self.posts
            .get(&id)
            .ok_or_else(|| AppError::NotFound(format!("Post {id} not found")))
    }

    pub fn list_posts(&self, params: &PostListParams) -> PaginatedResponse<Post> {
        let mut items: Vec<&Post> = self.posts.values().collect();

        if let Some(author_id) = params.author_id {
            items.retain(|p| p.author_id == author_id);
        }
        if params.published {
            items.retain(|p| p.published);
        }

        let total = items.len();
        let page = params.pagination.page;
        let per_page = params.pagination.per_page;
        let start = ((page - 1) * per_page) as usize;
        let paged: Vec<Post> = items
            .into_iter()
            .skip(start)
            .take(per_page as usize)
            .cloned()
            .collect();
        let pages = (total as u64 + per_page - 1) / per_page;

        PaginatedResponse {
            items: paged,
            total,
            page,
            per_page,
            pages,
        }
    }

    pub fn update_post(&mut self, id: u64, data: PostUpdate) -> Result<Post, AppError> {
        let post = self
            .posts
            .get_mut(&id)
            .ok_or_else(|| AppError::NotFound(format!("Post {id} not found")))?;

        if let Some(title) = data.title {
            post.title = title;
        }
        if let Some(body) = data.body {
            post.body = body;
        }
        if let Some(published) = data.published {
            post.published = published;
        }
        post.updated_at = Utc::now();

        Ok(post.clone())
    }

    pub fn delete_post(&mut self, id: u64) -> Result<(), AppError> {
        self.posts
            .remove(&id)
            .map(|_| ())
            .ok_or_else(|| AppError::NotFound(format!("Post {id} not found")))
    }
}
