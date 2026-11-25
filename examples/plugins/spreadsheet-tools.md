# 表格转换

把 Excel、Numbers 复制的单元格或 CSV 数据写成 Markdown 表格。要在电子表格里继续处理，也可以把 Markdown 表格导回 CSV。

## 试用

在「编辑 → 插件市场」的「内置插件」安装「表格转换」，进入「已安装」启用。把下方 CSV 粘贴到文档，选中含表头的三行。在「编辑 → 运行插件」选择「CSV 转 Markdown 表格」并审阅修改。

```csv
名称,负责人,说明
发布,林晨,"包含逗号, 管道符 | 和 ""引号"""
复核,,空单元格保留
```

```markdown
| 名称 | 负责人 | 说明 |
| --- | --- | --- |
| 发布 | 林晨 | 包含逗号, 管道符 \| 和 "引号" |
| 复核 |  | 空单元格保留 |
```

从电子表格复制单元格时，粘贴为纯文本，再运行「电子表格单元格转 Markdown」。要导回 CSV，选中整张 Markdown 表格，包含分隔行，再运行「Markdown 表格转 CSV」。插件处理至少两列且列数一致的数据。带引号的换行和制表符以字符实体保存在 Markdown 中，转回 CSV 时恢复。

## 开发

源码为 [spreadsheet-tools.mjs](spreadsheet-tools.mjs)。三个命令都是接收和返回文本的同步函数。修改模块后，从仓库根目录执行下面的命令。在页屿中从文件重装，核对含引号、空值和管道符的转换。使用同一数据再导回 CSV，可核对内容保留情况。[页屿插件开发指南](../../docs/plugins.md)

```sh
pnpm plugin:pack examples/plugins/spreadsheet-tools.yeyu-plugin.json examples/plugins/spreadsheet-tools.mjs
```

转换流程参考了 CSV 转 Markdown 和电子表格双向转换工具。实现与测试样稿由页屿编写。[csvtomd](https://github.com/mplewis/csvtomd) · [Table to Markdown Converter](https://github.com/turhe123/table-to-markdown-local)
