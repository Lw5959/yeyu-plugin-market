# 标题层级

在插件市场安装并启用「标题层级」。从行首选中一个或多个章节，运行「标题升一级」或「标题降一级」。例如 `### 验收` 升一级后变成 `## 验收`。标题层级始终在一级到六级之间。

命令调整由 `#` 开头的 ATX 标题。正文、Setext 标题、YAML Front Matter、围栏代码块和缩进代码行保留原文。

## 开发和重建

源码在 [heading-levels.c](heading-levels.c)，运行包在 [heading-levels.yeyu-plugin.json](heading-levels.yeyu-plugin.json)。`raise_headings` 和 `lower_headings` 分别删减或增加一个起始 `#`。解析检查 `#` 的数量与分隔空白，并复用 [yeyu.h](yeyu.h) 的围栏判断。

```sh
plugin_build=$(mktemp -d)
clang --target=wasm32-unknown-unknown -ffreestanding -fno-builtin -O2 \
  -c examples/plugins/heading-levels.c -o "$plugin_build/heading-levels.o"
wasm-ld --no-entry --export-memory --export=alloc --export=raise_headings \
  --export=lower_headings --strip-all "$plugin_build/heading-levels.o" \
  -o "$plugin_build/heading-levels.wasm"
pnpm plugin:pack examples/plugins/heading-levels.yeyu-plugin.json "$plugin_build/heading-levels.wasm"
```

测试升降边界、标题末尾的 `#`、标题前的缩进、混合换行，以及围栏内看起来像标题的文字。
