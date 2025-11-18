在 Windows 系统上使用 SQLite，可以按照以下步骤进行安装、配置和使用：

一、什么是 SQLite？

SQLite 是一个轻量级的嵌入式数据库，它不需要独立的服务器进程，数据通常存储在一个单一的文件中（如 todos.db）。非常适合小型应用、原型开发或学习用途。

二、在 Windows 上使用 SQLite 的方法

方法 1：通过命令行工具（sqlite3.exe）直接操作数据库文件

1. 下载 SQLite 命令行工具

• 访问官网下载页面：https://www.sqlite.org/download.html

• 找到 Precompiled Binaries for Windows 部分

• 下载如下文件（以最新版为例）：

  • sqlite-tools-win32-x86-*.zip （包含 sqlite3.exe 命令行工具）

例如：sqlite-tools-win32-x86-3470200.zip

2. 解压文件

将下载的 ZIP 文件解压到一个目录，比如：

C:\sqlite\


确保该目录中包含 sqlite3.exe 这个可执行文件。

3. 配置环境变量（可选，但推荐）

为了在任何目录下都能使用 sqlite3 命令，可以将 C:\sqlite\ 添加到系统的 PATH 环境变量中：

• 右键“此电脑” → “属性” → “高级系统设置” → “环境变量”

• 在“系统变量”中找到 Path，点击编辑 → 新建，添加：

  C:\sqlite\
  

• 保存所有窗口。

4. 使用 sqlite3 命令行工具

打开 命令提示符（cmd） 或 PowerShell，输入：
sqlite3


如果你看到类似如下的提示符：
SQLite version 3.47.2 2024-01-16 11:52:41
Enter ".help" for usage hints.
Connected to a transient in-memory database.
Use ".open FILENAME" to reopen on a persistent database.
sqlite>


说明你已经成功进入 SQLite 命令行界面。

打开或创建一个数据库文件（如 todos.db）

.open todos.db


如果 todos.db 文件不存在，SQLite 会自动创建它。

你可以开始执行 SQL 语句，例如：
CREATE TABLE todos (id INTEGER PRIMARY KEY, task TEXT);
INSERT INTO todos (task) VALUES ('Buy groceries');
SELECT * FROM todos;


要退出 sqlite3，输入：
.quit


方法 2：在 Rust 项目中使用 SQLite（如你给出的命令）

你运行的命令是：
$env:DATABASE_URL="sqlite:./database/todos.db"; cargo check


这是在 Windows PowerShell 中为 Rust 项目设置一个环境变量 DATABASE_URL，指向一个本地的 SQLite 数据库文件 ./database/todos.db，然后运行 cargo check 检查代码。

要让这个真正工作起来，你需要：

1. 确保你的 Rust 项目使用了某个支持 SQLite 的 ORM 或数据库连接库

   比如常用的有：

   • https://diesel.rs/（需要额外配置 SQLite 支持）

   • https://github.com/launchbadge/sqlx（推荐，支持异步，需编译时检查或运行时）

   • https://github.com/rusqlite/rusqlite（纯 Rust 的 SQLite 库，非常轻量和易用）

2. 确保目录结构存在

   你设置的路径是 ./database/todos.db，也就是项目根目录下的 database 文件夹中的 todos.db 文件。

   如果 database 文件夹不存在，程序可能会报错。你可以手动创建它：
   mkdir database
   

   或者在 Rust 代码中使用 std::fs::create_dir_all 自动创建。

3. 使用 rusqlite 的简单例子（如果你不用 ORM）

   如果你只是想用 SQLite 而不依赖重型框架，可以使用 https://crates.io/crates/rusqlite crate：

   Cargo.toml:
   [dependencies]
   rusqlite = "0.30"
   

   main.rs:
   use rusqlite::{params, Connection, Result};

   fn main() -> Result<()> {
       let conn = Connection::open("database/todos.db")?;

       conn.execute(
           "CREATE TABLE IF NOT EXISTS todos (
                id              INTEGER PRIMARY KEY,
                task            TEXT NOT NULL
            )",
           [],
       )?;

       conn.execute(
           "INSERT INTO todos (task) VALUES (?1)",
           params!("Learn SQLite on Windows"),
       )?;

       let mut stmt = conn.prepare("SELECT id, task FROM todos")?;
       let rows = stmt.query_map([], |row| {
           Ok((
               row.get::<_, i32>(0)?,
               row.get::<_, String>(1)?,
           ))
       })?;

       for row in rows {
           let (id, task) = row?;
           println!("ID: {}, Task: {}", id, task);
       }

       Ok(())
   }
   

   运行后会在项目下生成一个 database/todos.db 文件，存储你的数据。

三、图形化工具（推荐用于查看/管理 SQLite 数据库）

如果你不想一直用命令行，可以使用一些 SQLite 图形化管理工具，比如：

1. https://sqlitebrowser.org/dl/（强烈推荐）
   • 支持 Windows / macOS / Linux

   • 可以打开 .db 文件，直观地浏览表、数据，执行 SQL 语句

   • 下载地址：https://sqlitebrowser.org/dl/

2. 其他工具如 SQLiteStudio, HeidiSQL（也支持其他数据库）等

四、总结：在 Windows 上使用 SQLite 的基本步骤

场景 方法

命令行操作 SQLite 数据库 下载 sqlite3.exe，使用 sqlite3 todos.db 进入交互式命令行

在 Rust 项目中使用 SQLite 使用 rusqlite 或 sqlx 等库，配置好数据库路径如 sqlite:./database/todos.db

图形化管理 SQLite 数据库 使用 DB Browser for SQLite 打开 .db 文件进行可视化管理

创建数据库文件 SQLite 会在你首次连接（如 todos.db）时自动创建该文件

五、常见问题

Q1: 我的 Rust 项目报错找不到数据库文件？

• 确保路径正确，比如 ./database/todos.db 是相对于程序运行目录的。

• 确保 database 文件夹存在，或者代码中先创建文件夹。

Q2: 如何用 PowerShell 设置环境变量并运行程序？

你原来的命令：
$env:DATABASE_URL="sqlite:./database/todos.db"; cargo check


这是正确的，它只在当前 PowerShell 会话中临时设置环境变量 DATABASE_URL，然后运行 cargo check。如果要在运行程序（比如 cargo run）时也使用它，同样可以这样设置。

✅ 推荐入门流程（针对你的情况）：

1. 安装 https://sqlitebrowser.org/dl/（方便查看数据库文件）
2. 在 Rust 项目中使用 rusqlite 或 sqlx 连接 SQLite
3. 设置数据库路径如 sqlite:./database/todos.db
4. 确保该目录存在，或让代码自动创建
5. 运行程序，它会自动创建并操作 todos.db

如果你愿意告诉我你具体是在用什么库（比如 diesel、sqlx、rusqlite），我可以给你更精确的代码示例和配置帮助！