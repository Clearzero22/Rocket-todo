我来为你生成完整的API文档和curl命令，方便在Apifox中导入测试。

## 📚 API 文档

### 基础信息
- **Base URL**: `http://localhost:8000`
- **Content-Type**: `application/json`
- **数据库**: SQLite

### 数据结构

#### Todo 对象
```json
{
  "id": 1,
  "title": "任务标题",
  "description": "任务描述（可选）",
  "status": "pending|in_progress|completed",
  "priority": "low|medium|high",
  "created_at": "2025-10-28T05:51:53Z",
  "updated_at": "2025-10-28T05:51:53Z"
}
```

#### CreateTodoRequest
```json
{
  "title": "任务标题",
  "description": "任务描述（可选）",
  "priority": "low|medium|high（可选，默认medium）",
  "status": "pending|in_progress|completed（可选，默认pending）"
}
```

#### UpdateTodoRequest
```json
{
  "title": "新标题（可选）",
  "description": "新描述（可选）",
  "status": "pending|in_progress|completed（可选）",
  "priority": "low|medium|high（可选）"
}
```

---

## 🔗 API 端点

### 1. 获取API信息
**GET** `/`

**响应示例:**
```json
{
  "message": "Welcome to Todo List API",
  "version": "1.0.0",
  "endpoints": {
    "todos": "/api/todos",
    "health": "/health",
    "docs": "/docs"
  }
}
```

**curl命令:**
```bash
curl -X GET "http://localhost:8000/" \
  -H "Content-Type: application/json"
```

---

### 2. 健康检查
**GET** `/health`

**响应示例:**
```json
{
  "status": "healthy",
  "timestamp": "2025-10-28T05:51:53Z"
}
```

**curl命令:**
```bash
curl -X GET "http://localhost:8000/health" \
  -H "Content-Type: application/json"
```

---

### 3. 获取配置信息
**GET** `/config`

**响应示例:**
```json
{
  "app_name": "Todo List API",
  "version": "1.0.0",
  "features": ["CRUD operations", "Priority levels", "Status filtering"]
}
```

**curl命令:**
```bash
curl -X GET "http://localhost:8000/config" \
  -H "Content-Type: application/json"
```

---

## 📝 Todo 管理 API

### 4. 获取所有Todos
**GET** `/api/todos`

**响应示例:**
```json
[
  {
    "id": 1,
    "title": "学习Rust",
    "description": "完成Rust基础教程",
    "status": "pending",
    "priority": "high",
    "created_at": "2025-10-28T05:51:53Z",
    "updated_at": "2025-10-28T05:51:53Z"
  }
]
```

**curl命令:**
```bash
curl -X GET "http://localhost:8000/api/todos" \
  -H "Content-Type: application/json"
```

---

### 5. 获取特定Todo
**GET** `/api/todos/{id}`

**路径参数:**
- `id` (integer): Todo的ID

**响应示例:**
```json
{
  "id": 1,
  "title": "学习Rust",
  "description": "完成Rust基础教程",
  "status": "pending",
  "priority": "high",
  "created_at": "2025-10-28T05:51:53Z",
  "updated_at": "2025-10-28T05:51:53Z"
}
```

**curl命令:**
```bash
curl -X GET "http://localhost:8000/api/todos/1" \
  -H "Content-Type: application/json"
```

---

### 6. 创建Todo
**POST** `/api/todos`

**请求体:**
```json
{
  "title": "学习Rust",
  "description": "完成Rust基础教程",
  "priority": "high",
  "status": "pending"
}
```

**响应示例:**
```json
{
  "id": 1,
  "title": "学习Rust",
  "description": "完成Rust基础教程",
  "status": "pending",
  "priority": "high",
  "created_at": "2025-10-28T05:51:53Z",
  "updated_at": "2025-10-28T05:51:53Z"
}
```

**curl命令:**
```bash
curl -X POST "http://localhost:8000/api/todos" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "学习Rust",
    "description": "完成Rust基础教程",
    "priority": "high",
    "status": "pending"
  }'
```

**简化版本（只传必需字段）:**
```bash
curl -X POST "http://localhost:8000/api/todos" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "学习Rust"
  }'
```

---

### 7. 更新Todo
**PUT** `/api/todos/{id}`

**路径参数:**
- `id` (integer): Todo的ID

**请求体:**
```json
{
  "title": "学习Rust进阶",
  "description": "完成Rust高级教程",
  "status": "in_progress",
  "priority": "medium"
}
```

**响应示例:**
```json
{
  "id": 1,
  "title": "学习Rust进阶",
  "description": "完成Rust高级教程",
  "status": "in_progress",
  "priority": "medium",
  "created_at": "2025-10-28T05:51:53Z",
  "updated_at": "2025-10-28T05:52:15Z"
}
```

**curl命令:**
```bash
curl -X PUT "http://localhost:8000/api/todos/1" \
  -H "Content-Type: application/json" \
  -d '{
    "title": "学习Rust进阶",
    "status": "in_progress"
  }'
```

---

### 8. 删除Todo
**DELETE** `/api/todos/{id}`

**路径参数:**
- `id` (integer): Todo的ID

**响应:** 204 No Content

**curl命令:**
```bash
curl -X DELETE "http://localhost:8000/api/todos/1" \
  -H "Content-Type: application/json"
```

---

## 🔍 过滤查询 API

### 9. 按状态过滤Todos
**GET** `/api/todos/status/{status}`

**路径参数:**
- `status` (string): 状态值 (`pending`, `in_progress`, `completed`)

**响应示例:**
```json
[
  {
    "id": 1,
    "title": "学习Rust",
    "description": "完成Rust基础教程",
    "status": "pending",
    "priority": "high",
    "created_at": "2025-10-28T05:51:53Z",
    "updated_at": "2025-10-28T05:51:53Z"
  }
]
```

**curl命令:**
```bash
# 获取待处理的任务
curl -X GET "http://localhost:8000/api/todos/status/pending" \
  -H "Content-Type: application/json"

# 获取进行中的任务
curl -X GET "http://localhost:8000/api/todos/status/in_progress" \
  -H "Content-Type: application/json"

# 获取已完成的任务
curl -X GET "http://localhost:8000/api/todos/status/completed" \
  -H "Content-Type: application/json"
```

---

### 10. 按优先级过滤Todos
**GET** `/api/todos/priority/{priority}`

**路径参数:**
- `priority` (string): 优先级值 (`low`, `medium`, `high`)

**响应示例:**
```json
[
  {
    "id": 1,
    "title": "学习Rust",
    "description": "完成Rust基础教程",
    "status": "pending",
    "priority": "high",
    "created_at": "2025-10-28T05:51:53Z",
    "updated_at": "2025-10-28T05:51:53Z"
  }
]
```

**curl命令:**
```bash
# 获取高优先级任务
curl -X GET "http://localhost:8000/api/todos/priority/high" \
  -H "Content-Type: application/json"

# 获取中等优先级任务
curl -X GET "http://localhost:8000/api/todos/priority/medium" \
  -H "Content-Type: application/json"

# 获取低优先级任务
curl -X GET "http://localhost:8000/api/todos/priority/low" \
  -H "Content-Type: application/json"
```

---

## 🚨 错误响应

### 404 Not Found
```json
{
  "error": "Todo not found",
  "message": "Todo not found"
}
```

### 422 Unprocessable Entity
```json
{
  "error": "Failed to create todo",
  "message": "Validation error details"
}
```

### 500 Internal Server Error
```json
{
  "error": "Failed to fetch todos",
  "message": "Database error details"
}
```

---

## 📋 Apifox 导入说明

### 1. 创建新项目
在Apifox中创建一个新项目，命名为"Rocket Todo API"

### 2. 设置环境变量
- 创建环境变量 `baseUrl` = `http://localhost:8000`

### 3. 导入API
将上述curl命令逐个添加到Apifox中，或者使用以下JSON格式导入：

```json
{
  "openapi": "3.0.0",
  "info": {
    "title": "Rocket Todo API",
    "version": "1.0.0",
    "description": "基于Rocket和SQLite的Todo管理API"
  },
  "servers": [
    {
      "url": "http://localhost:8000",
      "description": "开发环境"
    }
  ],
  "paths": {
    "/": {
      "get": {
        "summary": "获取API信息",
        "responses": {
          "200": {
            "description": "成功",
            "content": {
              "application/json": {
                "schema": {
                  "type": "object",
                  "properties": {
                    "message": {"type": "string"},
                    "version": {"type": "string"},
                    "endpoints": {"type": "object"}
                  }
                }
              }
            }
          }
        }
      }
    }
  }
}
```

### 4. 测试流程建议
1. 先测试基础端点（GET /, GET /health）
2. 创建几个测试todos
3. 测试查询和过滤功能
4. 测试更新和删除功能

这样你就可以在Apifox中完整地测试你的Rocket Todo API了！