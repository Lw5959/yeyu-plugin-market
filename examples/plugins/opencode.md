# OpenCode 工作区接入

适用于页屿桌面版 0.2.21 及以上。可从在线市场或「内置插件」安装。插件生成配置的过程可以离线完成。

1. 在页屿打开工作区文件夹，从「视图 → 协作侧栏 → MCP 工具与连接」授权此工作区。
2. 安装并启用「OpenCode 工作区接入」。从「编辑 → 运行插件」选择「复制 OpenCode 2 配置」或「复制 OpenCode 1 配置」。
3. 把复制出的条目合并到 OpenCode 项目的 `opencode.json` 或 `opencode.jsonc`。OpenCode 2 的条目位于 `mcp.servers.yeyu`，OpenCode 1 位于 `mcp.yeyu`。在该项目目录运行 `opencode mcp list` 确认连接。 [OpenCode MCP 配置](https://opencode.ai/docs/mcp-servers/)
4. 在 OpenCode 中让页屿 MCP 读取当前选区或工作区文档。修改需要调用 `suggest_edit` 提交建议。在页屿 MCP 侧栏打开待审建议、核对差异并应用。 [页屿 MCP 工具](https://github.com/Lw5959/yeyu/blob/main/docs/mcp.md)

例如，在页屿选中一段 Markdown 后，对 OpenCode 说「用页屿 MCP 读取当前选区，精简这段文字，再提交一份修改建议」。

OpenCode 2.4.2 的运行时接口也可以注册插件生成的配置。项目文件已保存为纯 JSON、服务已启动、`mcp list` 仍为空时，可在 macOS 或 Linux 的项目目录使用下列命令。运行时注册的条目在服务重启后需重新注册。

```sh
opencode api experimental.mcp.add \
  --param server=yeyu --param "directory=$PWD" \
  --data "$(jq -c '{config:.mcp.servers.yeyu}' opencode.json)"
opencode mcp list
```

重建插件包可运行下方命令。源模块接收页屿生成的启动配置，并将其转换为对应 OpenCode 版本的 JSON。导出函数与清单中的命令 ID 对应。

```sh
pnpm plugin:pack examples/plugins/opencode.yeyu-plugin.json examples/plugins/opencode.mjs
```
