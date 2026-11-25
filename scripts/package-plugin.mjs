import { readFileSync, writeFileSync } from 'node:fs'

const [manifestPath, modulePath] = process.argv.slice(2)
if (!manifestPath?.endsWith('.yeyu-plugin.json') || !modulePath) {
  throw new Error('用法：pnpm plugin:pack <清单.yeyu-plugin.json> <模块.wasm>')
}
const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'))
const module = readFileSync(modulePath)
if (!WebAssembly.validate(module)) throw new Error('无效的 WebAssembly 模块')
const compiled = new WebAssembly.Module(module)
const exports = new Map(WebAssembly.Module.exports(compiled).map(({ name, kind }) => [name, kind]))
if (WebAssembly.Module.imports(compiled).length || exports.get('memory') !== 'memory'
  || exports.get('alloc') !== 'function' || !manifest.commands?.length
  || manifest.commands.some(command => exports.get(command.id) !== 'function')) {
  throw new Error('模块须无导入，并导出内存、分配函数和清单中的命令')
}
const output = JSON.stringify({ ...manifest, wasm: module.toString('base64') }, null, 2) + '\n'
if (Buffer.byteLength(output) > 2 * 1024 * 1024) throw new Error('插件包超过 2 MiB')
writeFileSync(manifestPath, output)
console.log(manifestPath)
