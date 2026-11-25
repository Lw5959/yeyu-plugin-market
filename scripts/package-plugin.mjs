import { spawnSync } from 'node:child_process'
import { readFileSync, writeFileSync } from 'node:fs'

const [manifestPath, modulePath] = process.argv.slice(2)
if (!manifestPath?.endsWith('.yeyu-plugin.json') || !modulePath) {
  throw new Error('用法：pnpm plugin:pack <清单.yeyu-plugin.json> <模块.wasm、.js 或 .mjs>')
}
const { wasm, javascript, ...manifest } = JSON.parse(readFileSync(manifestPath, 'utf8'))
const module = readFileSync(modulePath)
const script = /\.(?:mjs|js)$/i.test(modulePath)
let payload
if (script) {
  if (![2, 3].includes(manifest.apiVersion)) throw new Error('JavaScript 清单须使用 apiVersion 2 或 3')
  const source = new TextDecoder('utf-8', { fatal: true }).decode(module)
  if (!source.trim()) throw new Error('JavaScript 模块为空')
  // 打包只检查语法，命令导出由页屿在隔离运行时校验。
  const check = spawnSync(process.execPath, ['--input-type=module', '--check'], { input: source, encoding: 'utf8' })
  if (check.error) throw check.error
  if (check.status !== 0) throw new Error('JavaScript 模块语法错误\n' + check.stderr.trim())
  payload = { javascript: source }
} else {
  if (![1, 3].includes(manifest.apiVersion)) throw new Error('WebAssembly 清单须使用 apiVersion 1 或 3')
  if (!WebAssembly.validate(module)) throw new Error('无效的 WebAssembly 模块')
  const compiled = new WebAssembly.Module(module)
  const exports = new Map(WebAssembly.Module.exports(compiled).map(({ name, kind }) => [name, kind]))
  if (WebAssembly.Module.imports(compiled).length || exports.get('memory') !== 'memory'
    || exports.get('alloc') !== 'function' || !manifest.commands?.length
    || manifest.commands.some(command => exports.get(command.id) !== 'function')) {
    throw new Error('模块须无导入，并导出内存、分配函数和清单中的命令')
  }
  payload = { wasm: module.toString('base64') }
}
const output = JSON.stringify({ ...manifest, ...payload }, null, 2) + '\n'
if (Buffer.byteLength(output) > 2 * 1024 * 1024) throw new Error('插件包超过 2 MiB')
writeFileSync(manifestPath, output)
console.log(manifestPath)
