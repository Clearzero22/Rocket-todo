# 安装和设置指南

本指南将帮助您快速设置和运行 My Rocket App。

## 系统要求

### 最低要求

- **操作系统**: Windows 10+, macOS 10.15+, 或 Linux (Ubuntu 18.04+)
- **内存**: 至少 2GB RAM
- **存储**: 至少 1GB 可用空间
- **网络**: 互联网连接（用于下载依赖）

### 推荐配置

- **操作系统**: Windows 11, macOS 12+, 或 Ubuntu 20.04+
- **内存**: 4GB+ RAM
- **存储**: 5GB+ 可用空间
- **处理器**: 多核处理器

## 安装步骤

### 第一步：安装 Rust

#### Windows

1. **下载 Rust 安装程序**
   - 访问 [rustup.rs](https://rustup.rs/)
   - 点击 "Download rustup-init.exe"

2. **运行安装程序**
   ```cmd
   # 下载完成后，运行安装程序
   rustup-init.exe
   ```

3. **选择安装选项**
   - 选择 "1) Proceed with installation (default)"
   - 等待安装完成

4. **验证安装**
   ```cmd
   rustc --version
   cargo --version
   ```

#### macOS

1. **使用 Homebrew 安装**
   ```bash
   # 安装 Homebrew（如果未安装）
   /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"
   
   # 安装 Rust
   brew install rust
   ```

2. **或使用官方安装脚本**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

3. **验证安装**
   ```bash
   rustc --version
   cargo --version
   ```

#### Linux (Ubuntu/Debian)

1. **更新包管理器**
   ```bash
   sudo apt update
   sudo apt upgrade -y
   ```

2. **安装必要工具**
   ```bash
   sudo apt install -y curl build-essential
   ```

3. **安装 Rust**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   ```

4. **验证安装**
   ```bash
   rustc --version
   cargo --version
   ```

### 第二步：获取项目代码

#### 方法 1：克隆仓库

```bash
# 克隆项目
git clone <repository-url>
cd my-rocket-app
```

#### 方法 2：下载 ZIP

1. 访问项目页面
2. 点击 "Code" -> "Download ZIP"
3. 解压到本地目录
4. 打开终端，进入项目目录

### 第三步：安装项目依赖

```bash
# 进入项目目录
cd my-rocket-app

# 安装依赖（这会自动下载并编译所有依赖）
cargo build
```

### 第四步：配置应用

#### 1. 检查配置文件

确保 `Rocket.toml` 文件存在且配置正确：

```toml
[default]
address = "127.0.0.1"
port = 8000
log_level = "normal"
my_app_name = "My Awesome Rocket App"
max_file_size = "5 MiB"

[debug]
log_level = "debug"
secret_key = "a-very-secret-key-for-development"

[release]
log_level = "critical"
```

#### 2. 设置环境变量（可选）

```bash
# Windows (PowerShell)
$env:SECRET_KEY = "your-secret-key-here"

# Windows (CMD)
set SECRET_KEY=your-secret-key-here

# Linux/macOS
export SECRET_KEY="your-secret-key-here"
```

### 第五步：运行应用

#### 开发模式

```bash
# 启动开发服务器
cargo run
```

您应该看到类似输出：
```
🚀 Rocket has launched from http://127.0.0.1:8000
```

#### 发布模式

```bash
# 启动优化版本
cargo run --release
```

### 第六步：验证安装

#### 1. 测试基础端点

```bash
# 测试欢迎页面
curl http://127.0.0.1:8000/

# 预期输出: Hello, Rocket!
```

#### 2. 测试 API 端点

```bash
# 测试用户端点
curl http://127.0.0.1:8000/user/123

# 预期输出: {"id":123,"name":"User-123","age":23}
```

#### 3. 使用浏览器

打开浏览器访问: http://127.0.0.1:8000

## 开发工具设置

### Visual Studio Code

#### 1. 安装 Rust 扩展

1. 打开 VS Code
2. 按 `Ctrl+Shift+X` 打开扩展面板
3. 搜索 "rust-analyzer"
4. 点击安装

#### 2. 配置设置

创建 `.vscode/settings.json`：

```json
{
    "rust-analyzer.checkOnSave.command": "clippy",
    "rust-analyzer.cargo.features": "all",
    "editor.formatOnSave": true,
    "rust-analyzer.procMacro.enable": true
}
```

### IntelliJ IDEA

#### 1. 安装 Rust 插件

1. 打开 IntelliJ IDEA
2. 进入 File -> Settings -> Plugins
3. 搜索 "Rust"
4. 安装 Rust 插件

#### 2. 配置 Rust 工具链

1. 进入 File -> Settings -> Languages & Frameworks -> Rust
2. 设置 Rust toolchain 路径
3. 启用 Cargo 集成

## 常见问题解决

### 问题 1：Rust 安装失败

**症状**: `rustup` 命令未找到

**解决方案**:
```bash
# 重新安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### 问题 2：编译错误

**症状**: `cargo build` 失败

**解决方案**:
```bash
# 更新 Rust 工具链
rustup update

# 清理并重新构建
cargo clean
cargo build
```

### 问题 3：端口被占用

**症状**: `Address already in use`

**解决方案**:
1. 修改 `Rocket.toml` 中的端口：
   ```toml
   [default]
   port = 8001
   ```

2. 或终止占用端口的进程：
   ```bash
   # Windows
   netstat -ano | findstr :8000
   taskkill /PID <PID> /F
   
   # Linux/macOS
   lsof -ti:8000 | xargs kill -9
   ```

### 问题 4：依赖下载失败

**症状**: 网络超时或连接失败

**解决方案**:
```bash
# 使用国内镜像源
export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup

# 或使用代理
export https_proxy=http://proxy.example.com:8080
```

### 问题 5：权限错误

**症状**: 权限被拒绝

**解决方案**:
```bash
# Linux/macOS
sudo chown -R $USER:$USER ~/.cargo
chmod -R 755 ~/.cargo

# Windows (以管理员身份运行)
```

## 性能优化

### 编译优化

```bash
# 使用发布模式编译
cargo build --release

# 启用链接时优化
# 在 Cargo.toml 中添加：
[profile.release]
lto = true
codegen-units = 1
panic = "abort"
```

### 运行时优化

```toml
# Rocket.toml
[release]
workers = 4  # 根据 CPU 核心数调整
log_level = "critical"
```

## 生产环境部署

### Docker 部署

#### 1. 创建 Dockerfile

```dockerfile
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

#### 2. 构建和运行

```bash
# 构建镜像
docker build -t my-rocket-app .

# 运行容器
docker run -p 8000:8000 my-rocket-app
```

### 系统服务部署

#### 1. 创建 systemd 服务

```ini
# /etc/systemd/system/my-rocket-app.service
[Unit]
Description=My Rocket App
After=network.target

[Service]
Type=simple
User=rocket
WorkingDirectory=/opt/my-rocket-app
ExecStart=/opt/my-rocket-app/target/release/my-rocket-app
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
```

#### 2. 启动服务

```bash
# 重新加载 systemd
sudo systemctl daemon-reload

# 启动服务
sudo systemctl start my-rocket-app

# 设置开机自启
sudo systemctl enable my-rocket-app
```

## 监控和日志

### 日志配置

```toml
# Rocket.toml
[default]
log_level = "info"
log_file = "/var/log/my-rocket-app/app.log"
```

### 健康检查

```bash
# 检查应用状态
curl http://127.0.0.1:8000/

# 检查配置
curl http://127.0.0.1:8000/config
```

## 下一步

安装完成后，您可以：

1. 阅读 [API 文档](API.md) 了解所有端点
2. 查看 [开发指南](DEVELOPMENT.md) 开始开发
3. 参考 [配置指南](CONFIGURATION.md) 自定义配置
4. 运行测试确保一切正常

## 获取帮助

如果遇到问题：

1. 查看 [常见问题](FAQ.md)
2. 检查 [GitHub Issues](../../issues)
3. 参与 [Discussions](../../discussions)
4. 阅读 [Rocket 官方文档](https://rocket.rs/)

---

*最后更新: 2024年*
