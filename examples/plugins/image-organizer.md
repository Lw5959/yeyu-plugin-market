# 图片整理

把当前 Markdown 引用的本地行内图片归集到同文档旁的 `文档名.assets` 文件夹。文件名由图片内容决定，同格式的相同内容共用一份文件。原图可继续供其他文档引用。

从「编辑 → 插件市场」安装并启用「图片整理」。保存文档后，从「编辑 → 运行插件 → 收拢本文本地图片…」查看路径差异和无法读取的图片，再点「复制并更新链接」。相对路径、含空格的路径及本机绝对路径均可归集。远程图、嵌入式数据、引用式图片和 HTML 图片留在原有使用方式中。

```markdown
![初稿](<素材/截图 一.png> "来自评审")
![同一份截图](备用图.png)
```

若两张 PNG 的内容相同，两条引用会使用同一个 `文档名.assets/image-….png`。应用后可以撤销链接修改，已经复制的资源仍可在下次归集时复用。再次运行会检查未整理的图片，已归集的链接保持稳定。

## 开发

`workflow: "images"` 由宿主识别路径、检查文件和写入资源目录。JavaScript 使用宿主给出的内容摘要和扩展名生成稳定的文件名。源码与清单可以单独阅读，打包后用含重复、缺失和外部链接的文档复测预览、取消、应用和再次运行。[JavaScript 源码](image-organizer.mjs) · [插件开发指南](../../docs/plugins.md)

```sh
pnpm plugin:pack examples/plugins/image-organizer.yeyu-plugin.json examples/plugins/image-organizer.mjs
```

文件归集和链接同步参考了 Obsidian 的 AttachFlow 与 VS Code 的 Markdown Paste。[AttachFlow](https://github.com/Yaozhuwa/AttachFlow) · [Markdown Paste 的图片路径选项](https://github.com/telesoho/vscode-markdown-paste-image/blob/master/README.md)
