# 开发指南

本文档为 My Rocket App 的开发提供详细的指导和最佳实践。

## 开发环境设置

### 前置要求

- **Rust 1.64+**: 推荐使用最新稳定版
- **Cargo**: Rust 包管理器
- **Git**: 版本控制
- **IDE**: 推荐使用 VS Code 或 IntelliJ IDEA with Rust plugin

### 安装 Rust

#### Windows

1. 访问 [rustup.rs](https://rustup.rs/)
2. 下载并运行 `rustup-init.exe`
3. 按照提示完成安装
4. 重启终端

#### Linux/macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 验证安装

```bash
rustc --version
cargo --version
```

## 项目结构

```
my-rocket-app/
├── src/
│   └── main.rs              # 主应用文件
├── doc/                     # 文档目录
│   ├── 1_create_project.md
│   ├── API.md
│   ├── CONFIGURATION.md
│   └── DEVELOPMENT.md
├── docs/                    # 详细文档
│   ├── API.md
│   ├── CONFIGURATION.md
│   └── DEVELOPMENT.md
├── Cargo.toml              # 项目配置
├── Rocket.toml             # Rocket 配置
├── .gitignore              # Git 忽略文件
└── README.md               # 项目说明
```

## 开发工作流

### 1. 启动开发服务器

```bash
# 开发模式（带调试信息）
cargo run

# 发布模式（优化性能）
cargo run --release

# 后台运行
cargo run &
```

### 2. 代码热重载

Rocket 支持代码热重载，修改代码后自动重启：

```bash
# 安装 cargo-watch（如果未安装）
cargo install cargo-watch

# 使用 watch 模式
cargo watch -x run
```

### 3. 代码格式化

```bash
# 格式化代码
cargo fmt

# 检查代码风格
cargo clippy
```

### 4. 运行测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 运行测试并显示输出
cargo test -- --nocapture
```

## 代码组织

### 模块结构

#### 当前结构 (单文件)

```rust
// src/main.rs
#[macro_use] extern crate rocket;

// 结构体定义
#[derive(Debug, Deserialize, Serialize)]
struct User { ... }

// 路由处理函数
#[get("/")]
fn index() -> &'static str { ... }

// 应用启动
#[launch]
fn rocket() -> _ { ... }
```

#### 推荐结构 (多模块)

```
src/
├── main.rs                 # 应用入口
├── lib.rs                  # 库入口
├── routes/                 # 路由模块
│   ├── mod.rs
│   ├── user.rs
│   └── api.rs
├── models/                 # 数据模型
│   ├── mod.rs
│   └── user.rs
├── handlers/               # 请求处理器
│   ├── mod.rs
│   └── user_handler.rs
└── config/                 # 配置模块
    ├── mod.rs
    └── settings.rs
```

### 模块化示例

#### lib.rs

```rust
pub mod routes;
pub mod models;
pub mod handlers;
pub mod config;

use rocket::*;

pub fn create_app() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes::user::routes())
        .mount("/api", routes::api::routes())
}
```

#### routes/user.rs

```rust
use rocket::*;
use crate::handlers::user_handler::*;

pub fn routes() -> Vec<Route> {
    routes![get_user, create_user, update_user, delete_user]
}
```

## 路由开发

### 基础路由

```rust
#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}
```

### 路径参数

```rust
#[get("/user/<id>")]
fn get_user(id: usize) -> String {
    format!("User ID: {}", id)
}

// 支持多种类型
#[get("/user/<id>", rank = 2)]
fn get_user_str(id: String) -> String {
    format!("User ID: {}", id)
}
```

### 查询参数

```rust
#[get("/search?<query>&<page>")]
fn search(query: String, page: Option<u32>) -> String {
    let page = page.unwrap_or(1);
    format!("Search: {}, Page: {}", query, page)
}
```

### 请求体处理

```rust
#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[post("/users", data = "<user>")]
fn create_user(user: Json<CreateUserRequest>) -> Status {
    // 处理用户创建
    Status::Created
}
```

### 异步路由

```rust
use rocket::tokio::time::{sleep, Duration};

#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Delayed for {} seconds", seconds)
}
```

## 错误处理

### 自定义错误类型

```rust
use rocket::http::Status;
use rocket::response::{Responder, Response};
use rocket::serde::json::Json;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        Response::build()
            .status(Status::BadRequest)
            .header(ContentType::JSON)
            .sized_body(self.serialize().unwrap().len(), Cursor::new(self.serialize().unwrap()))
            .ok()
    }
}
```

### 错误处理宏

```rust
#[macro_use]
extern crate rocket;

macro_rules! handle_error {
    ($result:expr, $error_msg:expr) => {
        match $result {
            Ok(value) => value,
            Err(_) => return Err(Status::InternalServerError),
        }
    };
}
```

## 数据验证

### 请求体验证

```rust
use rocket::form::{Form, FromForm};
use rocket::http::Status;

#[derive(FromForm)]
struct UserForm {
    name: String,
    email: String,
    age: u8,
}

impl UserForm {
    fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        
        if !self.email.contains('@') {
            return Err("Invalid email format".to_string());
        }
        
        if self.age < 1 || self.age > 120 {
            return Err("Age must be between 1 and 120".to_string());
        }
        
        Ok(())
    }
}

#[post("/users", data = "<user>")]
fn create_user(user: Form<UserForm>) -> Result<Status, String> {
    user.validate()?;
    // 处理用户创建
    Ok(Status::Created)
}
```

## 中间件开发

### 自定义中间件

```rust
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response, Data};

pub struct LoggingFairing;

#[rocket::async_trait]
impl Fairing for LoggingFairing {
    fn info(&self) -> Info {
        Info {
            name: "Logging Fairing",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        println!("Request: {} {}", request.method(), request.uri());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("Response: {} {}", request.method(), response.status());
    }
}

// 在应用中使用
#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(LoggingFairing)
        .mount("/", routes![/* your routes */])
}
```

## 测试开发

### 单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    #[test]
    fn test_index() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello, Rocket!".into()));
    }

    #[test]
    fn test_get_user() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/user/123").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
```

### 集成测试

```rust
// tests/integration_test.rs
use rocket::local::blocking::Client;
use rocket::http::Status;
use my_rocket_app::rocket;

#[test]
fn test_user_creation() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    
    let user_data = serde_json::json!({
        "id": 1,
        "name": "Test User",
        "age": 25
    });
    
    let response = client
        .post("/user")
        .header(ContentType::JSON)
        .body(user_data.to_string())
        .dispatch();
    
    assert_eq!(response.status(), Status::Created);
}
```

## 性能优化

### 异步处理

```rust
use rocket::tokio::task;

#[post("/heavy-task")]
async fn heavy_task() -> String {
    let result = task::spawn_blocking(|| {
        // CPU 密集型任务
        std::thread::sleep(std::time::Duration::from_secs(5));
        "Task completed"
    }).await.unwrap();
    
    result
}
```

### 连接池

```rust
use rocket_sync_db_pools::{database, diesel};

#[database("my_db")]
struct DbConn(diesel::PgConnection);

#[get("/users")]
async fn get_users(conn: DbConn) -> Result<Json<Vec<User>>, Status> {
    // 使用连接池
    Ok(Json(conn.run(|c| {
        // 数据库查询
        users::table.load::<User>(c)
    }).await?))
}
```

## 调试技巧

### 日志记录

```rust
use rocket::log::LogLevel;

#[get("/debug")]
fn debug_endpoint() -> String {
    rocket::log::info!("Debug endpoint called");
    rocket::log::warn!("This is a warning");
    rocket::log::error!("This is an error");
    "Debug information logged"
}
```

### 请求追踪

```rust
use rocket::request::Request;
use rocket::response::{self, Responder, Response};

pub struct TraceResponse<T>(pub T);

impl<'r, 'o: 'r, T: Responder<'r, 'o>> Responder<'r, 'o> for TraceResponse<T> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
        println!("Request: {:?}", req);
        self.0.respond_to(req)
    }
}
```

## 部署准备

### 环境配置

```toml
# Rocket.toml
[release]
address = "0.0.0.0"
port = 8000
log_level = "critical"
workers = 4
secret_key = "${SECRET_KEY}"
```

### Docker 配置

```dockerfile
# Dockerfile
FROM rust:1.70 as builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/my-rocket-app /usr/local/bin/
EXPOSE 8000
CMD ["my-rocket-app"]
```

## 代码质量

### Clippy 配置

```toml
# .cargo/config.toml
[clippy]
all-targets = true
warn-on-all-wildcard-imports = true
```

### 代码格式化

```toml
# rustfmt.toml
max_width = 100
tab_spaces = 4
newline_style = "Unix"
```

## 常见问题

### 1. 编译错误

**问题**: `cannot find macro 'get' in this scope`

**解决方案**: 确保添加了 `#[macro_use] extern crate rocket;`

### 2. 路由冲突

**问题**: 多个路由匹配同一个路径

**解决方案**: 使用 `rank` 参数设置优先级

### 3. 异步函数错误

**问题**: 在非异步上下文中使用 `await`

**解决方案**: 确保函数标记为 `async`

## 最佳实践

1. **模块化**: 将代码组织到不同的模块中
2. **错误处理**: 使用适当的错误类型和状态码
3. **验证**: 验证所有输入数据
4. **测试**: 编写全面的单元测试和集成测试
5. **文档**: 为所有公共 API 编写文档
6. **性能**: 使用异步处理提高性能
7. **安全**: 验证输入，使用 HTTPS，保护敏感数据

---

*最后更新: 2024年*
