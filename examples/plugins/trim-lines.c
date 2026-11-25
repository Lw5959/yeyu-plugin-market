/* SPDX-License-Identifier: MIT */
/* 页屿插件接口 v1。输入与输出共享缓冲区，保留原本的 LF、CRLF 或 CR。 */
typedef __UINT32_TYPE__ u32;
typedef __UINT64_TYPE__ u64;
typedef __UINTPTR_TYPE__ uptr;

static unsigned char buffer[4 * 1024 * 1024];

unsigned char *alloc(u32 length) {
  return length <= sizeof(buffer) ? buffer : 0;
}

u64 trim_lines(unsigned char *text, u32 length) {
  u32 writing = 0;
  u32 end = 0;
  for (u32 reading = 0; reading < length; reading++) {
    unsigned char ch = text[reading];
    if (ch == '\r' || ch == '\n') {
      writing = end;
      text[writing++] = ch;
      end = writing;
    } else {
      text[writing++] = ch;
      if (ch != ' ' && ch != '\t') {
        end = writing;
      }
    }
  }
  return ((u64)(uptr)text << 32) | end;
}
