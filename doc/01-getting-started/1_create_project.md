# 项目创建指南

> **注意**: 这是一个基础的项目创建指南。完整的安装和设置说明请参考 [安装指南](../docs/SETUP.md)。

## 快速开始

### 第一步：安装 Rust

如果你还没有安装 Rust，请先安装它。Rocket v0.5 需要的最低版本是 Rust 1.64，但安装最新的稳定版总是最好的选择。

访问 [rustup.rs](https://rustup.rs/) 并按照页面上的指令进行安装。安装完成后，在终端运行以下命令来确保安装成功：

```sh
rustc --version
cargo --version
```

### 第二步：创建新的 Rust 项目

使用 Cargo（Rust 的包管理器和构建工具）来创建一个新的二进制项目。我们给项目起个名字，比如 `my-rocket-app`：

```sh
cargo new my-rocket-app
cd my-rocket-app
```

这个命令会创建一个新的目录 `my-rocket-app`，里面包含一个基本的 Rust 项目结构，包括一个 `src/main.rs` 文件和 `Cargo.toml` 配置文件。

### 第三步：添加 Rocket 依赖

现在，我们需要告诉 Cargo 我们的项目依赖于 Rocket。打开项目根目录下的 `Cargo.toml` 文件，在 `[dependencies]` 部分添加 Rocket：

```toml
[dependencies]
rocket = { version = "0.5.1", features = ["json"] }
```

**解释一下：**
- `rocket = "0.5.1"`：指定了我们要使用的 Rocket 版本。这是目前最新的稳定版。
- `features = ["json"]`：这是一个非常重要的部分！Rocket 的许多功能是可选的，通过 `features` 标志来启用。这里我们启用了 `json` 功能，这样我们就可以轻松地处理 JSON 数据了。未来如果你需要数据库、模板等功能，也要在这里添加相应的 `features`。

### 第四步：编写你的第一个 Rocket 应用

现在，让我们把 `src/main.rs` 文件里的默认内容替换成一个简单的 "Hello, world!" Rocket 应用。

打开 `src/main.rs`，写入以下代码：

```rust
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, Rocket!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
```

**代码解读：**
- `#[macro_use] extern crate rocket;`：这行代码导入了 Rocket 的宏，让我们可以在代码中使用像 `#[get]` 和 `routes!` 这样的宏。
- `#[get("/")]`：这是一个**路由宏**。它告诉 Rocket：当收到一个 `GET` 请求，并且路径是根目录 `/` 时，就执行下面的 `index` 函数。
- `fn index() -> &'static str`：这是我们的**请求处理器**。它返回一个静态字符串切片，Rocket 会自动将其作为 HTTP 响应体，并设置合适的 `Content-Type`。
- `#[launch]`：这是一个便利宏，它会自动创建一个 `main` 函数，并在其中启动我们的 Rocket 应用。
- `fn rocket() -> _`：这是应用的构建函数。
  - `rocket::build()`：创建一个新的 Rocket 实例。
  - `.mount("/", routes![index])`：将 `index` 路由挂载到根路径 `/` 上。

### 第五步：运行你的应用

一切就绪！回到你的终端（确保你在 `my-rocket-app` 目录下），运行以下命令：

```sh
cargo run
```

Cargo 会自动下载并编译 Rocket 和它的依赖，然后启动你的应用。你应该会看到类似下面的输出：

```text
🚀 Rocket has launched from http://127.0.0.1:8000
```

现在，打开你的浏览器，访问 `http://127.0.0.1:8000`，或者使用 `curl` 命令：

```sh
curl http://127.0.0.1:8000
```

你将会看到页面上显示着：

```text
Hello, Rocket!
```

恭喜！你已经成功创建并运行了你的第一个 Rocket 后端应用！

## 接下来做什么？

现在你有了一个可以运行的基础，你可以开始探索更多 Rocket 的强大功能了。这里有一些你可以尝试的方向：

1. **添加更多路由**：尝试添加一个新的路由，比如 `#[get("/hello/<name>")]`，来个性化问候。
2. **处理 JSON**：创建一个 `POST /api` 路由，接收并解析 JSON 请求体，然后返回一个 JSON 响应。
3. **查询参数**：在路由中处理查询字符串，例如 `GET /search?q=rocket`。
4. **阅读官方指南**：Rocket 的官方指南写得非常出色，是学习的最佳资源。

## 更多资源

- [完整安装指南](../docs/SETUP.md) - 详细的安装和配置说明
- [API 文档](../docs/API.md) - 所有端点的详细说明
- [开发指南](../docs/DEVELOPMENT.md) - 开发最佳实践和技巧
- [配置指南](../docs/CONFIGURATION.md) - 配置选项详解
- [常见问题](../docs/FAQ.md) - 常见问题解答

如果你在开发过程中遇到任何问题，别忘了还有社区聊天频道可以寻求帮助。

祝你开发愉快！

---

*最后更新: 2024年*
