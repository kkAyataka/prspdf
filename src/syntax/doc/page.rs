// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use super::super::objects::*;
use super::super::content_and_resouce::*;

pub struct Page {
    id: Id,
    parent_id: Id, // PageList id
    media_box: MediaBox,
    resources: Resources,
    content: Content,
}

impl Page {
    pub fn new(media_box: MediaBox) -> Self {
        Page {
            id: Id::new_0(),
            parent_id: Id::new_0(),
            media_box: media_box,
            resources: Resources::new(),
            content: Content::new(),
        }
    }

    pub fn resources(&mut self) -> &mut Resources {
        &mut self.resources
    }

    pub fn content(&mut self) -> &mut Content {
        &mut self.content
    }

    pub fn contents(&mut self) -> &mut Content {
        &mut self.content
    }

    fn to_pdf_obj_string(&self) -> String {
        format!(
            concat!(
                "{} obj\n",
                "<<\n",
                "  /Type /Page\n",
                "  /MediaBox {}\n",
                "  /Parent {}\n",
                "  /Resources {}\n",
                "  /Contents {}\n",
                ">>\n",
                "endobj"
            ),
            self.id.to_string(),
            self.media_box.to_string(),
            self.parent_id.to_ref_string(),
            self.resources.id().to_ref_string(),
            self.content.id().to_ref_string()
        )
    }
}

impl PdfObject for Page {
    fn id(&self) -> &Id {
        &self.id
    }

    fn assign_ids(&mut self, id_factory: &mut IdFactory) {
        self.id = id_factory.next_id();
        self.parent_id = id_factory.page_list_id().clone();
        self.resources.assign_ids(id_factory);
        self.content.assign_ids(id_factory);
    }

    fn get_objects(&self) -> Vec<&dyn PdfObject> {
        let mut list: Vec<&dyn PdfObject> = Vec::new();

        list.push(self);
        list.append(&mut self.resources.get_objects());
        list.append(&mut self.content.get_objects());

        list
    }

    fn to_bytes(&self, _indent_depth: usize) -> Vec<u8> {
        self.to_pdf_obj_string().into_bytes()
    }
}
