//! Services Layer Module

pub mod error;
pub mod todo_service;
pub mod user_service;
pub mod tag_service;
pub mod subtask_service;

// Re-export service structs
pub use error::ServiceError;
pub use error::ServiceResult;
pub use todo_service::TodoService;
pub use user_service::{UserService, AuthResponse};
pub use tag_service::TagService;
pub use subtask_service::SubtaskService;

/// Database connection type alias
pub type DbConnection = rocket_db_pools::Connection<crate::database::Db>;