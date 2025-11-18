use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct Tag {
    pub id: Option<i64>,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateTagRequest {
    pub name: String,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateTagRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TagResponse {
    pub id: i64,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Tag> for TagResponse {
    fn from(tag: Tag) -> Self {
        TagResponse {
            id: tag.id.unwrap_or(0),
            name: tag.name,
            color: tag.color,
            description: tag.description,
            created_at: tag
                .created_at
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|| Utc::now()),
            updated_at: tag
                .updated_at
                .map(|dt| DateTime::from_naive_utc_and_offset(dt, Utc))
                .unwrap_or_else(|| Utc::now()),
        }
    }
}

// TodoTag junction model
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, ToSchema)]
pub struct TodoTag {
    pub id: Option<i64>,
    pub todo_id: i64,
    pub tag_id: i64,
    pub created_at: Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddTagRequest {
    pub todo_id: i64,
    pub tag_id: i64,
}

// Extended Todo response with tags
#[derive(Debug, Serialize, ToSchema)]
pub struct TodoWithTagsResponse {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: String,
    pub priority: String,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<TagResponse>,
}

impl Tag {
    pub fn new(
        name: String,
        color: Option<String>,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now().naive_utc();
        Self {
            id: None,
            name,
            color: color.unwrap_or_else(|| "#007bff".to_string()),
            description,
            created_at: Some(now),
            updated_at: Some(now),
        }
    }
  pub fn update(&mut self, update: UpdateTagRequest) {
        let now = Utc::now().naive_utc();

        if let Some(name) = update.name {
            self.name = name;
        }

        if let Some(color) = update.color {
            self.color = color;
        }

        if let Some(description) = update.description {
            self.description = Some(description);
        }

        self.updated_at = Some(now);
    }
}

// Default colors for tags
pub const DEFAULT_TAG_COLORS: &[&str] = &[
    "#007bff", // Blue
    "#28a745", // Green
    "#dc3545", // Red
    "#ffc107", // Yellow
    "#6f42c1", // Purple
    "#fd7e14", // Orange
    "#20c997", // Teal
    "#6c757d", // Gray
    "#e83e8c", // Pink
    "#17a2b8", // Cyan
];

impl Tag {
    pub fn get_random_color() -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
            .hash(&mut hasher);

        let index = hasher.finish() as usize % DEFAULT_TAG_COLORS.len();
        DEFAULT_TAG_COLORS[index].to_string()
    }
}