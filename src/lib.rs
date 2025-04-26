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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let doc = Doc::new(Version::V1_4);
        let page = Page::new(MediaBox::Letter);
        page.get_object_list();
    }
}
