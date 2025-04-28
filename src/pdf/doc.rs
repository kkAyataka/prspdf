// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)


use std::fs;

use super::base::*;
use super::page::Page;
use super::page_list::*;

/// PDF document
pub struct Doc {
    ver: Version,
    is_binary: bool,
    page_list: PageList,
}

impl Doc {
    /// Creates new PDF document
    pub fn new(ver: Version) -> Doc {
        Doc {
            ver,
            is_binary: true,
            page_list: PageList::new(),
        }
    }

    /// Adds the page to the back.
    pub fn push_page(&mut self, page: Page) {
        self.page_list.push(page);
    }

    fn get_header_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        bytes.append(&mut self.ver.to_bytes());
        if self.is_binary {
            bytes.push("\n".as_bytes()[0]);
            bytes.push("%".as_bytes()[0]);
            bytes.push(0x80);
            bytes.push(0x80);
            bytes.push(0x80);
            bytes.push(0x80);
        }

        bytes
    }

    fn get_doc_catalog_bytes(&self, id: &Id, page_list_id: &Id) -> Vec<u8> {
        format!(concat!(
            "{} obj\n",
            "<< /Type /Catalog\n",
            "   /Pages {}\n",
            ">>\n",
            "endobj"),
            id.to_string(),
            page_list_id.to_ref_string()
        ).into_bytes()
    }

    fn get_cross_ref_table_bytes(&self, byte_offsets: &Vec<usize>) -> Vec<u8> {
        let mut s = String::new();
        s.push_str(&format!("0 {}\n", byte_offsets.len() + 1));
        s.push_str("0000000000 65535 f \n");

        for index in byte_offsets {
            s.push_str(&format!("{:0>10} 00000 n \n", index));
        }

        s.into_bytes()
    }

    fn get_trailer_bytes(&self, doc_catalog_id: &Id, object_count: usize) -> Vec<u8> {
        format!(concat!(
            "trailer\n",
            "<< /Root {}\n",
            "   /Size {}\n",
            ">>\n"),
            doc_catalog_id.to_ref_string(),
            object_count).into_bytes()
    }

    pub fn to_bytes(&mut self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        let mut id_factory = IdFactory::new();
        let mut byte_offsets: Vec<usize> = Vec::new();

        //
        self.page_list.assign_ids(&mut id_factory);

        // Header
        bytes.append(&mut self.get_header_bytes());

        // Page list, Page
        let objects = self.page_list.get_objects();
        for obj in &objects {
            bytes.append(&mut "\n".to_string().into_bytes());
            byte_offsets.push(bytes.len());
            bytes.append(&mut obj.to_bytes(0));
        }

        // Document catalog
        bytes.append(&mut "\n".to_string().into_bytes());
        byte_offsets.push(bytes.len());
        let doc_catalog_id = id_factory.next_id();
        let b = self.get_doc_catalog_bytes(&doc_catalog_id, &self.page_list.id);
        bytes.append(&mut b.clone());

        // Cross-reference table
        bytes.append(&mut "\n".to_string().into_bytes());
        let cross_ref_offset = bytes.len();
        bytes.append(&mut "xref\n".to_string().into_bytes());
        bytes.append(&mut self.get_cross_ref_table_bytes(&byte_offsets));

        // Trailer
        bytes.append(&mut self.get_trailer_bytes(&doc_catalog_id, byte_offsets.len() + 1));

        bytes.append(&mut "startxref\n".to_string().into_bytes());
        bytes.append(&mut format!("{}\n", cross_ref_offset).into_bytes());

        // EOF
        bytes.append(&mut "%%EOF".to_string().into_bytes());
        bytes.append(&mut "\n".to_string().into_bytes());

        bytes
    }

    /// Write out the specified path.
    pub fn write_to_file(&mut self, path: String) {
        fs::write(path, self.to_bytes()).unwrap();
    }
}


mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // let mut doc = Doc::new(Version::V1_4);
        // let mut page = Page::new(MediaBox::Letter);
        // page.resources().add_font("F0", Font::new("Times-Italic"));
        // page.contents().fill_text("F0", 32, Pos::new(0, 0), "Hello");
        // doc.push_page(page);

        // let exe_path = std::env::current_exe().unwrap();
        // let dir = exe_path.parent().unwrap();
        // let path = dir.join("hello.pdf");
        // println!("{}", path.to_str().unwrap());
        // doc.write_to_file(path.to_str().unwrap().to_string());
        // //assert_eq!(String::from_utf8(doc.to_bytes()).unwrap(), "".to_string());
    }
}
