# 页屿插件开发指南

页屿插件接收 Markdown 文本并返回修改后的文本。JavaScript 可以直接编写 ES 模块，Rust 使用 WebAssembly。两种语言都可从文件或市场安装。启用后从「编辑 → 运行插件」执行，修改经差异审阅后应用，应用后可撤销。

| 示例 | 语言 | 用法 |
| --- | --- | --- |
| [OpenCode 工作区接入](../examples/plugins/opencode.md) | JavaScript | 生成 OpenCode 1 和 2 的工作区 MCP 配置 |
| [批量格式化](../examples/plugins/batch-format.md) | Rust | 同时整理多份文档的标题间距和表格，逐份预览后保存 |
| [图片整理](../examples/plugins/image-organizer.md) | JavaScript | 预览图片归集，按内容去重并更新本地相对路径 |
| [表格排版](../examples/plugins/table-layout.md) | Rust | 对齐表格源码并保留单元格、列对齐和原有行尾 |
| [表格转换](../examples/plugins/spreadsheet-tools.md) | JavaScript | Excel 或 Numbers 单元格、CSV 转 Markdown 表格，Markdown 表格转 CSV |
| [中文排版](../examples/plugins/chinese-typography.md) | Rust | 给中文和英文、数字补空格，保留代码、链接和原有行尾 |
| [文档目录](../examples/plugins/document-toc.md) | Rust | 生成、更新或移除可点击的 Markdown 目录 |
| [清理行尾空白](../examples/plugins/trim-lines.md) | C/Wasm | 移除各行结尾的空格和 Tab |
| [任务清单](../examples/plugins/task-list.md) | C/Wasm | 将所选行转为任务，批量完成或重置任务 |
| [标题层级](../examples/plugins/heading-levels.md) | C/Wasm | 批量提升或降低 Markdown 标题层级 |

## 清单

从示例复制一份 `.yeyu-plugin.json` 清单，将 `id` 改成自己维护的唯一标识，命令 `id` 与模块的导出同名。发布更新时沿用插件 ID，并递增 SemVer 版本。使用 `scope` 指定菜单运行时的处理范围，`selection` 处理选区，`document` 处理全文。省略时使用选区，也可在插件管理中临时调整处理范围。

```json
{
  "apiVersion": 2,
  "id": "com.example.normalize",
  "name": "示例转换",
  "version": "1.0.0",
  "commands": [
    { "id": "normalize", "label": "整理所选文字", "scope": "selection" }
  ]
}
```

## JavaScript

将上面的清单保存为 `normalize.yeyu-plugin.json`，将下方源码保存为 `normalize.mjs`。每个命令导出一个同步函数，输入和输出都是字符串。原样返回输入表示没有改动。需要提示用户时，可抛出带简洁说明的 `Error`。

```js
export function normalize(text) {
  return text.replace(/[ \t]+(?=\r|\n|$)/g, '')
}
```

```sh
pnpm plugin:pack normalize.yeyu-plugin.json normalize.mjs
```

打包器检查 `.js` 或 `.mjs` 的语法，将 UTF-8 源码写到 `javascript` 字段。需要依赖时先用自己的构建工具产出一个独立的 ES 模块。安装时会加载模块并核对命令导出。页屿 0.2.2 起支持此接口。每次安装校验或运行有 2 秒执行预算，JavaScript 堆最多 32 MiB。运行时提供标准语言内建对象，模块加载、DOM、Node.js、文件和网络接口均未开放。[rquickjs 源码](https://github.com/DelSkayn/rquickjs)

## Rust

将清单的 `apiVersion` 设为 `1`。从 [Rust 示例工程](../examples/plugins/workflows/) 的 `src/bin/` 复制一个示例，保留 `src/lib.rs` 中的 `export_commands!` 宏和输入输出缓冲。每个命令实现 `fn(&str) -> Option<String>`，`None` 表示原文保持不变。下方命令可重建示例插件。

```sh
rustup target add wasm32-unknown-unknown
cargo test --locked --manifest-path examples/plugins/workflows/Cargo.toml
cargo build --locked --release --target wasm32-unknown-unknown --manifest-path examples/plugins/workflows/Cargo.toml
pnpm plugin:pack examples/plugins/document-toc.yeyu-plugin.json examples/plugins/workflows/target/wasm32-unknown-unknown/release/document-toc.wasm
```

打包器将 `.wasm` 编码为 Base64，写到 `wasm` 字段。采用其他语言的 WebAssembly 编译器时，模块按下表导出函数。页屿用 Wasmi 加载无导入的模块，线性内存最多 32 MiB，并限制指令数。[Wasmi 源码](https://github.com/wasmi-labs/wasmi)

| Wasm 导出 | 用法 |
| --- | --- |
| `memory` | 保存 UTF-8 输入和输出的线性内存 |
| `alloc(length: i32) -> i32` | 返回可写入 `length` 字节的地址 |
| `<命令 ID>(ptr: i32, length: i32) -> i64` | 返回输出地址和长度，高 32 位为地址，低 32 位为 UTF-8 字节数 |

地址和长度按无符号 32 位数打包。命令可以在原缓冲区修改，也可以返回另一个缓冲区。页屿只传入此次命令的文本。卸载、停用或更新后，仍在运行的旧结果会丢弃。单包最多 2 MiB，输入和输出各最多 4 MiB。

## 多文档和图片命令

页屿 0.2.3 提供接口 3。清单仍选择一个 `javascript` 或 `wasm` 模块，导出函数沿用上面的接口。使用 `workflow` 声明下列工作流。

| 工作流 | 输入和返回 | 页屿负责的操作 |
| --- | --- | --- |
| `files` | 每次传入一篇文档的 Markdown。返回完整的替换文本 | 选择本地文件、逐篇显示差异、勾选并保存。保存核对预览版本，沿用原 BOM |
| `images` | 输入 JSON 字符串 `{"images":[{"digest":"…","extension":"png"}]}`。返回同顺序的文件名数组，编码为 JSON 字符串 | 识别本文的本地行内图片，计算 BLAKE3，按摘要与扩展名聚合。确认后复制图片并更新链接 |

图片名称用小写字母、数字、下划线和连字符，名称主体最多 100 个字符，文件扩展名与输入一致。同一数组内各文件名唯一。可使用摘要生成稳定的名字。文件路径和文件字节由应用读取，模块接收的图片信息是摘要与扩展名。二次运行时可复用同名、同内容的副本。包清单和用法见[图片整理](../examples/plugins/image-organizer.md)与[批量格式化](../examples/plugins/batch-format.md)。

桌面版 0.2.21 提供接口 4。`mcp` 工作流把已授权工作区的 MCP 启动配置作为 JSON 字符串传入命令，形状为 `{"mcpServers":{"yeyu":{"command":"…","args":["…"]}}}`。命令返回客户端配置文本，页屿提供复制操作。实际的文档访问和修改建议仍由 MCP 服务检查工作区授权。 [OpenCode 示例](../examples/plugins/opencode.md)

## 试用和发布

1. 在「编辑 → 插件管理 → 从文件安装」选择生成的包，再点「启用」。运行命令后核对差异、应用并撤销。
2. 用中文、空选区、原文不变、混合换行及代码块做测试。每个示例附有可粘贴到编辑器的样稿。
3. 重装同一 ID 时，替换会即时生效，并沿用启用状态。通过市场更新时递增版本，并填写描述、作者和许可证。[插件市场发布指南](plugin-market.md)

示例源码及插件包采用 MIT 许可证，Rust 插件依赖的许可随包分发。[示例许可证](../examples/plugins/LICENSE) · [第三方许可](../examples/plugins/THIRD-PARTY-NOTICES.txt)
