# 贡献指南

> 本仓库暂不接受外部 PR，仅接受内部维护者的 PR。欢迎通过 Issue 反馈 Bug 或功能建议。

## 本地环境初始化

本仓库使用 `mise` 管理工具链、使用 `hk` 编排本地 Git Hooks；首次克隆项目后，在根目录安装项目依赖工具并激活 Git Hooks：

```sh
mise install
mise run hooks
```

激活后，以下 Git Hooks 会自动生效：

- **`pre-commit`**：校验并尝试修复文档规范；执行 GitHub Workflows 安全检查，以及 `cargo fmt`、`ast-grep` 架构检查、`cargo clippy`、`cargo nextest`。
- **`commit-msg`**：校验提交信息是否符合 Conventional Commits 规范。
- **`pre-push`**：当依赖或策略配置变更时执行 `cargo deny` 合规审计。

## Commit 与版本规范

遵循 Conventional Commits 与 Semantic Versioning 规范：

```text
<type>(<scope>): <动词开头的祈使短语，概括变更意图；严禁描述细节；使用中文>

<可选正文：仅当变更缘由无法从代码差异自明时补充；使用中文>
```

- 破坏性变更统一在作用域后标注感叹号 `<type>(<scope>)!:`。
- 合入 `main` 的 `feat`、`fix` 与破坏性变更会自动触发 `release-please` 创建或更新 Release PR（在 `0.x` 阶段破坏性变更仅升级次版本号，`>= 1.0.0` 后升级主版本号；其余提交类型不触发发版）。

## Pull Request 流程

1. 从 `main` 切出分支。
2. 创建 PR 并开启自动合并与删除分支，确保标题符合 Commit 规范、正文符合 [pull_request_template.md](.github/pull_request_template.md) 规范。
