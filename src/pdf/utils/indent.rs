// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)


/// Indents each line of the given string by a specified number of spaces.
///
/// # Arguments
///
/// * `s` - The input string to be indented.
/// * `indent_size` - The number of indentation levels to apply. Each level corresponds to two spaces.
///
/// # Returns
///
/// A new `String` where each line of the input string is prefixed with the specified indentation.
/// If the input string ends with a newline, the resulting string will preserve that trailing newline.
/// ```
pub fn indent(s:&str, indent_depth: usize) -> String {
    let lines = s.lines();

    let mut result = String::new();
    for line in lines {
        result.push_str(&format!("{}{}\n", "  ".repeat(indent_depth), line));
    }

    if !s.ends_with('\n') {
        result.pop();
    }

    result
}
