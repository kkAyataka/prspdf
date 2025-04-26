mod base;
pub use base::*;
pub mod contents;
pub mod doc;
pub mod font;
pub mod page;
pub mod page_list;
pub mod resources;
pub mod utils;

pub enum PdfObject<'a> {
    PageList(&'a page_list::PageList),
    Page(&'a page::Page),
    Resources(&'a resources::Resources),
    Contents(&'a contents::Contents),
    Font(&'a font::Font),
}

impl PdfObject<'_> {
    fn to_bytes(&self, indent_size: usize) -> Vec<u8> {
        match self {
            Self::PageList(v) => v.to_string(indent_size).into_bytes(),
            Self::Page(v) => v.to_string(indent_size).into_bytes(),
            Self::Resources(v) => v.to_string(indent_size).into_bytes(),
            Self::Contents(v) => v.to_string(indent_size).into_bytes(),
            Self::Font(v) => v.to_string(indent_size).into_bytes(),
        }
    }
}
