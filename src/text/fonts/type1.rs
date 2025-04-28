// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::syntax::*;
use crate::utils::*;

/// PDF32000-1:2008 9.6.2
pub struct Type1 {
    base_font_name: Name,
}

impl Type1 {
    pub fn new(base_font_name: &str) -> Self {
        Self {
            base_font_name: Name::new(base_font_name),
        }
    }
}

impl ToPdfString<Type1> for Type1 {
    fn to_pdf_string(&self) -> String {
        format!(
            concat!(
                "<<\n",
                "  /Type /Font\n",
                "  /Subtype /Type1\n",
                "  /BaseFont {}\n",
                ">>"
            ),
            self.base_font_name.to_pdf_string(),
        )
    }
}

//------------------------------------------------------------------------------
// tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pdf_string() {
        let f = Type1::new("Alial");
        let ok = concat!(
            "<<\n",
            "  /Type /Font\n",
            "  /Subtype /Type1\n",
            "  /BaseFont /Alial\n",
            ">>"
        );

        assert_eq!(f.to_pdf_string(), ok);
    }
}
