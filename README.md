# 页屿插件市场

页屿是一款桌面 Markdown 编辑器。这个仓库提供应用使用的公开插件目录、可安装的 Wasm 插件、源码和开发文档。

- [浏览市场](https://lw5959.github.io/yeyu-plugin-market/)
- [应用目录](https://lw5959.github.io/yeyu-plugin-market/index.json)
- [插件开发指南](docs/plugins.md)

在页屿中打开「编辑 → 插件市场」。也可以复制应用目录地址，在「市场源」中连接。安装后在「已安装」中启用插件，选中文本并运行命令，审阅差异后应用。

## 插件

| 插件 | 开发说明 |
|---|---|
| 清理行尾空白 | [输入输出与重建](examples/plugins/trim-lines.md) |
| 任务清单 | [输入输出与重建](examples/plugins/task-list.md) |
| 标题层级 | [输入输出与重建](examples/plugins/heading-levels.md) |

## 提交与发布

通过 Pull Request 提交插件包、源码、许可证、重建步骤和至少一个可复测样例。更新已有插件时沿用 ID，递增 SemVer。用「编辑 → 插件管理 → 从文件安装」核对运行、差异审阅和撤销。

在仓库根目录运行下列命令，更新 `docs/index.json` 和对应的插件包。保留现有按摘要命名的包，让仍在使用前一份目录的客户端可以完成下载。合并到 `main` 后，由 GitHub Pages 发布 `docs/`。

```sh
pnpm plugin:market examples/plugins docs
```

市场工具与随附插件采用 [MIT 许可证](LICENSE)。
