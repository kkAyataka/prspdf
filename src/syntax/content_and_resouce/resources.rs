// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::syntax::objects::base::*;
use crate::graphics::color_spaces::DeviceN;
use crate::text::Font;
use crate::utils::{indent, ToPdfString};

use std::collections::HashMap;

pub struct Resources {
    pub id: Id,
    fonts: HashMap<String, Font>,
    color_spaces: HashMap<Name, DeviceN>,
}

impl Resources {
    pub fn new() -> Resources {
        Resources {
            id: Id::new_0(),
            fonts: HashMap::new(),
            color_spaces: HashMap::new(),
        }
    }

    pub fn add_font(&mut self, name: &str, font: Font) {
        self.fonts.insert(name.to_string(), font);
    }

    pub fn add_color_space(&mut self, name: &str, space: DeviceN) {
        self.color_spaces.insert(Name::new(name), space);
    }

    pub fn to_string(&self, indent: usize) -> String {
        format!(concat!(
            "{} obj\n",
            "<<\n",
            "  /Font {}\n",
            "  /ColorSpace {}\n",
            "\n",
            ">>\n",
            "endobj"),
            self.id.to_string(),
            self.fonts_to_string(indent),
            self.get_color_space_string(indent),
        )
    }

    fn get_color_space_string(&self, indent_depth: usize) -> String {
        let mut dict = String::new();
        dict.push_str("<<\n");
        for space in &self.color_spaces {
            dict.push_str(&format!("{} {}", space.0.to_pdf_string(), space.1.id().to_ref_string()));
        }
        dict.push_str(">>");

        dict
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

        dict_elms
    }
}

impl PdfObject for Resources {
    fn id(&self) -> &Id {
        &self.id
    }

    fn assign_ids(&mut self, id_factory: &mut IdFactory) {
        self.id = id_factory.next_id();
        for cs in &mut self.color_spaces {
            cs.1.assign_ids(id_factory);
        }
    }

    fn get_objects(&self) -> Vec<&dyn PdfObject> {
        let mut list: Vec<&dyn PdfObject> = Vec::new();

        list.push(self);

        for cs in &self.color_spaces {
            list.append(&mut cs.1.get_objects());
        }

        list
    }

    fn to_bytes(&self, indent_depth: usize) -> Vec<u8> {
        self.to_string(indent_depth).into_bytes()
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
