// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

//! Small PDF maker
//!
//! # Quick Start
//!
//! ```rust
//! // Creates a new document
//! let mut doc = prspdf::Doc::new();
//!
//! // Creates a new page ans push it
//! let page = doc.push_page(prspdf::Page::new(prspdf::MediaBox::Letter));
//!
//! // Registers font
//! page.resources().add_font("F0", prspdf::fonts::Font::new_type1("Alial"));
//!
//! // Add "Hello" text
//! page.contents().fill_text("F0", 32, prspdf::Pos::new(0, 0), "Hello");
//!
//! // Write to file
//! let exe_path = std::env::current_exe().unwrap();
//! let dir = exe_path.parent().unwrap();
//! let path = dir.join("hello.pdf");
//! doc.write_to_file(path.to_str().unwrap().to_string());
//! ```

mod graphics;
pub use graphics::color_spaces;
pub use graphics::color_spaces::ColorSpace;

mod syntax;
pub use syntax::MediaBox;
pub use syntax::Version;
pub use syntax::Pos;
pub use syntax::doc::Doc;
pub use syntax::doc::PageList;
pub use syntax::doc::Page;
pub use syntax::functions;

mod text;
pub use text::fonts;

// for private implementation
mod utils;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::graphics::color_spaces;

    use super::*;

    #[test]
    fn hello() {
        let mut doc = crate::Doc::new();

        let mut page = Page::new(MediaBox::Letter);
        let font = fonts::Font::new_type1("Arial");
        page.resources().add_font("F0", font);
        page.contents().fill_text("F0", 32, Pos {x: 0, y: 760}, "Hello");

        doc.push_page(page);

        let exe_path = std::env::current_exe().unwrap();
        let dir = exe_path.parent().unwrap();
        let path = dir.join("hello.pdf");
        doc.write_to_file(path.to_str().unwrap().to_string());
    }

    #[test]
    fn cmykogv() {
        let mut doc = Doc::new();
        let mut page = Page::new(MediaBox::Letter);

        let font = fonts::Font::new_type1("Times-Italic");
        page.resources().add_font("F0", font);

        // CMYKOGV color space
        let in_domain = [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0)];
        let out_range = [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0)];
        let sample_sizes = [1, 1, 1, 1, 1, 1, 1];
        let bit_per_sample = 8;
        let samples = vec![64, 64, 255, 0];
        let type0 = functions::Type0::new(in_domain, out_range, sample_sizes, bit_per_sample, samples);
        let process_component_names: Vec<&str> = vec!["Cyan", "Magenta", "Yellow", "Black"];
        let orange = color_spaces::Separation::new(
            "Orange",
            color_spaces::Lab::new_with_white(0.964203, 1.0, 0.824905),
            functions::Type2::new([100.0, 0.0, 0.0], [65.0, 58.0, 88.0], 1.0));
        let green = color_spaces::Separation::new(
            "Green",
            color_spaces::Lab::new_with_white(0.964203, 1.0, 0.824905),
            functions::Type2::new([100.0, 0.0, 0.0], [60.0, -75.0, 0.0], 1.0));
        let violet = color_spaces::Separation::new(
            "Violet",
            color_spaces::Lab::new_with_white(0.964203, 1.0, 0.824905),
            functions::Type2::new([100.0, 0.0, 0.0], [22.0, 47.0, -56.0], 1.0));
        let mut colorants: HashMap<&str, color_spaces::Separation> = HashMap::new();
        colorants.insert("Orange", orange);
        colorants.insert("Green", green);
        colorants.insert("Violet", violet);
        let nchannel = color_spaces::NChannel::new(colorants, process_component_names);
        let names = vec!["Cyan", "Magenta", "Yellow", "Black", "Orange", "Green", "Violet"];
        let devicen = color_spaces::DeviceN::new(names, type0, nchannel);
        page.resources().add_color_space("CS0", ColorSpace::DeviceN(devicen));

        // Contents
        page.contents().fill_text("F0", 32, Pos {x: 0, y: 760}, "Hello");
        page.contents().set_fill_color_space("CS0");
        page.contents().set_fill_color_space_color([1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        page.contents().fill_rect(0, 700, 50, 50);
        page.contents().set_fill_color_space_color([0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0]);
        page.contents().fill_rect(50, 700, 50, 50);
        page.contents().set_fill_color_space_color([0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0]);
        page.contents().fill_rect(100, 700, 50, 50);
        page.contents().set_fill_color_space_color([0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0]);
        page.contents().fill_rect(150, 700, 50, 50);
        page.contents().set_fill_color_space_color([0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0]);
        page.contents().fill_rect(200, 700, 50, 50);
        page.contents().set_fill_color_space_color([0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0]);
        page.contents().fill_rect(250, 700, 50, 50);
        page.contents().set_fill_color_space_color([0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0]);
        page.contents().fill_rect(300, 700, 50, 50);

        doc.push_page(page);

        let exe_path = std::env::current_exe().unwrap();
        let dir = exe_path.parent().unwrap();
        let path = dir.join("cmykogv.pdf");
        doc.write_to_file(path.to_str().unwrap().to_string());
    }
}
