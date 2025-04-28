// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::utils::ToPdfString;

mod type1;
pub use type1::Type1;

pub enum Font {
    Type1(type1::Type1),
}

impl Font {
    pub fn new_type1(base_font_name: &str) -> Self {
        Font::Type1(type1::Type1::new(base_font_name))
    }
}

impl ToPdfString<Font> for Font {
    fn to_pdf_string(&self) -> String {
        match self {
            Font::Type1(f) => f.to_pdf_string(),
        }
    }
}
