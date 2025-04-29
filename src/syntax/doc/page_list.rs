// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use super::super::objects::base::*;
use super::page::Page;

pub struct PageList {
    id: Id,
    pages: Vec<Page>,
}

impl PageList {
    pub fn new() -> PageList {
        PageList {
            id: Id::new_0(),
            pages: Vec::new(),
        }
    }

    /// Appends the specified page to the back of a pages
    pub fn push(&mut self, page: Page) -> &mut Page {
        let last_index = self.pages.len();
        self.pages.push(page);
        &mut self.pages[last_index]
    }

    fn to_pdf_obj_string(&self) -> String {
        format!(
            concat!(
                "{} obj\n",
                "<<\n",
                "  /Type /Pages\n",
                "  /Count {}\n",
                "  /Kids {}\n",
                ">>\n",
                "endobj"
            ),
            self.id.to_string(),
            self.pages.len(),
            self.get_kids_string(),
        )
    }

    fn get_kids_string(&self) -> String {
        let mut kids = String::new();
        kids.push_str("[");
        let mut is_first = true;
        for page in &self.pages {
            if !is_first {
                kids.push(' ');
            }
            kids.push_str(&page.id().to_ref_string());
            is_first = false;
        }
        kids.push_str("]");
        kids
    }
}

impl PdfObject for PageList {
    fn id(&self) -> &Id {
        &self.id
    }

    fn assign_ids(&mut self, id_factory: &mut IdFactory) {
        self.id = id_factory.page_list_id().clone();
        for page in &mut self.pages {
            page.assign_ids(id_factory);
        }
    }

    fn get_objects(&self) -> Vec<&dyn PdfObject> {
        let mut list: Vec<&dyn PdfObject> = Vec::new();

        // Self
        list.push(self);

        // Pages
        for page in &self.pages {
            list.append(&mut page.get_objects());
        }

        list
    }

    fn to_bytes(&self, _indent_depth: usize) -> Vec<u8> {
        self.to_pdf_obj_string().into_bytes()
    }
}
