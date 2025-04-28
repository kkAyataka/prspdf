// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

pub struct Pos {
    pub x: u32,
    pub y: u32,
}

impl Pos {
    pub fn new(x: u32, y: u32) -> Pos {
        Pos { x, y }
    }
}

#[derive(Eq, Hash, PartialEq)]
pub struct Name {
    name: String,
}

impl Name {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "/{}", self.name)
    }
}

//------------------------------------------------------------------------------
// Version
//------------------------------------------------------------------------------

/// PDF versions.
#[derive(Clone, Copy)]
pub enum Version {
    // V1_0 = 0,
    // V1_1 = 1,
    // V1_2 = 2,
    // V1_3 = 3,
    V1_4 = 4,
    // V1_5 = 5,
    // V1_6 = 6,
    V1_7 = 7,
}

impl Version {
    /// Returns the PDF version string in the header.
    pub fn to_str(&self) -> &str {
        match self {
            Self::V1_4 => "%PDF-1.4",
            Self::V1_7 => "%PDF-1.7",
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_str().to_string().into_bytes()
    }
}

//------------------------------------------------------------------------------
// MediaBox
//------------------------------------------------------------------------------

/// MediaBox types.
pub enum MediaBox {
    /// 0, 0, 612, 792
    Letter,
    /// 0, 0, 595, 842
    A4,
    Custom(u32, u32, u32, u32),
}

impl MediaBox {
    /// Returns the string in PDF.
    pub fn to_string(&self) -> String {
        match self {
            Self::Letter => "[0 0 612 792]".to_string(), // pt = 1 / 72 inch
            Self::A4 => "[0 0 595 842]".to_string(),
            Self::Custom(v1, v2,v3, v4 ) => format!("[{v1} {v2} {v3} {v4}]"),
        }
    }
}

//------------------------------------------------------------------------------
// Id
//------------------------------------------------------------------------------

#[derive(Copy, Clone)]
pub struct Id {
    id: u32,
    generation: u32,
}

impl Id {
    /// Creates a new Id.
    pub fn new(id: u32, generation: u32) -> Id {
        Id { id: id, generation }
    }

    /// Creates a new 0 Id.
    pub fn new_0() -> Id {
        Id { id: 0, generation: 0 }
    }

    /// Converts to a simple string
    pub fn to_string(&self) -> String {
        format!("{} {}", self.id, self.generation)
    }

    /// Converts to a reference string
    pub fn to_ref_string(&self) -> String {
        format!("{} R", self.to_string())
    }
}

//------------------------------------------------------------------------------
// IdFactory
//------------------------------------------------------------------------------

pub struct IdFactory {
    page_list_id: Id,
    next_id: u32,
}

impl IdFactory {
    pub fn new() -> Self {
        IdFactory {
            page_list_id: Id::new(1, 0),
            next_id: 2
        }
    }

    pub fn page_list_id(&self) -> &Id {
        &self.page_list_id
    }

    /// Creates new Id and increments a next id number from the initial_id.
    pub fn next_id(&mut self) -> Id {
        let id = Id::new(self.next_id, 0);
        self.next_id += 1;
        id
    }
}

pub trait PdfObject {
    fn id(&self) -> &Id;
    fn assign_ids(&mut self, id_factory: &mut IdFactory);
    fn get_objects(&self) -> Vec<&dyn PdfObject>;
    fn to_bytes(&self, indent_depth: usize) -> Vec<u8>;
}

//------------------------------------------------------------------------------
// tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    mod pos {
        use super::super::*;

        #[test]
        fn new() {
            let p = Pos::new(1, 2);
            assert_eq!(p.x, 1);
            assert_eq!(p.y, 2);
        }
    }

    mod version {
        use super::super::*;

        #[test]
        fn to_string() {
            assert_eq!(Version::V1_4.to_str(), "%PDF-1.4");
            assert_eq!(Version::V1_7.to_str(), "%PDF-1.7");
        }
    }

    mod media_box {
        use super::super::*;

        #[test]
        fn new_custom() {
            let mb = MediaBox::Custom(1, 2, 3, 4);
            assert_eq!(mb.to_string(), "[1 2 3 4]");
        }
    }

    mod id {
        use super::super::*;

        #[test]
        fn new() {
            let id = Id::new(112, 2);
            assert_eq!(id.to_string(), "112 2");
            assert_eq!(id.to_ref_string(), "112 2 R");
        }

        #[test]
        fn new_0() {
            let id = Id::new_0();
            assert_eq!(id.to_string(), "0 0");
            assert_eq!(id.to_ref_string(), "0 0 R");
        }
    }

    mod id_factory {
        use super::super::*;

        #[test]
        fn new_id() {

            let mut id_factory = IdFactory::new();
            assert_eq!(id_factory.next_id().to_string(), "2 0");
            assert_eq!(id_factory.next_id().to_string(), "3 0");
            assert_eq!(id_factory.next_id().to_string(), "4 0");
        }
    }
}
