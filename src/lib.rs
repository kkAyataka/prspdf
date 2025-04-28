// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)


mod pdf;
pub use pdf::Version;
pub use pdf::MediaBox;
pub use pdf::doc::Doc;
pub use pdf::page::Page;
pub use pdf::font::Font;
pub use pdf::Pos;
pub use pdf::colour;
pub use pdf::function;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, hash::Hash};

    use super::*;

    #[test]
    fn hello() {
        let mut doc = Doc::new(Version::V1_7);

        let mut page = Page::new(MediaBox::Letter);
        let font = Font::new("Arial");
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
        let mut doc = Doc::new(Version::V1_7);
        let mut page = Page::new(MediaBox::Letter);

        let font = Font::new("Times-Italic");
        page.resources().add_font("F0", font);

        // CMYKOGV color space
        let names: Vec<&str> = vec!["Cyan", "Magenta", "Yellow", "Black"];
        let in_domain = [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0)];
        let out_range = [(0.0, 1.0), (0.0, 1.0), (0.0, 1.0), (0.0, 1.0)];
        let sample_sizes = [1, 1, 1, 1, 1, 1, 1];
        let bit_per_sample = 8;
        let samples = vec![64, 64, 255, 0];
        let type0 = function::Type0::new(in_domain, out_range, sample_sizes, bit_per_sample, samples);
        let process_component_names: Vec<&str> = vec!["Cyan", "Magenta", "Yellow", "Black"];
        let orange = colour::space::Separation::new(
            "Orange",
            colour::space::Lab::new_with_white(0.964203, 1.0, 0.824905),
            function::Type2::new([100.0, 0.0, 0.0], [65.0, 58.0, 88.0], 1.0));
        let green = colour::space::Separation::new(
            "Green",
            colour::space::Lab::new_with_white(0.964203, 1.0, 0.824905),
            function::Type2::new([100.0, 0.0, 0.0], [60.0, -75.0, 0.0], 1.0));
        let violet = colour::space::Separation::new(
            "Violet",
            colour::space::Lab::new_with_white(0.964203, 1.0, 0.824905),
            function::Type2::new([100.0, 0.0, 0.0], [22.0, 47.0, -56.0], 1.0));
        let mut colorants: HashMap<&str, colour::space::Separation> = HashMap::new();
        colorants.insert("Orange", orange);
        colorants.insert("Green", green);
        colorants.insert("Violet", violet);
        let nchannel = colour::space::NChannel::new(colorants, process_component_names);
        let names = vec!["Cyan", "Magenta", "Yellow", "Black", "Orange", "Green", "Violet"];
        let devicen = colour::space::DeviceN::new(names, type0, nchannel);
        page.resources().add_color_space("CS0", devicen);

        // Contents
        page.contents().fill_text("F0", 32, Pos {x: 0, y: 760}, "Hello");
        page.contents().set_fill_color_space("/CS0");
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
