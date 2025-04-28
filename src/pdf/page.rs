// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)


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

    pub fn reassign_ids(&mut self, id_factory: &mut IdFactory) {
        self.id = id_factory.next_id();
        self.parent_id = id_factory.page_list_id().clone();
        self.resources.assign_ids(id_factory);
        self.contents.id = id_factory.next_id();
    }

    fn get_contents_string(&self) -> String {
        format!("{}", self.contents.id.to_ref_string())
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

impl PdfObject for Page {
    fn id(&self) -> &Id {
        &self.id
    }

    fn assign_ids(&mut self, id_factory: &mut IdFactory) {
        self.reassign_ids(id_factory);
    }

    fn get_objects(&self) -> Vec<&dyn PdfObject> {
        let mut list: Vec<&dyn PdfObject> = Vec::new();

        list.push(self);
        list.append(&mut self.resources.get_objects());
        list.append(&mut self.contents.get_objects());

        list
    }

    fn to_bytes(&self, indent_depth: usize) -> Vec<u8> {
        self.to_string(indent_depth).into_bytes()
    }
}
