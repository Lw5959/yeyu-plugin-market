const source = document.querySelector('#source')
const catalog = document.querySelector('#catalog')
const status = document.querySelector('#status')
source.textContent = source.href

function text(tag, value, className) {
  const element = document.createElement(tag)
  element.textContent = value
  if (className) element.className = className
  return element
}

try {
  const response = await fetch(source.href)
  if (!response.ok) throw new Error()
  const { plugins } = await response.json()
  for (const plugin of plugins) {
    const row = document.createElement('article')
    const info = document.createElement('div')
    info.append(
      text('h2', plugin.name),
      text('p', plugin.description),
      text('p', `${plugin.author} · ${plugin.version} · ${plugin.license}`, 'details'),
    )
    const download = text('a', '下载', 'download')
    download.href = new URL(plugin.download, source.href)
    download.download = ''
    download.setAttribute('aria-label', '下载' + plugin.name)
    row.append(info, download)
    catalog.append(row)
  }
  status.hidden = plugins.length > 0
  status.textContent = '市场暂无插件'
} catch {
  status.textContent = '载入失败，请刷新页面。'
}
