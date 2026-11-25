# 批量格式化

把多篇 Markdown 的顶层标题间距和表格源码整理成一致的样式。安装并启用「批量格式化」后，从「编辑 → 运行插件 → 批量格式化文件…」多选 Markdown 文件。点击左侧文件名看该文件的差异，只勾选要保存的文件。

「格式化当前文档」适合先在正在编辑的文档中试用。可以逐段应用、撤销。批量保存走文件版本检查，预览之后有外部改动的文件会跳过并报告。当前文档有未保存编辑时，可先保存或用「格式化当前文档」。

```markdown
# 收集资料
这是第一段


## 确认来源
| 名称|状态 |
|:-|--:|
|项目文档|已读|
```

运行后标题与相邻顶层块之间留一空行，表格按列补齐。YAML、代码内容、段落硬换行和已有行尾随文件保留。已打开的干净文档会同步显示保存结果，可撤销其编辑区修改。其他文件由批量确认直接保存到磁盘。

## 开发

插件的两个入口都执行同一个 Rust 函数，结合语法节点位置调整顶层块之间的空行，并复用表格排版算法。清单里的 `workflow: "files"` 让宿主把用户选择的文件逐个送入命令。测试覆盖混合换行、代码以及重复运行。[Rust 源码](workflows/src/bin/batch-format.rs) · [插件开发指南](../../docs/plugins.md)

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked --manifest-path examples/plugins/workflows/Cargo.toml
cargo build --locked --release --target wasm32-unknown-unknown --manifest-path examples/plugins/workflows/Cargo.toml --bin batch-format
pnpm plugin:pack examples/plugins/batch-format.yeyu-plugin.json examples/plugins/workflows/target/wasm32-unknown-unknown/release/batch-format.wasm
```

批量处理和标题空行的工作流参考了 Obsidian Linter，表格排版参考了 VS Code 的 Markdown All in One。[Obsidian Linter 规则](https://github.com/platers/obsidian-linter/blob/master/docs/rules.md) · [Markdown All in One](https://github.com/yzhang-gh/vscode-markdown#github-flavored-markdown)
