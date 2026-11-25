# 表格排版

对齐 GFM 表格源码，适合整理从其他文档复制来的表格。在「编辑 → 插件市场」安装并启用后，从「编辑 → 运行插件 → 对齐 Markdown 表格」检查和应用差异。

```markdown
|功能|结果|
|:-|--:|
|图片归集|通过|
|表格|a\|b|
```

表格按 Unicode 字符宽度补齐。已有左、右、居中标记及转义竖线保留在源码中。命令处理列数一致的顶层表格，代码块和缺列的表格维持原样。也可在插件管理中选「当前选区」，只整理选中的整张表。

## 开发

Markdown 解析器先定位表格，排版函数只调整列两侧的空格和分隔行。批量格式化也调用此函数。样例测试覆盖中文、混合换行、转义、对齐标记、列数异常和代码块。[排版源码](workflows/src/table_layout.rs) · [插件开发指南](../../docs/plugins.md)

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked --manifest-path examples/plugins/workflows/Cargo.toml
cargo build --locked --release --target wasm32-unknown-unknown --manifest-path examples/plugins/workflows/Cargo.toml --bin table-layout
pnpm plugin:pack examples/plugins/table-layout.yeyu-plugin.json examples/plugins/workflows/target/wasm32-unknown-unknown/release/table-layout.wasm
```

对齐 Markdown 源码的使用场景参考了 VS Code 的表格格式化命令。[Markdown All in One](https://github.com/yzhang-gh/vscode-markdown#github-flavored-markdown)
