// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)


use super::base::*;
use super::font::Font;
use super::utils::indent;

use std::collections::HashMap;

pub struct Resources {
    pub id: Id,
    fonts: HashMap<String, Font>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            id: Id::new_0(),
            fonts: HashMap::new(),
        }
    }

    pub fn add_font(&mut self, name: &str, font: Font) {
        self.fonts.insert(name.to_string(), font);
    }

    pub fn to_string(&self, indent: usize) -> String {
        let fonts_str = self.fonts_to_string(indent);

        format!(concat!(
            "{} obj\n",
            "{}\n",
            "endobj"),
            self.id.to_string(),
            fonts_str
        )
    }

    fn fonts_to_string(&self, indent_size: usize) -> String {
        let dict_elms = (|| {
            let mut dict = String::new();
            for (name, font) in &self.fonts {
                dict.push_str(&indent(&format!(concat!(
                    "<< /{}\n",
                    "{}\n",
                    ">>"),
                    name,
                    font.to_string(1)),
                    indent_size + 1));
            }
            dict
        })();

        indent(&format!(concat!(
            "<< /Font\n",
            "{}\n",
            ">>"),
            dict_elms),
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
        let mut r = Resources::new();
        r.add_font("F0", Font::new("Times-Italic"));

        let ok = concat!(
            "0 0 obj\n",
            "<< /Font\n",
            "  << /F0\n",
            "    << /Type /Font\n",
            "       /Subtype /Type1\n",
            "       /BaseFont /Times-Italic\n",
            "    >>\n",
            "  >>\n",
            ">>\n",
            "endobj",);

        assert_eq!(r.to_string(0), ok);
    }
}
