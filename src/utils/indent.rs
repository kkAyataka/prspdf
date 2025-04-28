// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

// Add indent_depth * 2 spaces indent to all lines.
pub fn indent(s: &str, indent_depth: usize) -> String {
    indent_all_or_skip1(s, indent_depth, false)
}

// Add indent_depth * 2 spaces indent to lines that are excluded from the first line.
pub fn indent_skip1(s: &str, indent_depth: usize) -> String {
    indent_all_or_skip1(s, indent_depth, true)
}

/// real implement
fn indent_all_or_skip1(s: &str, indent_depth: usize, skip1: bool) -> String {
    let lines = s.lines();

    let mut result = String::new();
    let mut is_first = true;
    for line in lines {
        if is_first && skip1 {
            result.push_str(&format!("{}\n", line));
        } else {
            result.push_str(&format!("{}{}\n", "  ".repeat(indent_depth), line));
        }
        is_first = false;
    }

    if !s.ends_with('\n') {
        result.pop();
    }

    result
}
