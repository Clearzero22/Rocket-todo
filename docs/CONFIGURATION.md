# 配置指南

本文档详细说明了 My Rocket App 的配置选项和设置方法。

## 配置文件

应用使用 `Rocket.toml` 文件进行配置，支持多环境配置。

## 配置结构

```toml
[default]    # 默认配置
[debug]      # 开发环境配置
[release]    # 生产环境配置
```

## 配置选项详解

### 基础配置

#### 服务器设置

```toml
[default]
address = "127.0.0.1"    # 服务器监听地址
port = 8000             # 服务器端口
```

#### 日志配置

```toml
[default]
log_level = "normal"    # 日志级别: critical, error, warn, info, debug, trace
```

**日志级别说明:**
- `critical`: 只记录关键错误
- `error`: 记录错误信息
- `warn`: 记录警告和错误
- `info`: 记录一般信息、警告和错误
- `debug`: 记录调试信息（开发环境推荐）
- `trace`: 记录所有信息（最详细）

### 环境特定配置

#### 开发环境 (debug)

```toml
[debug]
# 继承 [default] 的所有配置
log_level = "debug"                    # 启用详细日志
secret_key = "a-very-secret-key-for-development"  # 开发环境密钥
```

**开发环境特点:**
- 启用详细调试日志
- 使用固定的开发密钥
- 热重载支持
- 详细的错误信息

#### 生产环境 (release)

```toml
[release]
# 继承 [default] 的所有配置
log_level = "critical"                 # 只记录关键错误
# secret_key = "${SECRET_KEY}"        # 从环境变量读取密钥
```

**生产环境特点:**
- 最小化日志输出
- 从环境变量读取敏感配置
- 优化的性能设置
- 安全的默认配置

### 自定义配置

#### 应用特定配置

```toml
[default]
# 自定义应用配置
my_app_name = "My Awesome Rocket App"  # 应用名称
max_file_size = "5 MiB"               # 最大文件大小
```

#### 数据库配置示例

```toml
[default]
database_url = "sqlite://./app.db"

[debug]
database_url = "sqlite://./debug.db"

[release]
database_url = "${DATABASE_URL}"
```

#### Redis 配置示例

```toml
[default]
redis_url = "redis://127.0.0.1:6379"

[release]
redis_url = "${REDIS_URL}"
```

## 环境变量配置

### 设置环境变量

#### Windows (PowerShell)

```powershell
# 设置环境变量
$env:SECRET_KEY = "your-secret-key-here"
$env:DATABASE_URL = "postgresql://user:pass@localhost/db"

# 运行应用
cargo run
```

#### Windows (CMD)

```cmd
# 设置环境变量
set SECRET_KEY=your-secret-key-here
set DATABASE_URL=postgresql://user:pass@localhost/db

# 运行应用
cargo run
```

#### Linux/macOS

```bash
# 设置环境变量
export SECRET_KEY="your-secret-key-here"
export DATABASE_URL="postgresql://user:pass@localhost/db"

# 运行应用
cargo run
```

### .env 文件支持

创建 `.env` 文件（需要添加 `dotenv` 依赖）：

```bash
# .env 文件
SECRET_KEY=your-secret-key-here
DATABASE_URL=postgresql://user:pass@localhost/db
REDIS_URL=redis://127.0.0.1:6379
```

## 配置验证

### 在代码中读取配置

```rust
use rocket::figment::{Figment, providers::{Format, Toml}};

#[launch]
fn rocket() -> _ {
    let figment = Figment::new()
        .merge(Toml::file("Rocket.toml"))
        .merge(Toml::file("Rocket.toml").nested());

    rocket::build()
        .configure(figment)
        .mount("/", routes![/* your routes */])
}
```

### 配置结构体

```rust
use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct AppConfig {
    my_app_name: String,
    max_file_size: String,
}

// 在路由中使用配置
#[get("/config")]
fn get_config(config: &Config) -> String {
    format!("App: {}", config.my_app_name)
}
```

## 安全配置

### 密钥管理

#### 开发环境

```toml
[debug]
secret_key = "a-very-secret-key-for-development"
```

#### 生产环境

```toml
[release]
secret_key = "${SECRET_KEY}"
```

**生成安全密钥:**
```bash
# 使用 OpenSSL
openssl rand -base64 32

# 使用 Python
python -c "import secrets; print(secrets.token_urlsafe(32))"
```

### HTTPS 配置

```toml
[default]
tls = { certs = "certs/cert.pem", key = "certs/key.pem" }
```

### CORS 配置

```toml
[default]
cors = { origins = ["http://localhost:3000", "https://yourdomain.com"] }
```

## 性能配置

### 连接池设置

```toml
[default]
# 数据库连接池
database_pool_size = 10
database_timeout = 30

# HTTP 连接
max_connections = 1000
keep_alive = 5
```

### 缓存配置

```toml
[default]
# 静态文件缓存
static_cache_control = "public, max-age=31536000"

# 模板缓存
template_cache = true
```

## 监控和日志

### 日志文件配置

```toml
[default]
# 日志文件
log_file = "logs/app.log"
log_max_size = "10 MB"
log_max_files = 5
```

### 指标收集

```toml
[default]
# 性能指标
metrics_enabled = true
metrics_port = 9090
```

## 配置最佳实践

### 1. 环境分离

- 为每个环境创建独立的配置段
- 使用环境变量存储敏感信息
- 避免在代码中硬编码配置

### 2. 配置验证

```rust
use rocket::figment::value::Value;

fn validate_config(config: &Value) -> Result<(), String> {
    // 验证必需配置
    if config.get("secret_key").is_none() {
        return Err("secret_key is required".to_string());
    }
    
    // 验证配置值
    if let Some(port) = config.get("port").and_then(|v| v.as_integer()) {
        if port < 1 || port > 65535 {
            return Err("port must be between 1 and 65535".to_string());
        }
    }
    
    Ok(())
}
```

### 3. 配置文档

- 为每个配置选项添加注释
- 提供配置示例
- 说明配置的影响和用途

### 4. 配置模板

创建配置模板文件：

```toml
# Rocket.toml.template
[default]
address = "127.0.0.1"
port = 8000
log_level = "normal"
secret_key = "${SECRET_KEY}"
my_app_name = "${APP_NAME}"
max_file_size = "${MAX_FILE_SIZE}"

[debug]
log_level = "debug"
secret_key = "dev-secret-key"

[release]
log_level = "critical"
```

## 故障排除

### 常见问题

#### 1. 配置未生效

**问题**: 修改配置后应用未使用新配置

**解决方案**:
- 确保配置文件语法正确
- 重启应用
- 检查环境变量设置

#### 2. 环境变量未读取

**问题**: 环境变量在配置中未生效

**解决方案**:
- 检查环境变量名称和格式
- 确保环境变量已正确设置
- 使用 `${VARIABLE_NAME}` 格式

#### 3. 端口冲突

**问题**: 端口已被占用

**解决方案**:
```toml
[default]
port = 8001  # 使用其他端口
```

### 调试配置

```rust
use rocket::figment::Figment;

#[launch]
fn rocket() -> _ {
    let figment = Figment::new()
        .merge(Toml::file("Rocket.toml"));
    
    // 打印配置用于调试
    println!("Configuration: {:#?}", figment.extract::<Config>().unwrap());
    
    rocket::build()
        .configure(figment)
        .mount("/", routes![/* your routes */])
}
```

## 配置示例

### 完整配置示例

```toml
# Rocket.toml
[default]
# 服务器配置
address = "127.0.0.1"
port = 8000
workers = 4

# 日志配置
log_level = "normal"

# 安全配置
secret_key = "${SECRET_KEY}"

# 应用配置
my_app_name = "My Awesome Rocket App"
max_file_size = "5 MiB"

# 数据库配置
database_url = "${DATABASE_URL}"

# CORS 配置
cors = { origins = ["http://localhost:3000"] }

[debug]
# 开发环境覆盖
log_level = "debug"
secret_key = "dev-secret-key"
database_url = "sqlite://./debug.db"

[release]
# 生产环境覆盖
log_level = "critical"
workers = 8
```

---

*最后更新: 2024年*
