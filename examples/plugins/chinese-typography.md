# 中文排版

为正文中相邻的中文与英文、数字补空格。适合整理中英混排的项目文档、报告和笔记。

## 试用

在「编辑 → 插件市场」安装并启用「中文排版」。打开 Markdown 文档，从「编辑 → 运行插件」选择「整理中英文与数字间距」，确认差异后应用。

```markdown
整理API文档，一共50页。见[接口V2](api-v2.md)，变量是 `读数A`。
```

```markdown
整理 API 文档，一共 50 页。见[接口V2](api-v2.md)，变量是 `读数A`。
```

默认读取全文来识别 Markdown 结构。代码行、代码块、链接和图片、美元符号公式、YAML、常见的标签和地址保留原样。只想调整一段时，可在「插件管理」选择「当前选区」，选中完整段落后再运行。

## 开发

[Rust 源码](workflows/src/bin/chinese-typography.rs) 用 Pulldown Cmark 识别 Markdown 节点及其源码位置，只在匹配的文本字符之间插入空格。测试覆盖混合行尾、中文、YAML、链接、图片、代码、公式及重复运行。以下命令从仓库根目录执行。[页屿插件开发指南](../../docs/plugins.md) · [Pulldown Cmark 源码](https://github.com/pulldown-cmark/pulldown-cmark)

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked --manifest-path examples/plugins/workflows/Cargo.toml --bin chinese-typography
cargo build --locked --release --target wasm32-unknown-unknown --manifest-path examples/plugins/workflows/Cargo.toml --bin chinese-typography
pnpm plugin:pack examples/plugins/chinese-typography.yeyu-plugin.json examples/plugins/workflows/target/wasm32-unknown-unknown/release/chinese-typography.wasm
```

正文间距规则参考了开源 Markdown 格式检查工具的实践。[Obsidian Linter 的排版规则](https://github.com/platers/obsidian-linter/blob/master/docs/rules.md) · [pangu.js 的 Markdown 使用说明](https://github.com/vinta/pangu.js)
