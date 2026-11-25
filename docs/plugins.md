# 页屿插件开发指南

页屿插件 v1 把一段 Markdown 交给 WebAssembly 函数处理。用户在「编辑 → 插件管理」选择当前选区或全文，运行命令并审阅差异。应用后的修改可以撤销。

| 示例 | 用法 |
| --- | --- |
| [清理行尾空白](../examples/plugins/trim-lines.md) | 移除各行结尾的空格和 Tab |
| [任务清单](../examples/plugins/task-list.md) | 将所选行转为任务，批量完成或重置任务 |
| [标题层级](../examples/plugins/heading-levels.md) | 批量提升或降低 Markdown 标题层级 |

## 编写模块和清单

1. 从任一示例复制源码和 `.yeyu-plugin.json` 清单。将 `id` 改成自己维护的唯一标识，例如 `com.example.normalize`。发布更新时沿用 ID，递增 `version`。
2. 实现 `alloc` 及命令函数。清单中的命令 `id` 要与对应的 WebAssembly 导出同名，`label` 是用户看到的命令名称。
3. 用带 `wasm32` 后端的 Clang 和 `wasm-ld` 编译。也可以使用能输出相同 ABI 的其他编译器。保留模块所需的导出。
4. 运行 `pnpm plugin:pack 路径/插件.yeyu-plugin.json 路径/模块.wasm`。脚本检查 Wasm 和导出名称，将标准 Base64 写入清单的 `wasm` 字段。页屿安装时继续校验函数签名。

清单是 UTF-8 JSON。开发新插件时可以先留空 `wasm`，打包后再安装。

```json
{
  "apiVersion": 1,
  "id": "com.example.normalize",
  "name": "示例转换",
  "version": "1.0.0",
  "commands": [
    { "id": "normalize", "label": "整理所选文字" }
  ],
  "wasm": ""
}
```

| Wasm 导出 | 用法 |
| --- | --- |
| `memory` | 保存 UTF-8 输入和输出的线性内存 |
| `alloc(length: i32) -> i32` | 返回可写入 `length` 字节的地址 |
| `<命令 ID>(ptr: i32, length: i32) -> i64` | 返回输出地址和长度，高 32 位为地址，低 32 位为 UTF-8 字节数 |

地址和长度按无符号 32 位数打包。命令可以在原缓冲区修改，也可以返回另一个缓冲区。原样返回输入表示没有改动。示例以字节处理 ASCII Markdown 标记，保留其他 UTF-8 字节和 LF、CRLF、CR。

页屿为每次运行创建独立的 Wasmi 实例，输入来自用户选定的文字。模块须无导入。输入和输出各最多 4 MiB，线性内存最多 32 MiB，插件包最多 2 MiB。插件运行有指令预算，停用、卸载或替换后会丢弃该次运行的旧结果。

## 本地试用和发布

1. 在「编辑 → 插件管理 → 从文件安装」选择生成的包，再点「启用」。运行命令后核对差异、应用并撤销。
2. 用中文、空选区、原文不变、混合换行以及代码块做测试。试用时可从任务清单和标题层级的示例复制输入。
3. 重装同一 ID 可在当前窗口替换插件，原启用状态会保留。通过市场发布时使用递增的 SemVer，并填写描述、作者和许可证。目录制作和提交步骤见[插件市场发布指南](plugin-market.md)。

首批示例源码及插件包采用 [MIT 许可证](../examples/plugins/LICENSE)。宿主使用的 Wasm 运行时见 [Wasmi 源码与许可证](https://github.com/wasmi-labs/wasmi)。
