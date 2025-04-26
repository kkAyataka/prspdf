// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use super::base::*;
use super::page::Page;
use super::utils::indent;
use super::PdfObject;

pub struct PageList {
    pub id: Id,
    pages: Vec<Page>,
}

impl PageList {
    pub fn new() -> PageList {
        PageList {
            id: Id::new_0(),
            pages: Vec::new(),
        }
    }

    pub fn push(&mut self, page: Page) {
        self.pages.push(page);
    }

    pub fn get_object_list(&mut self, id_factory: &mut IdFactory) -> Vec<PdfObject> {
        let mut list: Vec<PdfObject> = Vec::new();

        self.reassign_ids(id_factory);

        // Page list
        list.push(PdfObject::PageList(self));

        // Pages
        for page in &self.pages {
            list.append(&mut page.get_object_list());
        }

        list
    }

    fn reassign_ids(&mut self, id_factory: &mut IdFactory) {
        self.id = id_factory.new_id();
        for page in &mut self.pages {
            page.reassign_ids(self.id, id_factory);
        }
    }

    fn get_kids_string(&self) -> String {
        let mut kids = String::new();
        kids.push_str("[");
        for page in &self.pages {
            kids.push_str(&format!("{} ", page.id.to_ref_string()))
        }
        kids.push_str("]");
        kids
    }

    pub fn to_string(&self, indent_size: usize) -> String {
        indent(&format!(concat!(
            "{} obj\n",
            "<< /Type /Pages\n",
            "   /Count {}\n",
            "   /Kids {}\n",
            ">>\n",
            "endobj"),
            self.id.to_string(),
            self.pages.len(),
            self.get_kids_string()),
            indent_size)
    }
}
