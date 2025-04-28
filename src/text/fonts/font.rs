// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::utils::indent;

pub struct Font {
    base_font_name: String
}

impl Font {
    pub fn new(base_font_name: &str) -> Font {
        Font {
            base_font_name: base_font_name.to_string(),
        }
    }

    pub fn to_string(&self, indent_size: usize) -> String {
        indent(&format!(concat!(
            "<<\n",
            "  /Type /Font\n",
            "  /Subtype /Type1\n",
            "  /BaseFont /{}\n",
            ">>"),
            self.base_font_name),
            indent_size)
    }
}

//------------------------------------------------------------------------------
// tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_string() {
        let f = Font::new("Times-Italic");
        let ok = concat!(
            "<<\n",
            "  /Type /Font\n",
            "  /Subtype /Type1\n",
            "  /BaseFont /Times-Italic\n",
            ">>");

        assert_eq!(f.to_string(0), ok);
    }
}
