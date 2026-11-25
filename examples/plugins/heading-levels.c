/* SPDX-License-Identifier: MIT */
#include "yeyu.h"

static Byte input[MAX_TEXT];
static Byte output[MAX_TEXT];

Byte *alloc(u32 length) {
  return length <= sizeof(input) ? input : 0;
}

static u64 transform(const Byte *text, u32 length, int lower) {
  Output out = {output, 0};
  Fence fence = {0, 0};
  u32 start = front_matter_end(text, length);
  append(&out, text, start);
  for (u32 offset = start; offset < length && out.length <= MAX_TEXT;) {
    Line line = line_at(text, offset, length);
    offset = line.next;
    u32 pos = content_at(text, line);
    if (in_fence(text, line, pos, &fence) || indented(text, line, pos)) {
      append(&out, text + line.start, line.next - line.start);
      continue;
    }
    u32 marker_end = pos;
    while (marker_end < line.end && text[marker_end] == '#') marker_end++;
    u32 level = marker_end - pos;
    int heading = level >= 1 && level <= 6 &&
      (marker_end == line.end || space(text[marker_end]));
    if (heading && ((lower && level < 6) || (!lower && level > 1))) {
      append(&out, text + line.start, pos - line.start);
      if (lower) append(&out, (const Byte *)"#", 1);
      u32 body = lower ? pos : pos + 1;
      append(&out, text + body, line.next - body);
    } else {
      append(&out, text + line.start, line.next - line.start);
    }
  }
  return result(out.bytes, out.length);
}

u64 raise_headings(const Byte *text, u32 length) { return transform(text, length, 0); }
u64 lower_headings(const Byte *text, u32 length) { return transform(text, length, 1); }
