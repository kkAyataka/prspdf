// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::syntax::functions;
use crate::utils::indent;
use super::Lab;

/// PDF32000-1:2008 8.6.6.4
///
/// ```text
/// [
///   /Separation
/// %name
///   /Orange
/// %alternateSpace
///   [
///     /Lab
///     <<
///       /WhitePoint [0.964203 1.0 0.824905]
///       /BlackPoint [0.0 0.0 0.0]
///       /Range [-100 100 -100 100]
///     >>
///   ]
/// %tintTransform
///   <<
///     /FunctionType 2
///     /C0 [100.0 0.0 0.0]
///     /C1 [65.0 58.0 88.0]
///     /N 1.0
///   >>
/// ]
/// ```
pub struct Separation {
    name: String,
    alt_space: Lab,
    tint_transform: functions::Type2,
}

impl Separation {
    pub fn new(name: &str, alt_space: Lab, tint_transform: functions::Type2) -> Separation {
        Separation {
            name: name.to_string(),
            alt_space,
            tint_transform,
        }
    }

    pub fn to_pdf_string(&self, indent_depth: usize) -> String {
        indent(&format!(concat!(
            "[\n",
            "  /Separation\n",
            "  /{}\n",
            "  {}\n",
            "  {}\n",
            "]"),
            self.name,
            self.alt_space.to_pdf_string(indent_depth + 1),
            String::from_utf8(self.tint_transform.to_bytes(indent_depth + 1)).unwrap(),
        ), indent_depth)
    }

    pub fn to_bytes(&self, indent_depth: usize) -> Vec<u8> {
        self.to_pdf_string(indent_depth).into_bytes()
    }
}
