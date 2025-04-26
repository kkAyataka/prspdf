// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use super::PdfObject;
use super::base::*;
use super::contents::Contents;
use super::resources::Resources;
use super::utils::*;

pub struct Page {
    pub id: Id,
    parent_id: Id, // PageList id
    media_box: MediaBox,
    resources: Resources,
    contents: Contents,
}

impl Page {
    pub fn new(media_box: MediaBox) -> Self {
        Page {
            id: Id::new_0(),
            parent_id: Id::new_0(),
            media_box: media_box,
            resources: Resources::new(),
            contents: Contents::new(),
        }
    }

    pub fn resources(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn contents(&mut self) -> &mut Contents {
        &mut self.contents
    }

    pub(crate) fn get_object_list(&self) -> Vec<PdfObject> {
        let mut list: Vec<PdfObject> = Vec::new();

        list.push(PdfObject::Page(self));
        list.push(PdfObject::Resources(&self.resources));
        list.push(PdfObject::Contents(&self.contents));

        list
    }

    pub fn reassign_ids(&mut self, page_list_id: Id, id_factory: &mut IdFactory) {
        self.id = id_factory.new_id();
        self.parent_id = page_list_id;
        self.resources.id = id_factory.new_id();
        self.contents.id = id_factory.new_id();
    }

    fn get_contents_string(&self) -> String {
        format!("[ {} ]", self.contents.id.to_ref_string())
    }

    pub fn to_string(&self, indent_size: usize) -> String {
        indent(&format!(concat!(
            "{} obj\n",
            "<< /Type /Page\n",
            "   /MediaBox {}\n",
            "   /Resources {}\n",
            "   /Parent {}\n",
            "   /Contents {}\n",
            ">>\n",
            "endobj"),
            self.id.to_string(),
            self.media_box.to_string(),
            self.resources.id.to_ref_string(),
            self.parent_id.to_ref_string(),
            self.get_contents_string()),
            indent_size)
    }
}

// impl PdfObject for Page {
//     fn to_bytes(&self, indent_size: usize) -> Vec<u8> {
//         self.to_string(indent_size).into_bytes()
//     }
// }
