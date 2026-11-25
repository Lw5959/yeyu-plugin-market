// SPDX-License-Identifier: MIT
use crate::{markdown_options, physical_lines};
use pulldown_cmark::{Alignment, Event, Parser, Tag};
use unicode_width::UnicodeWidthStr;

fn cells(line: &str) -> Vec<&str> {
    let line = line.trim_matches([' ', '\t']);
    let start = usize::from(line.starts_with('|'));
    let mut from = start;
    let mut cells = Vec::new();
    for (at, byte) in line.bytes().enumerate().skip(from) {
        // GFM 中反斜杠紧邻的竖线属于单元格，包括代码跨度里的竖线。
        if byte == b'|' && (at == 0 || line.as_bytes()[at - 1] != b'\\') {
            cells.push(line[from..at].trim_matches([' ', '\t']));
            from = at + 1;
        }
    }
    if from < line.len() {
        cells.push(line[from..].trim_matches([' ', '\t']));
    }
    cells
}

fn format_table(text: &str, alignments: &[Alignment]) -> Option<String> {
    let rows: Vec<_> = physical_lines(text)
        .map(|(line, ending)| (cells(line), ending))
        .collect();
    if rows.len() < 2 || rows.iter().any(|(row, _)| row.len() != alignments.len()) {
        return None;
    }
    let mut widths = vec![5; alignments.len()];
    for (index, (row, _)) in rows.iter().enumerate() {
        if index != 1 {
            for (width, cell) in widths.iter_mut().zip(row) {
                *width = (*width).max(cell.width());
            }
        }
    }
    if (widths.iter().sum::<usize>() + widths.len() * 3 + 3).saturating_mul(rows.len())
        > 4 * 1024 * 1024
    {
        return None;
    }
    let mut output = String::new();
    for (index, (row, ending)) in rows.into_iter().enumerate() {
        output.push('|');
        for ((cell, width), alignment) in row.into_iter().zip(&widths).zip(alignments) {
            output.push(' ');
            if index == 1 {
                let left = matches!(alignment, Alignment::Left | Alignment::Center);
                let right = matches!(alignment, Alignment::Right | Alignment::Center);
                if left {
                    output.push(':');
                }
                output.extend(std::iter::repeat_n(
                    '-',
                    width - usize::from(left) - usize::from(right),
                ));
                if right {
                    output.push(':');
                }
            } else {
                output.push_str(cell);
                output.extend(std::iter::repeat_n(' ', width - cell.width()));
            }
            output.push_str(" |");
        }
        output.push_str(ending);
    }
    (output != text).then_some(output)
}

pub fn align(source: &str) -> Option<String> {
    let mut output = String::new();
    let mut copied = 0;
    let mut depth = 0;
    for (event, span) in Parser::new_ext(source, markdown_options()).into_offset_iter() {
        match event {
            Event::Start(tag) => {
                if let Tag::Table(alignments) = tag
                    && depth == 0
                    && (span.start == 0 || source[..span.start].ends_with(['\r', '\n']))
                    && let Some(formatted) = format_table(&source[span.clone()], &alignments)
                {
                    output.push_str(&source[copied..span.start]);
                    output.push_str(&formatted);
                    copied = span.end;
                }
                depth += 1;
            }
            Event::End(_) => depth -= 1,
            _ => {}
        }
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
    fn aligns_cjk_and_preserves_cells_alignment_and_newlines() {
        let source = "|名称|状态|\r\n|:-|--:|\n|截图|a\\|b|\r|很长的标题|\x60x\x60|\n";
        let formatted = align(source).unwrap();
        assert_eq!(
            formatted,
            "| 名称       | 状态  |\r\n| :--------- | ----: |\n| 截图       | a\\|b  |\r| 很长的标题 | \x60x\x60   |\n"
        );
        assert!(align(&formatted).is_none());
    }

    #[test]
    fn ignores_code_and_ragged_tables() {
        let source = "```md\n| 名称|状态 |\n|---|---|\n| a|b |\n```\n\n|a|b|\n|---|---|\n|a|\n";
        assert!(align(source).is_none());
    }
}
