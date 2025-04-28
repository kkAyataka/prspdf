// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::syntax::objects::base::*;
use crate::utils::indent;

pub struct Contents {
    id: Id,
    operators: Vec<String>,
}

impl Contents {
    pub fn new() -> Contents {
        Contents { id: Id::new_0(), operators: Vec::new() }
    }

    pub fn set_fill_color_space(&mut self, name: &str) {
        self.operators.push(format!("/{name} cs"));
    }

    pub fn set_fill_color_space_color<
        const COLOR_NUM: usize
    >(&mut self, colors: [f64; COLOR_NUM]) {
        let colors = colors.iter().map(|e|e.to_string()).collect::<Vec<String>>().join(" ");
        self.operators.push(format!("{colors} scn"));
    }

    pub fn set_stroke_color(&mut self, r: f32, g: f32, b: f32) {
        self.operators.push(format!("{r} {g} {b} RG"));
    }

    pub fn set_fill_color(&mut self, r: f32, g: f32, b: f32) {
        self.operators.push(format!("{r} {g} {b} rg"));
    }

    pub fn set_fill_cmyk_color(&mut self, c: f32, m: f32, y: f32, k:f32) {
        self.operators.push(format!("{c} {m} {y} {k} k"));
    }

    pub fn fill_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        self.operators.push(format!("{x} {y} {width} {height} re f"));
    }

    pub fn fill_text(&mut self, font_name: &str, font_size: u32, pos: Pos, text: &str) {
        self.operators.push(format!(concat!(
            "BT\n",
            "  /{} {} Tf\n",
            "  {} {} Td\n",
            "  0 Tr\n",
            "  ({}) Tj\n",
            "ET"),
            font_name, font_size,
            pos.x, pos.y,
            text));
    }

    fn get_stream_string(&self, indent_size: usize) -> String {
        indent(&self.operators.join("\n"), indent_size)
    }

    pub fn to_string(&self, indent_size: usize) -> String {
        let stream = self.get_stream_string(indent_size);

        indent(
            &format!(
                concat!(
                    "{} obj\n",
                    "<<\n",
                    "  /Length {}\n",
                    ">>\n",
                    "stream\n",
                    "{}\n",
                    "endstream\n",
                    "endobj"
                ),
                self.id.to_string(),
                stream.len(),
                stream
            ),
            indent_size,
        )
    }
}

impl PdfObject for Contents {
    fn id(&self) -> &Id {
        &self.id
    }

    fn assign_ids(&mut self, id_factory: &mut IdFactory) {
        self.id = id_factory.next_id()
    }

    fn get_objects(&self) -> Vec<&dyn PdfObject> {
        vec![self]
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
    fn set_stroke_color() {
        let mut c = Contents::new();
        c.set_stroke_color(0.1, 0.2, 0.3);

        let ok = concat!(
            "0 0 obj\n",
            "<<\n",
            "  /Length 14\n",
            ">>\n",
            "stream\n",
            "0.1 0.2 0.3 RG\n",
            "endstream\n",
            "endobj",
        );

        assert_eq!(c.to_string(0), ok);
    }
}
