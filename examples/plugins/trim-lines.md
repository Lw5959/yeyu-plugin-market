# 清理行尾空白

在插件市场安装并启用「清理行尾空白」。选择当前选区或全文，运行同名命令。每行最后的空格和 Tab 会移除，换行符和其他字符会保留。该操作也会移除 Markdown 的双空格硬换行，应用之前可在差异中核对。

`第一行  \r\n第二行\t` 会变成 `第一行\r\n第二行`，其中 `\r\n` 和 `\t` 表示换行与 Tab。

## 开发和重建

源码在 [trim-lines.c](trim-lines.c)，插件包在 [trim-lines.yeyu-plugin.json](trim-lines.yeyu-plugin.json)。`trim_lines` 用两个位置在同一个缓冲区内改写，遇到换行时回退到行尾空白之前。输入和输出的长度按字节计算。

```sh
plugin_build=$(mktemp -d)
clang --target=wasm32-unknown-unknown -ffreestanding -fno-builtin -O2 \
  -c examples/plugins/trim-lines.c -o "$plugin_build/trim-lines.o"
wasm-ld --no-entry --export-memory --export=alloc --export=trim_lines \
  --strip-all "$plugin_build/trim-lines.o" -o "$plugin_build/trim-lines.wasm"
pnpm plugin:pack examples/plugins/trim-lines.yeyu-plugin.json "$plugin_build/trim-lines.wasm"
```

测试空行、只有空格的一行、末行无换行，以及 LF、CRLF、CR 混用。
