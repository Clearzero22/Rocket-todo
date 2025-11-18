# API 文档

本文档详细描述了 My Rocket App 的所有 API 端点。

## 基础信息

- **Base URL**: `http://127.0.0.1:8000`
- **Content-Type**: `application/json` (对于 JSON 端点)
- **框架**: Rocket 0.5.1

## 端点列表

### 1. 欢迎页面

**GET** `/`

返回欢迎消息。

**响应:**
```
Hello, Rocket!
```

**示例:**
```bash
curl http://127.0.0.1:8000/
```

---

### 2. 应用配置

**GET** `/config`

获取应用配置信息。

**响应:**
```
App Config Route
```

**示例:**
```bash
curl http://127.0.0.1:8000/config
```

---

### 3. 延迟响应

**GET** `/delay/<seconds>`

异步延迟响应，用于测试异步功能。

**路径参数:**
- `seconds` (u64): 延迟秒数

**响应:**
```
Delayed for {seconds} seconds
```

**示例:**
```bash
curl http://127.0.0.1:8000/delay/5
```

---

### 4. 搜索功能

**GET** `/search?<query>&<page>`

处理搜索查询和分页。

**查询参数:**
- `query` (String): 搜索关键词
- `page` (u32): 页码

**响应:**
```
Search query: {query}, Page: {page}
```

**示例:**
```bash
curl "http://127.0.0.1:8000/search?query=rust&page=1"
```

---

### 5. 个性化问候

**GET** `/hello/<name>/<age>/<cool>`

根据用户信息返回个性化问候。

**路径参数:**
- `name` (String): 用户姓名
- `age` (u8): 用户年龄
- `cool` (bool): 是否酷炫

**响应:**
- 如果 `cool` 为 `true`: `You're a cool {age} year old, {name}!`
- 如果 `cool` 为 `false`: `{name}, we need to talk about your coolness.`

**示例:**
```bash
curl http://127.0.0.1:8000/hello/Alice/25/true
curl http://127.0.0.1:8000/hello/Bob/30/false
```

---

### 6. 路径显示

**GET** `/page/<path..>`

显示请求的路径信息。

**路径参数:**
- `path` (PathBuf): 任意路径

**响应:**
```
Path: {path}
```

**示例:**
```bash
curl http://127.0.0.1:8000/page/docs/api/users
```

---

### 7. 用户管理

#### 7.1 获取用户信息

**GET** `/user/<id>`

根据用户 ID 获取用户信息。支持多种数据类型，按优先级排序：

1. `usize` (默认)
2. `usize` (rank = 2)
3. `String` (rank = 3)

**路径参数:**
- `id`: 用户 ID

**响应 (JSON):**
```json
{
  "id": 123,
  "name": "User-123",
  "age": 23
}
```

**示例:**
```bash
curl http://127.0.0.1:8000/user/123
```

#### 7.2 创建用户

**POST** `/user`

创建新用户。

**请求体 (JSON):**
```json
{
  "id": 123,
  "name": "John Doe",
  "age": 25
}
```

**响应:**
- 状态码: `201 Created`
- 响应体: `✅ 用户已创建（模拟）`

**示例:**
```bash
curl -X POST http://127.0.0.1:8000/user \
  -H "Content-Type: application/json" \
  -d '{"id": 123, "name": "John Doe", "age": 25}'
```

---

### 8. 任务管理

#### 8.1 删除任务

**DELETE** `/delete/task/<id>`

删除指定 ID 的任务。

**路径参数:**
- `id` (String): 任务 ID

**响应:**
```
Deleted task with ID: {id}
```

**示例:**
```bash
curl -X DELETE http://127.0.0.1:8000/delete/task/task-123
```

---

## 数据模型

### User 结构体

```rust
struct User {
    id: usize,      // 用户 ID
    name: String,   // 用户姓名
    age: u8,        // 用户年龄
}
```

### AppConfig 结构体

```rust
struct AppConfig {
    my_app_name: String,     // 应用名称
    max_file_size: String,   // 最大文件大小
}
```

## 错误处理

应用使用标准的 HTTP 状态码：

- `200 OK`: 请求成功
- `201 Created`: 资源创建成功
- `400 Bad Request`: 请求参数错误
- `404 Not Found`: 资源未找到
- `500 Internal Server Error`: 服务器内部错误

## 路由优先级

Rocket 使用路由优先级系统来处理冲突的路由：

1. **rank 参数**: 数值越小，优先级越高
2. **类型匹配**: 更具体的类型优先级更高
3. **路径长度**: 更长的路径优先级更高

示例：
```rust
#[get("/user/<id>")]           // rank = 1 (默认)
fn user(id: usize) -> String { ... }

#[get("/user/<id>", rank = 2)]
fn user_int(id: usize) -> String { ... }

#[get("/user/<id>", rank = 3)]
fn user_str(id: String) -> String { ... }
```

## 异步支持

应用支持异步请求处理：

```rust
#[get("/delay/<seconds>")]
async fn delay(seconds: u64) -> String {
    sleep(Duration::from_secs(seconds)).await;
    format!("Delayed for {} seconds", seconds)
}
```

## 内容协商

支持多种内容类型：

- **JSON**: `Content-Type: application/json`
- **文本**: `Content-Type: text/plain`
- **HTML**: `Content-Type: text/html`

## 示例客户端代码

### JavaScript (Fetch API)

```javascript
// 获取用户信息
async function getUser(id) {
    const response = await fetch(`http://127.0.0.1:8000/user/${id}`);
    return await response.json();
}

// 创建用户
async function createUser(user) {
    const response = await fetch('http://127.0.0.1:8000/user', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: JSON.stringify(user)
    });
    return await response.text();
}
```

### Python (requests)

```python
import requests

# 获取用户信息
def get_user(user_id):
    response = requests.get(f'http://127.0.0.1:8000/user/{user_id}')
    return response.json()

# 创建用户
def create_user(user_data):
    response = requests.post(
        'http://127.0.0.1:8000/user',
        json=user_data
    )
    return response.text
```

### cURL 示例

```bash
# 获取所有端点
curl http://127.0.0.1:8000/

# 搜索
curl "http://127.0.0.1:8000/search?query=test&page=1"

# 创建用户
curl -X POST http://127.0.0.1:8000/user \
  -H "Content-Type: application/json" \
  -d '{"id": 1, "name": "Test User", "age": 30}'

# 删除任务
curl -X DELETE http://127.0.0.1:8000/delete/task/task-123
```

---

*最后更新: 2024年*
