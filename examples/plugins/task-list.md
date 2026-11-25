# 任务清单

在插件市场安装并启用「任务清单」。选中要整理的文字或列表，运行「转为任务」。例如两行 `读样稿` 和 `- 看差异` 会变成 `- [ ] 读样稿` 和 `- [ ] 看差异`。

「标记任务为完成」将所选任务的状态改为 `[x]`，「重置任务为待办」改为 `[ ]`。命令支持带项目符号或数字标记的行，保留原有标记、文本和换行。适合从行首选中完整的待办段落，缩进达到 4 格或 Tab 的代码行会跳过。

## 开发和重建

源码在 [task-list.c](task-list.c)，运行包在 [task-list.yeyu-plugin.json](task-list.yeyu-plugin.json)。三个导出共用行扫描，`make_tasks` 插入任务前缀，`complete_tasks` 和 `reset_tasks` 只改匹配的任务状态字符。[yeyu.h](yeyu.h) 提供换行、围栏和输入输出辅助函数。

```sh
plugin_build=$(mktemp -d)
clang --target=wasm32-unknown-unknown -ffreestanding -fno-builtin -O2 \
  -c examples/plugins/task-list.c -o "$plugin_build/task-list.o"
wasm-ld --no-entry --export-memory --export=alloc --export=make_tasks \
  --export=complete_tasks --export=reset_tasks --strip-all "$plugin_build/task-list.o" \
  -o "$plugin_build/task-list.wasm"
pnpm plugin:pack examples/plugins/task-list.yeyu-plugin.json "$plugin_build/task-list.wasm"
```

测试普通文字、混合列表、已有完成任务和围栏代码块。尝试在代码块外选中多行，对照差异确认代码块原样保留。
