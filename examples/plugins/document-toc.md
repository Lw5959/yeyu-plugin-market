# 文档目录

把当前大纲写进 Markdown 文本，适合随文件分享的使用手册、项目文档和长篇笔记。增删标题后可以更新目录。

## 试用

在「编辑 → 插件市场」安装并启用「文档目录」。打开下方示例，从「编辑 → 运行插件」选择「生成或更新目录」。

```markdown
# 使用手册

[TOC]

## 安装

### 验证 API

## 排查问题
```

插件会把单独一段的 `[TOC]` 替换为带链接的分层列表。如果没有占位符，就把目录放在文档标题下方。修改标题后再次运行即可更新。定位旧目录用的两行引用定义可以在源码模式查看。移除时选择「移除生成的目录」，正文保持原样。

实时模式按住 Command（macOS）或 Ctrl（Windows、Linux）点击目录链接，可跳转到标题。锚点沿用页屿的标题规则，在页屿及其 HTML 导出中使用。其他 Markdown 阅读器可能对重复标题使用不同的编号。

## 开发

[Rust 源码](workflows/src/bin/document-toc.rs) 从 Markdown 解析结果读取章节，按标题深度生成列表。用完整的引用定义定位可更新区块，兼容混合行尾。测试包含重复标题、Setext 标题、代码块、重复运行和保留手写正文。下面的命令从仓库根目录执行。[页屿插件开发指南](../../docs/plugins.md)

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked --manifest-path examples/plugins/workflows/Cargo.toml --bin document-toc
cargo build --locked --release --target wasm32-unknown-unknown --manifest-path examples/plugins/workflows/Cargo.toml --bin document-toc
pnpm plugin:pack examples/plugins/document-toc.yeyu-plugin.json examples/plugins/workflows/target/wasm32-unknown-unknown/release/document-toc.wasm
```

将可更新的目录保存在 Markdown 源码中的工作流参考了 Markdown All in One。[上游目录功能](https://github.com/yzhang-gh/vscode-markdown#table-of-contents)
