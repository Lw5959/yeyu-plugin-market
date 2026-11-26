function server(input) {
  const launcher = JSON.parse(input)?.mcpServers?.yeyu
  if (typeof launcher?.command !== 'string' || !launcher.command || !Array.isArray(launcher.args) || !launcher.args.every(arg => typeof arg === 'string')) {
    throw new Error('请先在页屿授权当前工作区并生成 MCP 启动配置')
  }
  // OpenCode 的本地服务把可执行文件和参数放在同一个数组中，保留路径中的空格。
  return { type: 'local', command: [launcher.command, ...launcher.args] }
}

export function copy_v2(input) {
  return JSON.stringify({ mcp: { servers: { yeyu: server(input) } } }, null, 2)
}

export function copy_v1(input) {
  return JSON.stringify({ mcp: { yeyu: server(input) } }, null, 2)
}
