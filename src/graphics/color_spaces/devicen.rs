// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use std::collections::HashMap;

use super::{DeviceCMYK, Separation};
use crate::syntax::*;

use crate::utils::*;

/// PDF32000-1:2008 8.6.6.5
///
/// 10 0 obj
/// <<
///   /FunctionType 0
///   /Domain [0.0 1.0 0.0 1.0 0.0 1.0 0.0 1.0 0.0 1.0 0.0 1.0 0.0 1.0]
///   /Range [0.0 1.0 0.0 1.0 0.0 1.0 0.0 1.0]
///   /Sizs [1 1 1 1 1 1 1]
///   /BitsPerSample 8
///   /Length 4
/// >>
/// stream
/// endstream
/// end obj
/// 11 0 obj
/// [
///   /DeviceN
/// %names
///   [/Cyan /Magenta /Yellow /Black /Orange /Green /Violet]
/// %alternateSpace
///   /DeviceCMYK
/// %tintTransform
///   10 0 R
/// %attributes (NChannel)
///   <<
///     /Subtype /NChannel
///     /Colorants <<
///       /Orange [
///         /Separation
///         /Orange
///         [
///           /Lab
///           <<
///             /WhitePoint [0.964203 1.0 0.824905]
///             /BlackPoint [0.0 0.0 0.0]
///             /Range [-100 100 -100 100]
///           >>
///         ]
///         <<
///           /FunctionType 2
///           /C0 [100.0 0.0 0.0]
///           /C1 [65.0 58.0 88.0]
///           /N 1.0
///         >>
///       ]
///       /Green
///       ...
///       /Violet
///       ...
///     >>
///     /Process <<
///       /ColorSpace /DeviceCMYK
///       /Components [/Cyan /Magenta /Yellow /Black]
///     >>
///   >>
/// ]
/// endobj
pub struct DeviceN {
    id: Id,
    names: Vec<Name>,
    alt_space: DeviceCMYK,
    tint_transform: functions::Type0,
    attributes: NChannel,
}

impl DeviceN {
    pub fn new(names: Vec<&str>, tint_transform: functions::Type0, attributes: NChannel) -> Self {
        Self {
            id: Id::new_0(),
            names: names.iter().map(|e| Name::new(e)).collect(),
            alt_space: DeviceCMYK {},
            tint_transform,
            attributes
        }
    }

    pub fn to_pdf_string(&self, indent_depth: usize) -> String {
        indent(&format!(concat!(
            "{} obj\n",
            "[\n",
            "  /DeviceN\n",
            "  {}\n", // names
            "  {}\n", // alternateSpace
            "  {}\n", // tintTransform
            "  {}\n", // attributes
            "]\n",
            "endobj"),
            self.id.to_string(),
            self.names.to_pdf_string(),
            self.alt_space.to_pdf_string(),
            self.tint_transform.id().to_ref_string(),
            self.attributes.to_pdf_string(indent_depth + 1)
        ), indent_depth)
    }
}

impl PdfObject for DeviceN {
    fn id(&self) -> &Id {
        &self.id
    }

    fn assign_ids(&mut self, id_factory: &mut IdFactory) {
        self.tint_transform.assign_ids(id_factory);
        self.id = id_factory.next_id()
    }

    fn get_objects(&self) -> Vec<&dyn PdfObject> {
        vec![&self.tint_transform, self]
    }

    fn to_bytes(&self, indent_depth: usize) -> Vec<u8> {
        self.to_pdf_string(indent_depth).into_bytes()
    }
}

pub struct NChannel {
    colorants: HashMap<Name, Separation>,
    process_space: DeviceCMYK,
    process_component_names: Vec<Name>,
}

impl NChannel {
    pub fn new(colorants: HashMap<&str, Separation>, process_component_names: Vec<&str>) -> Self {
        let mut m: HashMap<Name, Separation> = HashMap::new();
        for c in colorants {
            m.insert(Name::new(c.0), c.1);
        }

        NChannel {
            colorants: m,
            process_space: DeviceCMYK{},
            process_component_names: process_component_names.iter().map(|e| Name::new(e)).collect(),
        }
    }

    fn get_colorants_pdf_string(&self) -> String {
        let mut s = String::new();
        s.push_str("<<\n");
        for c in &self.colorants {
            s.push_str(&format!(concat!(
                "  {}\n",
                "{}\n"
            ), c.0.to_pdf_string(), c.1.to_pdf_string(2)
        ));
        }
        s.push_str(">>");
        s
    }

    pub fn to_pdf_string(&self, indent_depth: usize) -> String {
        indent(&format!(concat!(
            "<<\n",
            "  /Subtype /NChannel\n",
            "  /Colorants\n",
            "  {}\n",
            "  /Process\n",
            "    <<\n",
            "      /ColorSpace {}\n",
            "      /Components {}\n",
            "    >>\n",
            "\n",
            ">>"),
            self.get_colorants_pdf_string(),
            self.process_space.to_pdf_string(),
            self.process_component_names.to_pdf_string(),
        ), indent_depth)
    }
}
