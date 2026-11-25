// SPDX-License-Identifier: MIT

const eol = /\r\n|\r|\n/

function bounds(source) {
  const prefix = source.match(/^[\r\n]*/)[0]
  const suffix = source.slice(prefix.length).match(/[\r\n]*$/)[0]
  const body = source.slice(prefix.length, source.length - suffix.length)
  return { prefix, body, suffix, newline: source.match(eol)?.[0] ?? '\n' }
}

function readRows(source, separator) {
  const rows = []
  let at = 0
  while (at < source.length) {
    const row = []
    while (true) {
      let value = ''
      if (source[at] === '"') {
        at++
        while (true) {
          const quote = source.indexOf('"', at)
          if (quote < 0) return null
          value += source.slice(at, quote)
          at = quote + 1
          if (source[at] !== '"') break
          value += '"'
          at++
        }
      } else {
        const start = at
        while (at < source.length && source[at] !== separator && source[at] !== '\r' && source[at] !== '\n') {
          if (source[at] === '"') return null
          at++
        }
        value = source.slice(start, at)
      }
      row.push(value)
      if (source[at] !== separator) break
      at++
    }
    rows.push(row)
    // 分隔符可以出现在引号字段中。只有已结束的记录才消耗行尾。
    if (source[at] === '\r') at += source[at + 1] === '\n' ? 2 : 1
    else if (source[at] === '\n') at++
    else if (at !== source.length) return null
  }
  return rows
}

const entities = { '&': '&amp;', '<': '&lt;', '>': '&gt;', '\n': '&#10;', '\r': '&#13;', '\t': '&#9;' }
const decodedEntities = { '&amp;': '&', '&lt;': '<', '&gt;': '>', '&#10;': '\n', '&#13;': '\r', '&#9;': '\t', '&#32;': ' ' }

function markdownCell(value) {
  return value
    .replace(/[&<>\r\n\t]/g, match => entities[match])
    .replace(/[\\|\[\]*_`~]/g, '\\$&')
    .replace(/^ +| +$/g, margin => '&#32;'.repeat(margin.length))
}

function toMarkdown(source, separator) {
  const { prefix, body, suffix, newline } = bounds(source)
  const records = readRows(body.replace(/^\uFEFF/, ''), separator)
  const columns = records?.[0]?.length ?? 0
  if (columns < 2 || records.some(record => record.length !== columns)) return source
  const rows = records.map(record => '| ' + record.map(markdownCell).join(' | ') + ' |')
  rows.splice(1, 0, '|' + ' --- |'.repeat(columns))
  return prefix + rows.join(newline) + suffix
}

function splitMarkdownRow(line) {
  const cells = []
  const raw = line.trim()
  let start = raw.startsWith('|') ? 1 : 0
  for (let at = start; at < raw.length; at++) {
    if (raw[at] === '\\') {
      at++
    } else if (raw[at] === '|') {
      cells.push(raw.slice(start, at).trim())
      start = at + 1
    }
  }
  if (start < raw.length || cells.length === 0) cells.push(raw.slice(start).trim())
  return cells
}

function csvCell(value) {
  const decoded = value
    .replace(/\\([\\|\[\]*_`~])/g, '$1')
    .replace(/&(amp|lt|gt|#10|#13|#9|#32);/g, match => decodedEntities[match])
  return /[",\r\n]|^\s|\s$/.test(decoded) ? '"' + decoded.replaceAll('"', '""') + '"' : decoded
}

export function tsv_to_markdown(text) {
  return toMarkdown(text, '\t')
}

export function csv_to_markdown(text) {
  return toMarkdown(text, ',')
}

export function markdown_to_csv(text) {
  const { prefix, body, suffix, newline } = bounds(text)
  const rows = body.split(eol).map(splitMarkdownRow)
  const columns = rows[0]?.length ?? 0
  if (columns < 2 || rows.length < 2 || rows.some(row => row.length !== columns) ||
      !rows[1].every(cell => /^:?-+:?$/.test(cell))) return text
  rows.splice(1, 1)
  return prefix + rows.map(row => row.map(csvCell).join(',')).join(newline) + suffix
}
