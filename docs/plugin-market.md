# 插件市场目录与发布

「编辑 → 插件市场」默认连接官方目录。表格转换、中文排版、文档目录及此前的三个插件随应用发布，可在「内置插件」中离线搜索和安装。在线目录连接失败时会切到内置插件，并保留原市场源。新安装的插件默认停用，点击卡片上的「已安装」进入管理页启用，再从「编辑 → 运行插件」执行命令。[官方市场页面](https://lw5959.github.io/yeyu-plugin-market/) · [应用目录](https://lw5959.github.io/yeyu-plugin-market/index.json)

## 向官方市场提交

官方目录通过 GitHub Pages 发布，开源仓库的 `main` 分支下 `docs/` 是发布目录。通过 Pull Request 提交插件包、源码、许可证、重建步骤和可复测样例。更新包沿用原 ID，并递增 SemVer。[开源市场仓库](https://github.com/Lw5959/yeyu-plugin-market) · [GitHub Pages 发布源说明](https://docs.github.com/en/pages/getting-started-with-github-pages/configuring-a-publishing-source-for-your-github-pages-site)

在市场仓库根目录运行以下命令，生成新的目录和按摘要命名的插件包。保留旧包，已经打开前一份目录的用户可以继续下载。合并到 `main` 后，检查 Pages 发布状态，并从公网下载核对 `index.json` 与各包的 SHA-256。

```sh
pnpm plugin:market examples/plugins docs
```

## 自建目录

在仓库根目录生成一份可部署的静态目录。

```sh
pnpm plugin:market
```

输出在 `release/plugin-market/`。其中 `index.json` 是市场目录，`packages/` 包含插件包。目录按包内容生成 SHA-256，文件名也含摘要。自己的插件放在一个目录中，可以指定输入和输出路径。

```sh
pnpm plugin:market ./my-plugins ./release/community-market
```

本机试用时用下面的命令启动预览服务。在市场右上角「市场源」填写 `http://127.0.0.1:8750/index.json`，点击「连接」。「内置插件」可随时切换，「恢复官方市场」可清除自定义地址。

```sh
python3 -m http.server 8750 --bind 127.0.0.1 --directory release/plugin-market
```

向其他用户提供目录时，将输出文件部署到同一个静态 HTTPS 站点，并公布 `index.json` 的直达地址。先上传新包，再发布新的目录。旧版目录引用的插件包可以保留，便于已经打开市场的用户完成安装。应用完成连接后保存市场源地址，点击「刷新市场」可检查新版本。

## 准备插件包

1. 按[插件开发指南](plugins.md)制作并在页屿试用插件包。
2. 在清单中增加 `market`，填写用途、作者和许可证。和插件包一起提供对应源码、构建步骤和许可证文本。
3. 把包加入目录的输入文件夹。同一个目录为每个 ID 发布一个版本。替换已有 ID 时递增 SemVer，再生成并发布目录。
4. 在页屿刷新市场。核对「安装」或「更新」按钮、版本及运行结果。更新沿用插件的启用状态。

```json
{
  "market": {
    "description": "整理选区内的 Markdown 标题。",
    "author": "开发者名称",
    "license": "MIT"
  }
}
```

## 目录格式

目录包含 `schemaVersion: 1`、用于界面显示的 `name` 和 `plugins` 数组。作用与限制如下。

| 插件字段 | 用途 |
| --- | --- |
| `id`、`name`、`version`、`apiVersion` | 与包内清单一致。版本遵循 SemVer。接口 1 为 WebAssembly，接口 2 为 JavaScript（页屿 0.2.2 起） |
| `description`、`author`、`license` | 显示用途、作者和许可证。描述最多 600 字符 |
| `download` | 插件包的直达 URL，可相对于 `index.json`。公网用 HTTPS，本机 HTTP 要与目录同源 |
| `sha256` | 对下载的完整文件字节计算的 SHA-256，写成 64 个十六进制字符 |

目录最多 1 MiB，单包最多 2 MiB。下载完成后核对摘要、包清单和命令导出，再写入安装记录。安装错误会在市场中显示，已安装版本可以继续使用。旧式本地包允许自定义版本文字，市场可以比较符合 SemVer 的已安装版本。
