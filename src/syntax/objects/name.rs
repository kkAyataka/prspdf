// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::utils::ToPdfString;

#[derive(Eq, Hash, PartialEq)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string()
        }
    }
}

impl ToPdfString<Name> for Name {
    fn to_pdf_string(&self) -> String {
        format!("/{}", self.name)
    }
}
