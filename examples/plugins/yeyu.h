/* SPDX-License-Identifier: MIT */
#ifndef YEYU_PLUGIN_H
#define YEYU_PLUGIN_H

typedef __UINT32_TYPE__ u32;
typedef __UINT64_TYPE__ u64;
typedef __UINTPTR_TYPE__ uptr;
typedef unsigned char Byte;

#define MAX_TEXT (4u * 1024 * 1024)

typedef struct { u32 start, end, next; } Line;
typedef struct { Byte marker; u32 count; } Fence;
typedef struct { Byte *bytes; u32 length; } Output;

static inline int space(Byte c) {
  return c == ' ' || c == '\t';
}

static inline Line line_at(const Byte *text, u32 start, u32 length) {
  u32 end = start;
  while (end < length && text[end] != '\r' && text[end] != '\n') end++;
  u32 next = end;
  if (next < length) {
    next++;
    if (text[end] == '\r' && next < length && text[next] == '\n') next++;
  }
  return (Line){start, end, next};
}

static inline u32 content_at(const Byte *text, Line line) {
  u32 pos = line.start;
  while (pos < line.end && text[pos] == ' ' && pos - line.start < 4) pos++;
  return pos;
}

static inline int indented(const Byte *text, Line line, u32 pos) {
  return pos - line.start >= 4 || (pos < line.end && text[pos] == '\t');
}

static inline int three_of(const Byte *text, Line line, Byte marker) {
  return line.end - line.start == 3 && text[line.start] == marker &&
    text[line.start + 1] == marker && text[line.start + 2] == marker;
}

/* 只有文首存在一对分隔线时，才作为 YAML Front Matter 跳过。 */
static inline u32 front_matter_end(const Byte *text, u32 length) {
  Line line = line_at(text, 0, length);
  if (!three_of(text, line, '-')) return 0;
  for (u32 pos = line.next; pos < length; pos = line.next) {
    line = line_at(text, pos, length);
    if (three_of(text, line, '-') || three_of(text, line, '.')) return line.next;
  }
  return 0;
}

static inline int in_fence(const Byte *text, Line line, u32 pos, Fence *fence) {
  if (pos == line.end || indented(text, line, pos)) return fence->count != 0;
  Byte marker = text[pos];
  if (marker != '`' && marker != '~') return fence->count != 0;
  u32 end = pos;
  while (end < line.end && text[end] == marker) end++;
  u32 count = end - pos;
  if (fence->count) {
    if (marker == fence->marker && count >= fence->count) {
      while (end < line.end && space(text[end])) end++;
      if (end == line.end) fence->count = 0;
    }
    return 1;
  }
  if (count < 3) return 0;
  if (marker == '`') {
    for (; end < line.end; end++) {
      if (text[end] == '`') return 0;
    }
  }
  *fence = (Fence){marker, count};
  return 1;
}

static inline void append(Output *out, const Byte *text, u32 count) {
  if (out->length > MAX_TEXT || count > MAX_TEXT - out->length) {
    // 宿主先校验长度，再读取内存。返回越限长度以获得明确提示。
    out->length = MAX_TEXT + 1;
    return;
  }
  for (u32 i = 0; i < count; i++) out->bytes[out->length + i] = text[i];
  out->length += count;
}

static inline u64 result(const Byte *bytes, u32 length) {
  return ((u64)(uptr)bytes << 32) | length;
}

#endif
