// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::graphics::color_spaces::ColorSpace;
use crate::syntax::objects::*;
use crate::text::fonts::*;
use crate::utils::*;

use std::collections::HashMap;

pub struct Resources {
    id: Id,
    fonts: HashMap<Name, Font>,
    color_spaces: HashMap<Name, ColorSpace>,
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
        self.fonts.insert(Name::new(name), font);
    }

    pub fn add_color_space(&mut self, name: &str, space: ColorSpace) {
        self.color_spaces.insert(Name::new(name), space);
    }

    pub fn to_pdf_string(&self) -> String {
        format!(
            concat!(
                "{} obj\n",
                "<<\n",
                "  /Font {}\n",
                "  /ColorSpace {}\n",
                ">>\n",
                "endobj"
            ),
            self.id.to_string(),
            indent_skip1(&self.get_font_string(), 1),
            indent_skip1(&self.get_color_space_string(), 1),
        )
    }

    fn get_color_space_string(&self) -> String {
        let mut dict = String::new();
        dict.push_str("<<\n");
        for space in &self.color_spaces {
            let cs_str = match &space.1 {
                ColorSpace::DeviceN(cs) => cs.id().to_ref_string(),
                ColorSpace::DeviceRGB(cs) => cs.to_pdf_string(),
                ColorSpace::Lab(cs) => cs.to_pdf_string(0),
                ColorSpace::Separation(cs) => cs.to_pdf_string(0),
            };

            dict.push_str(&format!(
                "{} {}\n",
                space.0.to_pdf_string(),
                &cs_str,
            ));
        }
        dict.push_str(">>");

        dict
    }

    fn get_font_string(&self) -> String {
        let dict_elms = (|| {
            let mut dict = String::new();
            for (name, font) in &self.fonts {
                dict.push_str(&format!(
                    concat!(
                        "<<\n",
                        "  {} {}\n",
                        ">>",
                    ),
                    name.to_pdf_string(),
                    indent_skip1(&font.to_pdf_string(), 1),
                ));
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
            match cs.1 {
                ColorSpace::DeviceN(cs) => cs.assign_ids(id_factory),
                _ => (),
            }
        }
    }

    fn get_objects(&self) -> Vec<&dyn PdfObject> {
        let mut list: Vec<&dyn PdfObject> = Vec::new();

        list.push(self);

        for cs in &self.color_spaces {
            match cs.1 {
                ColorSpace::DeviceN(cs) => list.append(&mut cs.get_objects()),
                _ => (),
            }
        }

        list
    }

    fn to_bytes(&self, _indent_depth: usize) -> Vec<u8> {
        self.to_pdf_string().into_bytes()
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
        let mut r = Resources::new();
        r.add_font("F0", Font::new_type1("Times-Italic"));

        //r.add_color_space("CS0", );

        let ok = concat!(
            "0 0 obj\n",
            "<<\n",
            "  /Font <<\n",
            "    /F0 <<\n",
            "      /Type /Font\n",
            "      /Subtype /Type1\n",
            "      /BaseFont /Times-Italic\n",
            "    >>\n",
            "  >>\n",
            "  /ColorSpace <<\n",
            "  >>\n",
            ">>\n",
            "endobj"
        );

        assert_eq!(r.to_pdf_string(), ok);
    }
}
