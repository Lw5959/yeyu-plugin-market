// SPDX-License-Identifier: MIT
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};
use yeyu_plugin_workflows::{export_commands, line_ending, markdown_options};

export_commands!(update_toc => update_document_toc, remove_toc => remove_document_toc);

const BEGIN: &str = "[yeyu-toc-begin]: #yeyu-toc-begin";
const END: &str = "[yeyu-toc-end]: #yeyu-toc-end";

struct Heading {
    title: String,
    level: u8,
    range: Range<usize>,
}

fn line_start(source: &str, index: usize) -> bool {
    index == 0 || source[..index].ends_with(['\r', '\n'])
}

fn managed_block(source: &str, parser: &Parser<'_>) -> Result<Option<Range<usize>>, ()> {
    let definitions = parser.reference_definitions();
    match (
        definitions.get("yeyu-toc-begin"),
        definitions.get("yeyu-toc-end"),
    ) {
        (None, None) => Ok(None),
        (Some(begin), Some(end)) => {
            // 只写入完整的生成区块，避免误覆盖手写段落或半截标记。
            let start = begin.span.start;
            let middle = start + BEGIN.len();
            let stop = end.span.start;
            let finish = stop + END.len();
            if !line_start(source, start)
                || !line_start(source, stop)
                || source.get(start..middle) != Some(BEGIN)
                || source.get(stop..finish) != Some(END)
                || !source[finish..].starts_with(['\r', '\n']) && finish != source.len()
                || !source.get(middle..stop).is_some_and(|body| {
                    body.split(['\r', '\n']).all(|line| {
                        let line = line.trim_start();
                        line.is_empty() || line.starts_with("- [")
                    })
                })
            {
                return Err(());
            }
            Ok(Some(start..finish))
        }
        _ => Err(()),
    }
}

fn heading_base(title: &str) -> String {
    // 与页屿正文中的 headingId 约定一致，标题保留字母、数字、连字符及下划线。
    let mut base = String::new();
    let mut whitespace = false;
    for character in title.trim().to_lowercase().chars() {
        if character.is_whitespace() {
            if !whitespace {
                base.push('-');
            }
            whitespace = true;
        } else if character.is_alphanumeric() || matches!(character, '_' | '-') {
            base.push(character);
            whitespace = false;
        }
    }
    if base.is_empty() {
        "heading".into()
    } else {
        base
    }
}

fn heading_ids(headings: &[Heading]) -> Vec<String> {
    let mut used = HashSet::new();
    let mut versions = HashMap::new();
    headings
        .iter()
        .map(|heading| {
            let base = heading_base(&heading.title);
            if used.insert(base.clone()) {
                return base;
            }
            let counter = versions.entry(base.clone()).or_insert(2);
            loop {
                let id = format!("{base}-{counter}");
                *counter += 1;
                if used.insert(id.clone()) {
                    return id;
                }
            }
        })
        .collect()
}

fn escape_label(title: &str) -> String {
    let mut label = String::new();
    for character in title.chars() {
        if matches!(
            character,
            '\\' | '[' | ']' | '*' | '_' | '`' | '<' | '>' | '~'
        ) {
            label.push('\\');
        }
        label.push(character);
    }
    label
}

fn contents(headings: &[Heading], start: usize, newline: &str) -> String {
    let mut block = String::from(BEGIN);
    block.push_str(newline);
    block.push_str(newline);
    let mut stack = Vec::new();
    let ids = heading_ids(headings);
    for (heading, id) in headings.iter().zip(ids).skip(start) {
        if heading.title.trim().is_empty() {
            continue;
        }
        while stack.last().is_some_and(|&level| level > heading.level) {
            stack.pop();
        }
        if stack.last() != Some(&heading.level) {
            stack.push(heading.level);
        }
        block.push_str(&"  ".repeat(stack.len() - 1));
        block.push_str(&format!(
            "- [{}](#{id})",
            escape_label(heading.title.trim())
        ));
        block.push_str(newline);
    }
    block.push_str(newline);
    block.push_str(END);
    block
}

fn update_document_toc(source: &str) -> Option<String> {
    let parser = Parser::new_ext(source, markdown_options());
    let managed = managed_block(source, &parser).ok()?;
    let mut headings = Vec::new();
    let mut current = None;
    let mut marker = None;
    let mut front_end = 0;
    for (event, range) in parser.into_offset_iter() {
        match event {
            Event::Start(Tag::Heading { level, .. }) => {
                current = Some(Heading {
                    title: String::new(),
                    level: level as u8,
                    range,
                })
            }
            Event::End(TagEnd::Heading(_)) => headings.extend(current.take()),
            Event::Start(Tag::MetadataBlock(_)) if range.start == 0 => front_end = range.end,
            Event::Start(Tag::Paragraph)
                if marker.is_none()
                    && line_start(source, range.start)
                    && source[range.clone()].trim().eq_ignore_ascii_case("[TOC]") =>
            {
                marker =
                    Some(range.start..range.start + source[range.start..range.end].trim_end().len())
            }
            Event::Text(text)
            | Event::Code(text)
            | Event::InlineMath(text)
            | Event::InlineHtml(text) => {
                if let Some(heading) = &mut current {
                    heading.title.push_str(&text);
                }
            }
            Event::SoftBreak | Event::HardBreak => {
                if let Some(heading) = &mut current {
                    heading.title.push(' ');
                }
            }
            _ => {}
        }
    }
    let first_title = headings
        .first()
        .filter(|heading| heading.level == 1 && line_start(source, heading.range.start));
    let skip = usize::from(first_title.is_some());
    if headings.len() <= skip {
        return managed.map(|range| replaced(source, range, ""));
    }
    let newline = line_ending(source);
    let block = contents(&headings, skip, newline);
    if let Some(range) = managed.or(marker) {
        let mut replacement = block;
        if !source[range.end..].is_empty() && !source[range.end..].starts_with(['\r', '\n']) {
            replacement.push_str(newline);
        }
        return Some(replaced(source, range, &replacement));
    }
    let at = first_title.map_or(front_end, |heading| heading.range.end);
    let before = &source[..at];
    let after = &source[at..];
    let mut out = String::with_capacity(source.len() + block.len() + newline.len() * 4);
    out.push_str(before);
    if !before.is_empty() {
        if !before.ends_with(['\r', '\n']) {
            out.push_str(newline);
        }
        out.push_str(newline);
    }
    out.push_str(&block);
    if !after.is_empty() {
        out.push_str(newline);
        if !after.starts_with(['\r', '\n']) {
            out.push_str(newline);
        }
        out.push_str(after);
    }
    Some(out)
}

fn replaced(source: &str, range: Range<usize>, block: &str) -> String {
    let mut output = source.to_owned();
    output.replace_range(range, block);
    output
}

fn remove_document_toc(source: &str) -> Option<String> {
    let parser = Parser::new_ext(source, markdown_options());
    let range = managed_block(source, &parser).ok()??;
    Some(replaced(source, range, ""))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_and_updates_only_the_managed_directory() {
        let text = "---\r\ntitle: 指南\r\n---\r\n# 指南\r\n\r\n[TOC]\r\n\r\n## 安装\r\n### 测试 `API`\r\n## 安装\r\n\r\n```md\r\n## 假标题\r\n```\r\n";
        let output = update_document_toc(text).unwrap();
        assert!(
            output.contains("- [安装](#安装)\r\n  - [测试 API](#测试-api)\r\n- [安装](#安装-2)")
        );
        assert!(!output.contains("[假标题]"));
        assert!(output.starts_with("---\r\ntitle: 指南\r\n---\r\n# 指南\r\n"));
        assert_eq!(update_document_toc(&output).unwrap(), output);
        let removed = remove_document_toc(&output).unwrap();
        assert!(!removed.contains("yeyu-toc-begin"));
        assert!(removed.ends_with("## 假标题\r\n```\r\n"));
        let hand_written = format!("{BEGIN}\n\n正文必须保留。\n\n{END}\n\n## 资料");
        assert!(update_document_toc(&hand_written).is_none());
    }

    #[test]
    fn creates_a_hidden_block_after_the_first_title() {
        let result = update_document_toc("# 路线图\n\n目标摘要\n\n功能\n----\n").unwrap();
        assert!(result.starts_with(&format!("# 路线图\n\n{BEGIN}")));
        assert!(result.contains("- [功能](#功能)"));
        assert!(result.contains("目标摘要"));
        assert_eq!(update_document_toc(&result).unwrap(), result);
    }
}
