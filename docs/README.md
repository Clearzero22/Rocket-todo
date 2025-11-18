# 文档索引

欢迎来到 My Rocket App 的文档中心！这里包含了项目的完整文档，帮助您快速上手和深入开发。

## 📚 文档概览

### 🚀 快速开始

| 文档 | 描述 | 适用人群 |
|------|------|----------|
| [项目创建指南](../doc/1_create_project.md) | 从零开始创建 Rocket 项目 | 初学者 |
| [安装指南](SETUP.md) | 详细的安装和配置说明 | 所有用户 |
| [README](../README.md) | 项目概览和快速入门 | 所有用户 |

### 📖 核心文档

| 文档 | 描述 | 内容 |
|------|------|------|
| [API 文档](API.md) | 完整的 API 端点说明 | 所有端点、参数、响应格式 |
| [开发指南](DEVELOPMENT.md) | 开发最佳实践和技巧 | 代码组织、测试、调试 |
| [配置指南](CONFIGURATION.md) | 配置选项详解 | 环境配置、安全设置 |

### ❓ 帮助和支持

| 文档 | 描述 | 内容 |
|------|------|------|
| [常见问题](FAQ.md) | 常见问题解答 | 安装、配置、开发问题 |

## 🎯 按使用场景导航

### 我是新用户
1. 阅读 [README](../README.md) 了解项目
2. 按照 [安装指南](SETUP.md) 设置环境
3. 参考 [项目创建指南](../doc/1_create_project.md) 创建项目
4. 查看 [API 文档](API.md) 了解功能

### 我要开发功能
1. 阅读 [开发指南](DEVELOPMENT.md) 了解最佳实践
2. 参考 [API 文档](API.md) 了解现有端点
3. 查看 [配置指南](CONFIGURATION.md) 进行环境配置
4. 遇到问题时查看 [常见问题](FAQ.md)

### 我要部署应用
1. 阅读 [配置指南](CONFIGURATION.md) 的生产环境部分
2. 参考 [安装指南](SETUP.md) 的部署章节
3. 查看 [常见问题](FAQ.md) 的部署相关问题

### 我要贡献代码
1. 阅读 [开发指南](DEVELOPMENT.md) 了解代码规范
2. 查看 [API 文档](API.md) 了解现有功能
3. 参考 [常见问题](FAQ.md) 了解常见开发问题

## 📋 文档结构

```
docs/
├── README.md              # 文档索引（本文件）
├── API.md                 # API 文档
├── CONFIGURATION.md       # 配置指南
├── DEVELOPMENT.md         # 开发指南
├── SETUP.md              # 安装指南
└── FAQ.md                # 常见问题

doc/
└── 1_create_project.md   # 项目创建指南
```

## 🔍 快速查找

### 按功能查找

**安装和设置**
- [安装 Rust](SETUP.md#安装-rust)
- [配置应用](CONFIGURATION.md#基础配置)
- [环境变量设置](CONFIGURATION.md#环境变量配置)

**API 开发**
- [路由定义](DEVELOPMENT.md#路由开发)
- [JSON 处理](DEVELOPMENT.md#请求体处理)
- [异步编程](DEVELOPMENT.md#异步编程)

**配置管理**
- [服务器配置](CONFIGURATION.md#服务器设置)
- [日志配置](CONFIGURATION.md#日志配置)
- [安全配置](CONFIGURATION.md#安全配置)

**测试和调试**
- [单元测试](DEVELOPMENT.md#单元测试)
- [集成测试](DEVELOPMENT.md#集成测试)
- [调试技巧](DEVELOPMENT.md#调试技巧)

### 按问题类型查找

**安装问题**
- [Rust 安装失败](FAQ.md#问题-1rust-安装失败)
- [编译错误](FAQ.md#问题-2编译错误)
- [端口被占用](FAQ.md#问题-3端口被占用)

**开发问题**
- [如何添加路由](FAQ.md#q-如何添加新的路由)
- [如何处理 POST 请求](FAQ.md#q-如何处理-post-请求)
- [如何返回 JSON](FAQ.md#q-如何返回-json-响应)

**配置问题**
- [配置文件不生效](FAQ.md#q-配置文件不生效怎么办)
- [如何修改端口](FAQ.md#q-如何修改服务器端口)
- [环境变量设置](FAQ.md#q-如何设置环境变量)

## 📝 文档贡献

如果您发现文档有错误或需要改进，欢迎：

1. **提交 Issue**: 报告文档问题
2. **创建 Pull Request**: 直接改进文档
3. **参与讨论**: 在 Discussions 中提出建议

### 文档编写规范

- 使用 Markdown 格式
- 保持中文文档的一致性
- 提供清晰的代码示例
- 包含必要的截图和图表
- 定期更新过时信息

## 🔗 外部资源

### Rocket 官方资源
- [Rocket 官网](https://rocket.rs/)
- [Rocket 指南](https://rocket.rs/v0.5/guide/)
- [Rocket API 文档](https://api.rocket.rs/v0.5/rocket/)

### Rust 资源
- [Rust 官网](https://www.rust-lang.org/)
- [Rust 书](https://doc.rust-lang.org/book/)
- [Rust 标准库](https://doc.rust-lang.org/std/)

### 社区资源
- [Rust 用户论坛](https://users.rust-lang.org/)
- [Rocket 聊天频道](https://chat.mozilla.org/#/room/#rocket:mozilla.org)
- [Stack Overflow](https://stackoverflow.com/questions/tagged/rust+rocket)

## 📊 文档统计

- **总文档数**: 6 个
- **总字数**: 约 50,000+ 字
- **代码示例**: 100+ 个
- **最后更新**: 2024年

## 🎉 开始使用

现在您已经了解了文档结构，可以选择适合您的文档开始阅读：

- **新手**: 从 [README](../README.md) 开始
- **开发者**: 直接查看 [API 文档](API.md)
- **运维**: 参考 [配置指南](CONFIGURATION.md)

祝您使用愉快！如果遇到任何问题，请随时查看 [常见问题](FAQ.md) 或提交 Issue。

---

*最后更新: 2024年*
