use chrono::Local;
use colored::Colorize;

use crate::error::AppError;
use crate::models::{Priority, Todo};
use crate::storage::{load, save};

pub fn add(text: String, priority: Priority, tags: Vec<String>) -> Result<(), AppError> {
    let mut store = load()?;
    let id = store.next_id;
    store.next_id += 1;
    store.todos.push(Todo {
        id,
        text: text.clone(),
        done: false,
        priority,
        tags,
        created_at: Local::now(),
        completed_at: None,
    });
    save(&store)?;
    println!("{} #{}: {}", "Added".green(), id, text);
    Ok(())
}

pub fn list_todos(
    show_done: bool,
    tag: Option<&str>,
    priority: Option<Priority>,
) -> Result<(), AppError> {
    let store = load()?;
    let todos: Vec<&Todo> = store
        .todos
        .iter()
        .filter(|t| show_done || !t.done)
        .filter(|t| tag.map_or(true, |tag| t.tags.iter().any(|tt| tt == tag)))
        .filter(|t| priority.map_or(true, |p| t.priority == p))
        .collect();

    if todos.is_empty() {
        println!("{}", "No todos found.".dimmed());
        return Ok(());
    }

    println!(
        "{:<4} {:<6} {:<6} {:<40} {:<20} {}",
        "ID", "Status", "Pri", "Text", "Tags", "Created"
    );
    println!("{}", "-".repeat(90));

    for t in &todos {
        let status = if t.done {
            "done".green().to_string()
        } else {
            "open".dimmed().to_string()
        };

        let pri = match t.priority {
            Priority::High => "high".red().to_string(),
            Priority::Medium => "medium".yellow().to_string(),
            Priority::Low => "low".green().to_string(),
        };

        let tags = if t.tags.is_empty() {
            String::new()
        } else {
            t.tags.join(", ")
        };

        let created = t.created_at.format("%Y-%m-%d").to_string();

        println!(
            "{:<4} {:<6} {:<6} {:<40} {:<20} {}",
            t.id, status, pri, t.text, tags, created
        );
    }

    Ok(())
}

pub fn done(todo_id: u64) -> Result<(), AppError> {
    let mut store = load()?;
    let todo = store
        .todos
        .iter_mut()
        .find(|t| t.id == todo_id)
        .ok_or(AppError::NotFound(todo_id))?;

    if todo.done {
        return Err(AppError::AlreadyDone(todo_id));
    }

    todo.done = true;
    todo.completed_at = Some(Local::now());
    let text = todo.text.clone();
    save(&store)?;
    println!("{} #{}: {}", "Completed".green(), todo_id, text);
    Ok(())
}

pub fn remove(todo_id: u64) -> Result<(), AppError> {
    let mut store = load()?;
    let len_before = store.todos.len();
    store.todos.retain(|t| t.id != todo_id);
    if store.todos.len() == len_before {
        return Err(AppError::NotFound(todo_id));
    }
    save(&store)?;
    println!("{} #{}", "Removed".red(), todo_id);
    Ok(())
}

pub fn edit(
    todo_id: u64,
    text: Option<String>,
    priority: Option<Priority>,
    tags: Option<Vec<String>>,
) -> Result<(), AppError> {
    let mut store = load()?;
    let todo = store
        .todos
        .iter_mut()
        .find(|t| t.id == todo_id)
        .ok_or(AppError::NotFound(todo_id))?;

    if let Some(t) = text {
        todo.text = t;
    }
    if let Some(p) = priority {
        todo.priority = p;
    }
    if let Some(tg) = tags {
        todo.tags = tg;
    }

    save(&store)?;
    println!("{} #{}", "Updated".yellow(), todo_id);
    Ok(())
}

pub fn stats() -> Result<(), AppError> {
    let store = load()?;
    let total = store.todos.len();
    let completed = store.todos.iter().filter(|t| t.done).count();
    let pending = total - completed;

    let mut by_priority: std::collections::BTreeMap<Priority, usize> =
        std::collections::BTreeMap::new();
    for t in store.todos.iter().filter(|t| !t.done) {
        *by_priority.entry(t.priority).or_insert(0) += 1;
    }

    println!();
    println!("{}", "Stats".bold());
    println!(
        "  Total: {}  |  Pending: {}  |  Done: {}",
        total, pending, completed
    );
    if !by_priority.is_empty() {
        let parts: Vec<String> = by_priority
            .iter()
            .map(|(p, c)| format!("{}: {}", p, c))
            .collect();
        println!("  By priority: {}", parts.join(", "));
    }
    println!();

    Ok(())
}
