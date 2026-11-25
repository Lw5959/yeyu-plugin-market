// SPDX-License-Identifier: MIT
use pulldown_cmark::{Event, Parser, Tag};
use yeyu_plugin_workflows::{
    export_commands, line_ending, markdown_options, physical_lines, table_layout,
};

export_commands!(format_document => format, format_files => format);

fn format(source: &str) -> Option<String> {
    match format_spacing(source) {
        Some(spaced) => Some(table_layout::align(&spaced).unwrap_or(spaced)),
        None => table_layout::align(source),
    }
}

fn format_spacing(source: &str) -> Option<String> {
    let mut blocks = Vec::new();
    let mut depth = 0;
    for (event, span) in Parser::new_ext(source, markdown_options()).into_offset_iter() {
        match event {
            Event::Start(tag) => {
                if depth == 0 {
                    blocks.push((span, matches!(tag, Tag::Heading { .. })));
                }
                depth += 1;
            }
            Event::End(_) => depth -= 1,
            Event::Rule if depth == 0 => blocks.push((span, false)),
            _ => {}
        }
    }
    let mut output = String::new();
    let mut copied = 0;
    for pair in blocks.windows(2) {
        let [(left, left_heading), (right, right_heading)] = pair else {
            unreachable!()
        };
        let Some(gap) = source.get(left.end..right.start) else {
            continue;
        };
        if !gap
            .bytes()
            .all(|byte| matches!(byte, b' ' | b'\t' | b'\r' | b'\n'))
            || !source[..right.start].ends_with(['\r', '\n'])
        {
            continue;
        }
        // 只编辑已识别的顶层块之间的空行，列表内部、代码、YAML 和段落硬换行沿用原文。
        let needed = if source[..left.end].ends_with(['\r', '\n']) {
            1
        } else {
            2
        };
        let mut lines = physical_lines(gap);
        let breaks = lines.by_ref().filter(|(_, end)| !end.is_empty()).count();
        if breaks <= needed && (!(*left_heading || *right_heading) || breaks == needed) {
            continue;
        }
        output.push_str(&source[copied..left.end]);
        let mut input = physical_lines(gap);
        let (tail, existing) = input.next().unwrap_or_default();
        let ending = if !existing.is_empty() {
            existing
        } else if source[..left.end].ends_with("\r\n") {
            "\r\n"
        } else if source[..left.end].ends_with('\r') {
            "\r"
        } else if source[..left.end].ends_with('\n') {
            "\n"
        } else {
            line_ending(source)
        };
        if needed == 2 {
            // 第一行属于前一个块，保留它的行尾空格和换行类型。
            output.push_str(tail);
            output.push_str(ending);
        }
        output.push_str(if needed == 2 {
            input
                .next()
                .map_or(ending, |(_, end)| if end.is_empty() { ending } else { end })
        } else {
            ending
        });
        copied = right.start;
    }
    if copied == 0 {
        return None;
    }
    output.push_str(&source[copied..]);
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_heading_spacing_and_preserves_other_content() {
        let source = "---\r\ntitle: 示例\r\n\r\n---\r\n# 标题\r\n正文  \n下一行\n\n\n## 子标题\n```md\n\n\n# 代码\n```\n";
        let expected = "---\r\ntitle: 示例\r\n\r\n---\r\n\r\n# 标题\r\n\r\n正文  \n下一行\n\n## 子标题\n\n```md\n\n\n# 代码\n```\n";
        let formatted = format(source).unwrap();
        assert_eq!(formatted, expected);
        assert!(format(&formatted).is_none());
    }

    #[test]
    fn keeps_list_quote_and_unknown_blocks_as_written() {
        let source =
            "- 要点\n\n  说明\n  # 列表内标题\n\n> 正文\n> ## 引用内标题\n\n[自定义]: abc\n\n结束";
        assert!(format(source).is_none());
    }
}
