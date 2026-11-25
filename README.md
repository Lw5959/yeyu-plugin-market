# 页屿插件市场

页屿是一款桌面 Markdown 编辑器。这个仓库提供公开插件目录、可安装的 JavaScript 和 WebAssembly 插件、源码和开发文档。

- [浏览市场](https://lw5959.github.io/yeyu-plugin-market/)
- [应用目录](https://lw5959.github.io/yeyu-plugin-market/index.json)
- [插件开发指南](docs/plugins.md)

页屿 0.2.1 起，在「编辑 → 插件市场」默认连接本目录。「内置插件」可以离线安装。页屿 0.2.2 支持 JavaScript，并随应用提供下列六款插件。安装后在「已安装」中启用，再从「编辑 → 运行插件」执行命令。审阅差异后可以应用并撤销。

## 插件

| 插件 | 语言 | 用法与开发说明 |
|---|---|---|
| 表格转换 | JavaScript | [电子表格、CSV 与 Markdown 表格互转](examples/plugins/spreadsheet-tools.md) |
| 中文排版 | Rust | [整理中英文和数字间距](examples/plugins/chinese-typography.md) |
| 文档目录 | Rust | [生成、更新和移除 Markdown 目录](examples/plugins/document-toc.md) |
| 清理行尾空白 | C/Wasm | [输入输出与重建](examples/plugins/trim-lines.md) |
| 任务清单 | C/Wasm | [输入输出与重建](examples/plugins/task-list.md) |
| 标题层级 | C/Wasm | [输入输出与重建](examples/plugins/heading-levels.md) |

## 提交与发布

通过 Pull Request 提交插件包、源码、许可证、重建步骤和至少一个可复测样例。更新已有插件时沿用 ID，递增 SemVer。用「编辑 → 插件管理 → 从文件安装」核对运行、差异审阅和撤销。

在仓库根目录运行下列命令，更新 `docs/index.json` 和对应的插件包。保留现有按摘要命名的包，让仍在使用前一份目录的客户端可以完成下载。合并到 `main` 后，由 GitHub Pages 发布 `docs/`。

```sh
pnpm plugin:market examples/plugins docs
```

市场工具与随附插件采用 MIT 许可证。[许可证](LICENSE) · [第三方许可](examples/plugins/THIRD-PARTY-NOTICES.txt)
