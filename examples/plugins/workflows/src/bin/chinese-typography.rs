// SPDX-License-Identifier: MIT
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use yeyu_plugin_workflows::{export_commands, markdown_options};

export_commands!(space_markdown => space_markdown_text);

fn is_cjk(character: char) -> bool {
    matches!(
        character as u32,
        0x3040..=0x30FF | 0x3400..=0x4DBF | 0x4E00..=0x9FFF |
        0xAC00..=0xD7AF | 0xF900..=0xFAFF | 0x20000..=0x2FA1F | 0x30000..=0x323AF
    )
}

fn spaces_in(text: &str, start: usize, positions: &mut Vec<usize>) {
    let mut offset = start;
    for word in text.split_inclusive(char::is_whitespace) {
        // 裸链接、邮件、标签与路径可能没有独立的 Markdown 节点。
        let protected = word.contains(['@', '#', '/', '\\']);
        if !protected {
            let mut previous = '\0';
            for (index, character) in word.char_indices() {
                if (is_cjk(previous) && character.is_ascii_alphanumeric())
                    || (previous.is_ascii_alphanumeric() && is_cjk(character))
                {
                    positions.push(offset + index);
                }
                previous = character;
            }
        }
        offset += word.len();
    }
}

fn space_markdown_text(source: &str) -> Option<String> {
    let mut positions = Vec::new();
    let mut protected = 0;
    for (event, range) in Parser::new_ext(source, markdown_options()).into_offset_iter() {
        match event {
            Event::Start(
                Tag::CodeBlock(_)
                | Tag::HtmlBlock
                | Tag::MetadataBlock(_)
                | Tag::Link { .. }
                | Tag::Image { .. },
            ) => protected += 1,
            Event::End(
                TagEnd::CodeBlock
                | TagEnd::HtmlBlock
                | TagEnd::MetadataBlock(_)
                | TagEnd::Link
                | TagEnd::Image,
            ) => protected -= 1,
            // 只在未经解码的源码文本中插入空格，实体、转义和其他标记保留字节。
            Event::Text(text)
                if protected == 0 && source.get(range.clone()) == Some(text.as_ref()) =>
            {
                spaces_in(&text, range.start, &mut positions)
            }
            _ => {}
        }
    }
    if positions.is_empty() {
        return None;
    }
    positions.sort_unstable();
    positions.dedup();
    let mut output = String::with_capacity(source.len() + positions.len());
    let mut last = 0;
    for position in positions {
        output.push_str(&source[last..position]);
        output.push(' ');
        last = position;
    }
    output.push_str(&source[last..]);
    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_code_links_metadata_math_and_mixed_line_endings() {
        let prefix = "---\r\ntitle: 开发Rust\r\n---\r\n";
        let protected = "`变量A` [文档A](guideA.md) ![截图1](图A.png) \
            [[资料A]] https://example.org/中文A #标签A $温度T$\n\n```js\nconst 文档A = 1;\n```\r\n";
        let input = format!("{prefix}# 发布API文档\n\n{protected}\n今天转换50行。");
        let expected = format!("{prefix}# 发布 API 文档\n\n{protected}\n今天转换 50 行。");
        let output = space_markdown_text(&input).unwrap();
        assert_eq!(output, expected);
        assert!(space_markdown_text(&output).is_none());
    }
}
