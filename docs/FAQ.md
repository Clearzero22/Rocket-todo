# 常见问题解答 (FAQ)

本文档回答了 My Rocket App 的常见问题和解决方案。

## 安装和设置

### Q: 如何安装 Rust？

**A**: 访问 [rustup.rs](https://rustup.rs/) 并按照页面指示安装。安装完成后运行以下命令验证：

```bash
rustc --version
cargo --version
```

### Q: 项目需要什么版本的 Rust？

**A**: 项目需要 Rust 1.64 或更高版本。推荐使用最新稳定版：

```bash
rustup update
```

### Q: 如何克隆和运行项目？

**A**: 
```bash
git clone <repository-url>
cd my-rocket-app
cargo run
```

### Q: 编译时出现 "cannot find macro" 错误？

**A**: 确保在 `main.rs` 文件顶部添加了：

```rust
#[macro_use] extern crate rocket;
```

## 配置问题

### Q: 如何修改服务器端口？

**A**: 编辑 `Rocket.toml` 文件：

```toml
[default]
port = 8080  # 修改为您想要的端口
```

### Q: 如何设置环境变量？

**A**: 

**Windows (PowerShell):**
```powershell
$env:SECRET_KEY = "your-secret-key"
```

**Windows (CMD):**
```cmd
set SECRET_KEY=your-secret-key
```

**Linux/macOS:**
```bash
export SECRET_KEY="your-secret-key"
```

### Q: 配置文件不生效怎么办？

**A**: 
1. 检查 `Rocket.toml` 语法是否正确
2. 重启应用
3. 确保配置文件在项目根目录

## 运行时问题

### Q: 端口被占用怎么办？

**A**: 

**查找占用端口的进程：**
```bash
# Windows
netstat -ano | findstr :8000

# Linux/macOS
lsof -ti:8000
```

**终止进程：**
```bash
# Windows
taskkill /PID <PID> /F

# Linux/macOS
kill -9 <PID>
```

**或修改端口：**
```toml
[default]
port = 8001
```

### Q: 应用启动失败？

**A**: 检查以下几点：

1. **端口冲突**: 确保端口未被占用
2. **配置文件**: 检查 `Rocket.toml` 语法
3. **依赖问题**: 运行 `cargo clean && cargo build`
4. **权限问题**: 确保有足够权限

### Q: 如何查看详细日志？

**A**: 修改 `Rocket.toml` 中的日志级别：

```toml
[debug]
log_level = "debug"  # 显示详细日志
```

## 开发问题

### Q: 如何添加新的路由？

**A**: 
1. 定义处理函数：
```rust
#[get("/new-route")]
fn new_route() -> String {
    "New route response".to_string()
}
```

2. 在 `routes!` 宏中注册：
```rust
routes![
    index,
    new_route,  // 添加新路由
    // ... 其他路由
]
```

### Q: 如何处理 POST 请求？

**A**: 
```rust
#[derive(Deserialize)]
struct UserData {
    name: String,
    age: u8,
}

#[post("/users", data = "<user>")]
fn create_user(user: Json<UserData>) -> Status {
    // 处理用户创建
    Status::Created
}
```

### Q: 如何返回 JSON 响应？

**A**: 
```rust
use rocket::serde::json::Json;

#[derive(Serialize)]
struct Response {
    message: String,
    status: String,
}

#[get("/api/status")]
fn api_status() -> Json<Response> {
    Json(Response {
        message: "API is running",
        status: "ok",
    })
}
```

### Q: 如何处理查询参数？

**A**: 
```rust
#[get("/search?<query>&<page>")]
fn search(query: String, page: Option<u32>) -> String {
    let page = page.unwrap_or(1);
    format!("Search: {}, Page: {}", query, page)
}
```

## 异步编程

### Q: 如何创建异步路由？

**A**: 
```rust
use rocket::tokio::time::{sleep, Duration};

#[get("/async-route")]
async fn async_route() -> String {
    sleep(Duration::from_secs(1)).await;
    "Async response".to_string()
}
```

### Q: 异步函数中可以使用 `await` 吗？

**A**: 可以，但函数必须标记为 `async`：

```rust
#[get("/async-example")]
async fn async_example() -> String {
    let result = some_async_function().await;
    format!("Result: {}", result)
}
```

## 错误处理

### Q: 如何自定义错误响应？

**A**: 
```rust
use rocket::http::Status;
use rocket::response::{Responder, Response};

#[derive(Debug)]
struct CustomError {
    message: String,
}

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> Result<Response<'static>, Status> {
        Response::build()
            .status(Status::BadRequest)
            .sized_body(self.message.len(), Cursor::new(self.message))
            .ok()
    }
}
```

### Q: 如何返回不同的 HTTP 状态码？

**A**: 
```rust
use rocket::http::Status;

#[get("/not-found")]
fn not_found() -> (Status, &'static str) {
    (Status::NotFound, "Resource not found")
}

#[post("/created")]
fn created() -> (Status, &'static str) {
    (Status::Created, "Resource created")
}
```

## 测试

### Q: 如何运行测试？

**A**: 
```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 显示测试输出
cargo test -- --nocapture
```

### Q: 如何编写集成测试？

**A**: 在 `tests/` 目录下创建测试文件：

```rust
// tests/integration_test.rs
use rocket::local::blocking::Client;
use my_rocket_app::rocket;

#[test]
fn test_index() {
    let client = Client::tracked(rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
```

## 性能优化

### Q: 如何提高应用性能？

**A**: 
1. **使用发布模式**:
   ```bash
   cargo run --release
   ```

2. **优化配置**:
   ```toml
   [release]
   workers = 4  # 根据 CPU 核心数调整
   ```

3. **启用链接时优化**:
   ```toml
   [profile.release]
   lto = true
   codegen-units = 1
   ```

### Q: 如何监控应用性能？

**A**: 
1. **启用详细日志**:
   ```toml
   [debug]
   log_level = "debug"
   ```

2. **添加性能指标**:
   ```rust
   use std::time::Instant;
   
   #[get("/timed")]
   fn timed_route() -> String {
       let start = Instant::now();
       // 执行操作
       let duration = start.elapsed();
       format!("Operation took: {:?}", duration)
   }
   ```

## 部署问题

### Q: 如何部署到生产环境？

**A**: 
1. **编译发布版本**:
   ```bash
   cargo build --release
   ```

2. **配置生产环境**:
   ```toml
   [release]
   address = "0.0.0.0"
   port = 8000
   log_level = "critical"
   ```

3. **使用 Docker**:
   ```dockerfile
   FROM rust:1.70 as builder
   WORKDIR /app
   COPY . .
   RUN cargo build --release
   
   FROM debian:bullseye-slim
   COPY --from=builder /app/target/release/my-rocket-app /usr/local/bin/
   EXPOSE 8000
   CMD ["my-rocket-app"]
   ```

### Q: 如何设置反向代理？

**A**: 使用 Nginx 配置：

```nginx
server {
    listen 80;
    server_name your-domain.com;
    
    location / {
        proxy_pass http://127.0.0.1:8000;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

## 安全相关

### Q: 如何保护 API 端点？

**A**: 
1. **使用认证中间件**:
   ```rust
   use rocket::request::{FromRequest, Request, Outcome};
   
   struct ApiKey(String);
   
   #[rocket::async_trait]
   impl<'r> FromRequest<'r> for ApiKey {
       type Error = ();
       
       async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
           // 实现认证逻辑
       }
   }
   ```

2. **验证输入数据**:
   ```rust
   #[derive(Deserialize)]
   struct UserInput {
       name: String,
       email: String,
   }
   
   impl UserInput {
       fn validate(&self) -> Result<(), String> {
           if self.name.is_empty() {
               return Err("Name cannot be empty".to_string());
           }
           // 更多验证...
           Ok(())
       }
   }
   ```

### Q: 如何防止 SQL 注入？

**A**: 使用参数化查询和 ORM：

```rust
use diesel::prelude::*;

#[get("/user/<id>")]
fn get_user(conn: DbConn, id: i32) -> Result<Json<User>, Status> {
    users::table
        .filter(users::id.eq(id))
        .first::<User>(&*conn)
        .map(Json)
        .map_err(|_| Status::NotFound)
}
```

## 调试技巧

### Q: 如何调试路由问题？

**A**: 
1. **启用详细日志**:
   ```toml
   [debug]
   log_level = "debug"
   ```

2. **添加调试输出**:
   ```rust
   #[get("/debug")]
   fn debug_route() -> String {
       println!("Debug route called");
       "Debug response".to_string()
   }
   ```

3. **使用 Rocket 的调试功能**:
   ```rust
   #[launch]
   fn rocket() -> _ {
       rocket::build()
           .mount("/", routes![/* your routes */])
           .attach(rocket::fairing::AdHoc::on_ignite("Debug", |rocket| async {
               println!("Rocket launched with routes: {:?}", rocket.routes());
               Ok(rocket)
           }))
   }
   ```

### Q: 如何查看请求详情？

**A**: 使用中间件记录请求：

```rust
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Request, Response, Data};

pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "Request Logger",
            kind: Kind::Request,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        println!("Request: {} {}", request.method(), request.uri());
    }
}
```

## 社区和支持

### Q: 在哪里可以获得帮助？

**A**: 
1. **GitHub Issues**: 报告 bug 和功能请求
2. **Discussions**: 参与社区讨论
3. **Rocket 官方文档**: [rocket.rs](https://rocket.rs/)
4. **Rust 社区**: [users.rust-lang.org](https://users.rust-lang.org/)

### Q: 如何贡献代码？

**A**: 
1. Fork 项目
2. 创建功能分支
3. 提交更改
4. 创建 Pull Request

### Q: 如何报告 bug？

**A**: 在 GitHub Issues 中提供：
1. 详细的错误描述
2. 复现步骤
3. 系统环境信息
4. 相关日志

---

*最后更新: 2024年*

如果您的问题未在此 FAQ 中找到答案，请：
1. 查看 [GitHub Issues](../../issues)
2. 参与 [Discussions](../../discussions)
3. 阅读 [Rocket 官方文档](https://rocket.rs/)
