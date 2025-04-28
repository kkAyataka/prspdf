// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use std::collections::HashMap;

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

impl<T: ToPdfString<T>> ToPdfString<HashMap<Name, T>> for HashMap<Name, T> {
    fn to_pdf_string(&self) -> String {
        let mut s = String::new();
        s.push_str("<<\n");
        for e in self {
            s.push_str(&format!("  {} {}\n", e.0.to_pdf_string(), e.1.to_pdf_string()));
        }
        s.push_str(">>");
        s
    }
}
