// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)


use super::base::*;
use super::page::Page;
use super::utils::indent;

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

    fn reassign_ids(&mut self, id_factory: &mut IdFactory) {
        self.id = id_factory.page_list_id().clone();
        for page in &mut self.pages {
            page.assign_ids(id_factory);
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

impl PdfObject for PageList {
    fn id(&self) -> &Id {
        &self.id
    }

    fn assign_ids(&mut self, id_factory: &mut IdFactory) {
        self.reassign_ids(id_factory);
    }

    fn get_objects(&self) -> Vec<&dyn PdfObject> {
        let mut list: Vec<&dyn PdfObject> = Vec::new();

        list.push(self);

        // Pages
        for page in &self.pages {
            list.append(&mut page.get_objects());
        }

        list
    }

    fn to_bytes(&self, indent_depth: usize) -> Vec<u8> {
        self.to_string(indent_depth).into_bytes()
    }
}
