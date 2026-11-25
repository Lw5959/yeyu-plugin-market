import { createHash, randomUUID } from 'node:crypto'
import { copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, renameSync, writeFileSync } from 'node:fs'
import { join, resolve } from 'node:path'

const [source = 'examples/plugins', destination = 'release/plugin-market'] = process.argv.slice(2)
const packages = readdirSync(source).filter(name => name.endsWith('.yeyu-plugin.json')).sort()
if (!packages.length) throw new Error('目录中没有 .yeyu-plugin.json 插件包')
const ids = new Set()
const validText = (value, length = 120) => typeof value === 'string' && value.trim()
  && [...value].length <= length && !/\p{Cc}/u.test(value)
const versionPattern = /^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?(?:\+[0-9A-Za-z-]+(?:\.[0-9A-Za-z-]+)*)?$/
const plugins = packages.map(file => {
  const bytes = readFileSync(join(source, file))
  const { apiVersion, id, name, version, market, wasm, javascript } = JSON.parse(bytes.toString('utf8'))
  const validProgram = (apiVersion === 1 && typeof wasm === 'string' && wasm && javascript === undefined)
    || (apiVersion === 2 && typeof javascript === 'string' && javascript.trim() && wasm === undefined)
  if (!Number.isInteger(apiVersion) || apiVersion < 1 || apiVersion > 0xFFFFFFFF
    || typeof id !== 'string' || !/^[a-zA-Z0-9_.-]{1,100}$/.test(id)
    || !validText(name) || !validText(version) || !versionPattern.test(version)
    || !validText(market?.description, 600) || !validText(market?.author) || !validText(market?.license)
    || !validProgram || ids.has(id) || bytes.length > 2 * 1024 * 1024) {
    throw new Error(`插件包或市场资料无效 ${file}`)
  }
  ids.add(id)
  const sha256 = createHash('sha256').update(bytes).digest('hex')
  const download = `packages/${id}-${sha256}.yeyu-plugin.json`
  const { description, author, license } = market
  return { entry: { apiVersion, id, name, version, description, author, license, sha256, download }, bytes,
    featured: market.featured === true }
}).sort((left, right) => Number(right.featured) - Number(left.featured))

mkdirSync(join(destination, 'packages'), { recursive: true })
for (const { entry, bytes } of plugins) writeFileSync(join(destination, entry.download), bytes)
for (const notice of ['LICENSE', 'THIRD-PARTY-NOTICES.txt']) {
  if (existsSync(join(source, notice))) copyFileSync(join(source, notice), join(destination, notice))
}
// 先就绪所有包，再切换目录，使用旧目录的客户端仍可下载旧版本。
const temporary = join(destination, `.index-${randomUUID()}.json`)
const index = { schemaVersion: 1, name: '页屿插件市场', plugins: plugins.map(({ entry }) => entry) }
const json = JSON.stringify(index, null, 2) + '\n'
if (Buffer.byteLength(json) > 1024 * 1024) throw new Error('市场目录超过 1 MiB')
writeFileSync(temporary, json)
renameSync(temporary, join(destination, 'index.json'))
console.log(`已生成 ${plugins.length} 个插件的市场目录 ${resolve(destination, 'index.json')}`)
