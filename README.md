# My Rocket App

一个基于 Rust Rocket 框架构建的现代化 Web 后端应用程序，展示了 Rocket 框架的核心功能和最佳实践。

## 🚀 项目特性

- **高性能**: 基于 Rust 和 Rocket 框架，提供卓越的性能和内存安全
- **RESTful API**: 提供完整的 REST API 接口
- **异步支持**: 支持异步请求处理，提高并发性能
- **JSON 支持**: 内置 JSON 序列化和反序列化
- **配置管理**: 灵活的配置系统，支持多环境配置
- **路由系统**: 强大的路由系统，支持路径参数、查询参数和请求体解析

## 📋 功能列表

### 基础路由
- `GET /` - 欢迎页面
- `GET /config` - 应用配置信息

### 用户管理
- `GET /user/<id>` - 获取用户信息（支持多种数据类型）
- `POST /user` - 创建新用户
- `DELETE /delete/task/<id>` - 删除任务

### 实用功能
- `GET /search?<query>&<page>` - 搜索功能
- `GET /delay/<seconds>` - 延迟响应（异步）
- `GET /hello/<name>/<age>/<cool>` - 个性化问候
- `GET /page/<path..>` - 路径显示

## 🛠️ 技术栈

- **Rust**: 系统编程语言
- **Rocket**: Web 框架
- **Serde**: 序列化/反序列化
- **Tokio**: 异步运行时

## 📦 依赖项

```toml
[dependencies]
rocket = { version = "0.5.1", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
```

## 🚀 快速开始

### 前置要求

- Rust 1.64+ (推荐使用最新稳定版)
- Cargo (Rust 包管理器)

### 安装步骤

1. **克隆项目**
   ```bash
   git clone <repository-url>
   cd my-rocket-app
   ```

2. **安装依赖**
   ```bash
   cargo build
   ```

3. **运行应用**
   ```bash
   cargo run
   ```

4. **访问应用**
   打开浏览器访问: http://127.0.0.1:8000

### 开发模式

```bash
# 开发模式运行（启用调试日志）
cargo run

# 发布模式运行（优化性能）
cargo run --release
```

### 🐍 Python 快速开发脚本（可选）

项目根目录提供了一个 `dev.py`，用于加速常见开发任务：

```bash
# 运行应用
python dev.py run

# 自动重载（需安装 cargo-watch）
python dev.py watch  # 安装: cargo install cargo-watch

# 代码检查/格式化/静态检查/测试
python dev.py check
python dev.py fmt
python dev.py clippy
python dev.py test

# 数据库（SQLite）
python dev.py migrate   # 执行 migrations/*.sql
python dev.py reset-db  # 删除数据库并重建
python dev.py seed      # 写入示例数据
```

`dev.py` 会从 `Rocket.toml` 的 `default.databases.sqlite_db.url` 读取数据库地址（如 `sqlite:./database/todos.db`）。

## 📖 API 文档

详细的 API 文档请参考 [API Documentation](docs/API.md)

## ⚙️ 配置

应用配置通过 `Rocket.toml` 文件管理，支持多环境配置：

- **开发环境** (`debug`): 启用详细日志
- **生产环境** (`release`): 优化性能，关键日志
- **默认配置** (`default`): 基础配置

详细配置说明请参考 [Configuration Guide](docs/CONFIGURATION.md)

## 🏗️ 项目结构

```
my-rocket-app/
├── src/
│   └── main.rs          # 主应用文件
├── doc/
│   └── 1_create_project.md  # 项目创建指南
├── Cargo.toml           # 项目配置和依赖
├── Rocket.toml          # Rocket 框架配置
├── .gitignore           # Git 忽略文件
└── README.md            # 项目说明文档
```

## 🔧 开发指南

### 添加新路由

1. 在 `src/main.rs` 中定义处理函数
2. 使用适当的宏装饰器（如 `#[get]`, `#[post]` 等）
3. 在 `routes!` 宏中注册路由

示例：
```rust
#[get("/api/status")]
fn status() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "timestamp": chrono::Utc::now()
    }))
}
```

### 处理 JSON 数据

```rust
#[derive(Serialize, Deserialize)]
struct User {
    id: usize,
    name: String,
    age: u8,
}

#[post("/users", data = "<user>")]
fn create_user(user: Json<User>) -> Status {
    // 处理用户创建逻辑
    Status::Created
}
```

## 🧪 测试

```bash
# 运行测试
cargo test

# 运行特定测试
cargo test test_name
```

## 📝 日志

应用使用 Rocket 内置的日志系统：

- **开发环境**: 详细调试信息
- **生产环境**: 关键错误和警告

## 🤝 贡献

1. Fork 项目
2. 创建功能分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 打开 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

## 📞 支持

如果您遇到任何问题或有任何建议，请：

1. 查看 [FAQ](docs/FAQ.md)
2. 提交 [Issue](../../issues)
3. 参与 [Discussions](../../discussions)

## 🙏 致谢

- [Rocket](https://rocket.rs/) - 优秀的 Rust Web 框架
- [Rust](https://www.rust-lang.org/) - 系统编程语言
- [Serde](https://serde.rs/) - 序列化框架

---

**Happy Coding! 🦀**
