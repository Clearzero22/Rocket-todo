use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Todo {
    pub id: Option<i64>,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl Priority {
    pub fn as_str(&self) -> &'static str {
        match self {
            Priority::Low => "low",
            Priority::Medium => "medium",
            Priority::High => "high",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "low" => Some(Priority::Low),
            "medium" => Some(Priority::Medium),
            "high" => Some(Priority::High),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum Status {
    Pending,
    InProgress,
    Completed,
}

impl Status {
    pub fn as_str(&self) -> &'static str {
        match self {
            Status::Pending => "pending",
            Status::InProgress => "in_progress",
            Status::Completed => "completed",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(Status::Pending),
            "in_progress" => Some(Status::InProgress),
            "completed" => Some(Status::Completed),
            _ => None,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTodoRequest {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<Status>,
    pub priority: Option<Priority>,
    pub due_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TodoResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Todo> for TodoResponse {
    fn from(todo: Todo) -> Self {
        TodoResponse {
            id: todo.id.unwrap_or(0),
            title: todo.title,
            description: todo.description,
            status: todo.status,
            priority: todo.priority,
            due_date: todo.due_date.map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            created_at: todo
                .created_at
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|| Utc::now()),
            updated_at: todo
                .updated_at
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|| Utc::now()),
        }
    }
}

impl Todo {
    pub fn new(
        title: String,
        description: Option<String>,
        priority: Priority,
        status: Status,
        due_date: Option<NaiveDateTime>,
    ) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: None, // Will be set by database
            title,
            description,
            status: status.as_str().to_string(),
            priority: priority.as_str().to_string(),
            due_date,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }

    pub fn update(&mut self, update: UpdateTodoRequest) {
        let now = Utc::now().naive_utc();

        if let Some(title) = update.title {
            self.title = title;
        }

        if let Some(description) = update.description {
            self.description = Some(description);
        }

        if let Some(status) = update.status {
            self.status = status.as_str().to_string();
        }

        if let Some(priority) = update.priority {
            self.priority = priority.as_str().to_string();
        }

        if let Some(due_date) = update.due_date {
            self.due_date = Some(due_date);
        }

        self.updated_at = Some(now);
    }

    pub fn is_completed(&self) -> bool {
        self.status == "completed"
    }

    pub fn get_priority(&self) -> Option<Priority> {
        Priority::from_str(&self.priority)
    }

    pub fn get_status(&self) -> Option<Status> {
        Status::from_str(&self.status)
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due_date) = self.due_date {
            let now = Utc::now().naive_utc();
            due_date < now && !self.is_completed()
        } else {
            false
        }
    }

    pub fn is_due_soon(&self, days: i64) -> bool {
        if let Some(due_date) = self.due_date {
            let now = Utc::now().naive_utc();
            let threshold = now + chrono::Duration::days(days);
            due_date <= threshold && due_date >= now && !self.is_completed()
        } else {
            false
        }
    }
}
