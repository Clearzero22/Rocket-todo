use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Subtask {
    pub id: Option<i64>,
    pub parent_todo_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<NaiveDateTime>,
    pub order_index: i64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateSubtaskRequest {
    pub parent_todo_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    pub order_index: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateSubtaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
    pub due_date: Option<NaiveDateTime>,
    pub order_index: Option<i64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SubtaskResponse {
    pub id: i64,
    pub parent_todo_id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<DateTime<Utc>>,
    pub order_index: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TodoWithSubtasksResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub subtasks: Vec<SubtaskResponse>,
}

impl From<Subtask> for SubtaskResponse {
    fn from(subtask: Subtask) -> Self {
        SubtaskResponse {
            id: subtask.id.unwrap_or(0),
            parent_todo_id: subtask.parent_todo_id,
            title: subtask.title,
            description: subtask.description,
            status: subtask.status,
            priority: subtask.priority,
            due_date: subtask
                .due_date
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc)),
            order_index: subtask.order_index,
            created_at: subtask
                .created_at
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|| Utc::now()),
            updated_at: subtask
                .updated_at
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|| Utc::now()),
        }
    }
}

impl Subtask {
    pub fn new(
        parent_todo_id: i64,
        title: String,
        description: Option<String>,
        priority: Option<String>,
        due_date: Option<NaiveDateTime>,
        order_index: Option<i32>,
    ) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: None,
            parent_todo_id,
            title,
            description,
            status: "pending".to_string(),
            priority: priority.unwrap_or_else(|| "medium".to_string()),
            due_date,
            order_index: order_index.unwrap_or(0) as i64,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }

    pub fn update(&mut self, update: UpdateSubtaskRequest) {
        let now = Utc::now().naive_utc();

        if let Some(title) = update.title {
            self.title = title;
        }

        if let Some(description) = update.description {
            self.description = Some(description);
        }

        if let Some(status) = update.status {
            if ["pending", "in_progress", "completed"].contains(&status.as_str()) {
                self.status = status;
            }
        }

        if let Some(priority) = update.priority {
            if ["low", "medium", "high"].contains(&priority.as_str()) {
                self.priority = priority;
            }
        }

        if let Some(due_date) = update.due_date {
            self.due_date = Some(due_date);
        }

        if let Some(order_index) = update.order_index {
            self.order_index = order_index;
        }

        self.updated_at = Some(now);
    }

    pub fn is_completed(&self) -> bool {
        self.status == "completed"
    }

    pub fn is_overdue(&self) -> bool {
        if let (Some(due_date), false) = (self.due_date, self.is_completed()) {
            due_date < Utc::now().naive_utc()
        } else {
            false
        }
    }

    pub fn get_progress_percentage(subtasks: &[Subtask]) -> f32 {
        if subtasks.is_empty() {
            return 0.0;
        }

        let completed_count = subtasks.iter().filter(|s| s.is_completed()).count();
        (completed_count as f32 / subtasks.len() as f32) * 100.0
    }

    pub fn reorder_subtasks(subtasks: &mut [Subtask], start_index: i64) {
        for (index, subtask) in subtasks.iter_mut().enumerate() {
            subtask.order_index = start_index + index as i64;
        }
    }
}

// Batch operations for subtasks
#[derive(Debug, Deserialize, ToSchema)]
pub struct BatchUpdateSubtasksRequest {
    pub subtasks: Vec<UpdateSubtaskRequest>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ReorderSubtasksRequest {
    pub subtask_ids: Vec<i64>,
}

// Validation functions
impl CreateSubtaskRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Title cannot be empty".to_string());
        }

        if let Some(ref priority) = self.priority {
            if !["low", "medium", "high"].contains(&priority.as_str()) {
                return Err("Priority must be 'low', 'medium', or 'high'".to_string());
            }
        }

        Ok(())
    }
}

impl UpdateSubtaskRequest {
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref status) = self.status {
            if !["pending", "in_progress", "completed"].contains(&status.as_str()) {
                return Err("Status must be 'pending', 'in_progress', or 'completed'".to_string());
            }
        }

        if let Some(ref priority) = self.priority {
            if !["low", "medium", "high"].contains(&priority.as_str()) {
                return Err("Priority must be 'low', 'medium', or 'high'".to_string());
            }
        }

        if let Some(ref title) = self.title {
            if title.trim().is_empty() {
                return Err("Title cannot be empty".to_string());
            }
        }

        Ok(())
    }
}

impl ReorderSubtasksRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.subtask_ids.is_empty() {
            return Err("Subtask IDs list cannot be empty".to_string());
        }

        // Check for duplicate IDs
        let mut unique_ids = self.subtask_ids.clone();
        unique_ids.sort();
        unique_ids.dedup();

        if unique_ids.len() != self.subtask_ids.len() {
            return Err("Duplicate subtask IDs are not allowed".to_string());
        }

        Ok(())
    }
}