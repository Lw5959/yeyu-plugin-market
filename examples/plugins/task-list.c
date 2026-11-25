/* SPDX-License-Identifier: MIT */
#include "yeyu.h"

static Byte input[MAX_TEXT];
static Byte output[MAX_TEXT];

Byte *alloc(u32 length) {
  return length <= sizeof(input) ? input : 0;
}

enum Operation { MAKE, COMPLETE, RESET };

static u32 list_content(const Byte *text, u32 start, u32 end) {
  u32 pos = start;
  Byte ch = text[pos];
  if (ch == '-' || ch == '*' || ch == '+') {
    pos++;
  } else {
    while (pos < end && pos - start < 9 && text[pos] >= '0' && text[pos] <= '9') pos++;
    if (pos == start || pos == end || (text[pos] != '.' && text[pos] != ')')) return start;
    pos++;
  }
  if (pos == end || !space(text[pos])) return start;
  while (pos < end && space(text[pos])) pos++;
  return pos;
}

static int task_at(const Byte *text, u32 pos, u32 end) {
  return end - pos >= 3 && text[pos] == '[' && text[pos + 2] == ']' &&
    (text[pos + 1] == ' ' || text[pos + 1] == 'x' || text[pos + 1] == 'X') &&
    (pos + 3 == end || space(text[pos + 3]));
}

static int markup_at(const Byte *text, u32 pos, u32 end) {
  Byte ch = text[pos];
  if (ch == '#' || ch == '>' || ch == '<' || ch == '|' || ch == '[') return 1;
  // 分隔线和 Setext 标题线保留原文。
  if (ch != '-' && ch != '*' && ch != '_' && ch != '=') return 0;
  for (u32 i = pos; i < end; i++) {
    if (text[i] != ch && !space(text[i])) return 0;
  }
  return 1;
}

static u64 transform(const Byte *text, u32 length, enum Operation operation) {
  Output out = {output, 0};
  Fence fence = {0, 0};
  u32 start = front_matter_end(text, length);
  append(&out, text, start);
  for (u32 offset = start; offset < length && out.length <= MAX_TEXT;) {
    Line line = line_at(text, offset, length);
    offset = line.next;
    u32 pos = content_at(text, line);
    if (in_fence(text, line, pos, &fence) || pos == line.end || indented(text, line, pos) ||
        markup_at(text, pos, line.end)) {
      append(&out, text + line.start, line.next - line.start);
      continue;
    }
    u32 content = list_content(text, pos, line.end);
    int has_task = content != pos && task_at(text, content, line.end);
    if (has_task && operation != MAKE) {
      const Byte *mark = (const Byte *)(operation == COMPLETE ? "x" : " ");
      append(&out, text + line.start, content + 1 - line.start);
      append(&out, mark, 1);
      append(&out, text + content + 2, line.next - content - 2);
    } else if (operation == MAKE && !has_task && content < line.end) {
      append(&out, text + line.start, content - line.start);
      if (content == pos) append(&out, (const Byte *)"- ", 2);
      append(&out, (const Byte *)"[ ] ", 4);
      append(&out, text + content, line.next - content);
    } else {
      append(&out, text + line.start, line.next - line.start);
    }
  }
  return result(out.bytes, out.length);
}

u64 make_tasks(const Byte *text, u32 length) { return transform(text, length, MAKE); }
u64 complete_tasks(const Byte *text, u32 length) { return transform(text, length, COMPLETE); }
u64 reset_tasks(const Byte *text, u32 length) { return transform(text, length, RESET); }
