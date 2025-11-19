# Service Layer Implementation Guide

## 概述

本文档介绍了为Rocket Todo项目实现的Service Layer（服务层）架构。服务层将业务逻辑与数据访问分离，提高了代码的可维护性、可测试性和可复用性。

## 🏗️ 架构设计

### 分层架构

```
┌─────────────────┐
│   Routes Layer  │  ← HTTP 请求处理，参数验证
├─────────────────┤
│  Handlers Layer │  ← 业务逻辑协调，错误处理
├─────────────────┤
│ Services Layer │  ← 核心业务逻辑，数据验证
├─────────────────┤
│ Database Layer │  ← 数据持久化，SQL查询
└─────────────────┘
```

### 核心组件

1. **ServiceError**: 统一的错误类型定义
2. **BaseService**: 服务层基础接口
3. **具体服务**: UserService, TodoService, TagService, SubtaskService
4. **验证工具**: 通用的数据验证函数
5. **分页工具**: 统一的分页处理

## 📁 文件结构

```
src/
├── services/
│   ├── mod.rs              # 服务层模块定义和通用工具
│   ├── user_service.rs     # 用户相关业务逻辑
│   ├── todo_service.rs     # 任务相关业务逻辑
│   ├── tag_service.rs      # 标签相关业务逻辑
│   ├── subtask_service.rs  # 子任务相关业务逻辑
│   └── app_state.rs        # 应用状态管理
├── handlers/               # HTTP处理器（调用服务层）
└── models/                 # 数据模型
```

## 🔧 核心特性

### 1. 统一错误处理

```rust
#[derive(Debug, thiserror::Error)]
pub enum ServiceError {
    #[error("Database error: {0}")]
    Database(#[from] SqlxError),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Not found: {0}")]
    NotFound(String),
    #[error("Unauthorized: {0}")]
    Unauthorized(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Internal server error: {0}")]
    Internal(String),
}
```

### 2. 数据验证

```rust
pub mod validation {
    pub fn validate_email(email: &str) -> Result<(), ServiceError>
    pub fn validate_password(password: &str) -> Result<(), ServiceError>
    pub fn validate_title(title: &str) -> Result<(), ServiceError>
    pub fn validate_priority(priority: &str) -> Result<(), ServiceError>
    pub fn validate_status(status: &str) -> Result<(), ServiceError>
}
```

### 3. 分页支持

```rust
pub struct PaginationParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

pub struct PaginatedResponse<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub has_more: bool,
}
```

### 4. 用户数据隔离

所有服务层方法都包含用户ID参数，确保数据隔离：

```rust
// 创建任务时指定用户ID
pub async fn create_todo(
    &self,
    mut db: DbConnection,
    user_id: i64,  // 用户ID
    request: CreateTodoRequest,
) -> ServiceResult<TodoResponse>

// 查询时验证用户权限
pub async fn get_todo_by_id_with_user(
    &self,
    mut db: DbConnection,
    todo_id: i64,
    user_id: i64,  // 用户ID
) -> ServiceResult<TodoResponse>
```

## 📋 服务层方法

### UserService

- `create_user()` - 创建用户
- `authenticate_user()` - 用户认证
- `get_user_by_id()` - 根据ID获取用户
- `update_user()` - 更新用户信息
- `delete_user()` - 删除用户
- `list_users()` - 用户列表

### TodoService

- `create_todo()` - 创建任务
- `get_todo_by_id()` - 获取任务详情
- `update_todo()` - 更新任务
- `delete_todo()` - 删除任务
- `list_user_todos()` - 用户任务列表
- `list_todos_by_status()` - 按状态查询
- `list_todos_by_priority()` - 按优先级查询
- `search_todos()` - 搜索任务
- `get_overdue_todos()` - 过期任务
- `get_todo_statistics()` - 任务统计

### TagService

- `create_tag()` - 创建标签
- `get_tag_by_id()` - 获取标签
- `update_tag()` - 更新标签
- `delete_tag()` - 删除标签
- `list_user_tags()` - 用户标签列表
- `add_tag_to_todo()` - 为任务添加标签
- `remove_tag_from_todo()` - 移除任务标签
- `get_todos_by_tag()` - 按标签查询任务

### SubtaskService

- `create_subtask()` - 创建子任务
- `get_subtask_by_id()` - 获取子任务
- `update_subtask()` - 更新子任务
- `delete_subtask()` - 删除子任务
- `get_subtasks_by_todo()` - 任务的子任务列表
- `reorder_subtasks()` - 重新排序子任务
- `get_subtasks_by_status()` - 按状态查询子任务
- `get_overdue_subtasks()` - 过期子任务
- `get_subtask_progress()` - 子任务进度

## 💡 使用示例

### 1. 在Handler中使用服务层

```rust
use crate::services::{TodoService, ServiceError};

pub async fn create_todo(
    mut db: Connection<Db>,
    auth: JwtAuth,
    request: Json<CreateTodoRequest>,
) -> Result<Json<TodoResponse>, Status> {
    let user_id = auth.user_id;
    let todo_service = TodoService::new();

    match todo_service.create_todo(db, user_id, request.into_inner()).await {
        Ok(todo) => Ok(Json(todo)),
        Err(ServiceError::Validation(msg)) => {
            Err(Status::BadRequest)
        }
        Err(err) => Err(Status::InternalServerError),
    }
}
```

### 2. 复杂业务逻辑

```rust
pub async fn get_user_dashboard(
    mut db: Connection<Db>,
    auth: JwtAuth,
) -> Result<Json<DashboardData>, Status> {
    let user_id = auth.user_id;
    let todo_service = TodoService::new();
    let tag_service = TagService::new();

    // 获取统计数据
    let stats = todo_service.get_todo_statistics(db, user_id).await?;

    // 获取最近任务
    let recent_todos = todo_service.list_user_todos(
        db,
        user_id,
        PaginationParams::new(Some(5), Some(0))
    ).await?;

    // 获取热门标签
    let popular_tags = tag_service.get_popular_tags(db, user_id, 10).await?;

    Ok(Json(DashboardData {
        statistics: stats,
        recent_todos,
        popular_tags,
    }))
}
```

### 3. 错误处理

```rust
match service.method(db, user_id, request).await {
    Ok(result) => Ok(Json(result)),
    Err(ServiceError::Validation(msg)) => Err(status::Custom(
        status::BadRequest,
        Json(json!({ "error": "Validation failed", "message": msg }))
    )),
    Err(ServiceError::NotFound(msg)) => Err(status::Custom(
        status::NotFound,
        Json(json!({ "error": "Not found", "message": msg }))
    )),
    Err(ServiceError::Unauthorized(msg)) => Err(status::Custom(
        status::Unauthorized,
        Json(json!({ "error": "Unauthorized", "message": msg }))
    )),
    Err(ServiceError::Conflict(msg)) => Err(status::Custom(
        status::Conflict,
        Json(json!({ "error": "Conflict", "message": msg }))
    )),
    Err(err) => Err(status::Custom(
        status::InternalServerError,
        Json(json!({ "error": "Internal error", "message": format!("{}", err) }))
    )),
}
```

## 🔄 迁移指南

### 从直接数据库访问迁移到服务层

**之前 (直接SQL):**
```rust
pub async fn create_todo(
    mut db: Connection<Db>,
    request: Json<CreateTodoRequest>,
) -> Result<Json<TodoResponse>, Status> {
    let result = sqlx::query!(
        "INSERT INTO todos (title, description) VALUES (?, ?)",
        request.title,
        request.description
    )
    .execute(&mut **db)
    .await?;

    // 更多直接SQL代码...
}
```

**之后 (服务层):**
```rust
pub async fn create_todo(
    mut db: Connection<Db>,
    auth: JwtAuth,
    request: Json<CreateTodoRequest>,
) -> Result<Json<TodoResponse>, Status> {
    let user_id = auth.user_id;
    let todo_service = TodoService::new();

    match todo_service.create_todo(db, user_id, request.into_inner()).await {
        Ok(todo) => Ok(Json(todo)),
        Err(err) => handle_service_error(err),
    }
}
```

## 🎯 优势

### 1. 业务逻辑封装
- 所有业务规则集中在服务层
- 数据验证和业务规则统一管理
- 易于维护和测试

### 2. 数据隔离
- 强制用户数据隔离
- 防止数据泄露和越权访问
- 安全性更高

### 3. 错误处理统一
- 统一的错误类型和处理方式
- 更好的错误信息和调试体验
- 一致的API响应格式

### 4. 可测试性
- 服务层方法易于单元测试
- 业务逻辑与数据库操作分离
- Mock和测试友好的设计

### 5. 代码复用
- 相同的业务逻辑可以在多个Handler中复用
- 避免重复的SQL查询
- 统一的业务规则实现

## 🚀 下一步

1. **完善服务层**: 添加更多业务方法和复杂逻辑
2. **集成测试**: 为服务层编写单元测试和集成测试
3. **性能优化**: 添加缓存层和查询优化
4. **日志记录**: 添加详细的操作日志
5. **API文档**: 为服务层方法生成API文档

## 📚 参考资料

- [Clean Architecture](https://blog.cleancoder.com/uncle-bob-2017-05-05-article1.html)
- [Domain-Driven Design](https://en.wikipedia.org/wiki/Domain-driven_design)
- [Rust Service Layer Patterns](https://rust-lang.github.io/what-we-learn/writing-a-service-layer-in-rust.html)