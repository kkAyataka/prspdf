// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::utils::*;

/// PDF32000-1:2008 8.6.5.4
///
/// ```text
/// [
///   /Lab
///   <<
///     /WhitePoint [0.964203 1.0 0.824905]
///     /BlackPoint [0.0 0.0 0.0]
///     /Range [-100 100 -100 100]
///   >>
/// ]
/// ```
#[derive(Clone)]
pub struct Lab {
    white: [f64; 3],
    black: [f64; 3],
    range: [f64; 4],
}

impl Lab {
    pub fn new(white: [f64; 3], black: [f64; 3], range: [f64; 4]) -> Self {
        Self {
            white,
            black,
            range,
        }
    }

    pub fn new_with_white(cie_x: f64, cie_y: f64, cie_z: f64) -> Self {
        Self::new(
            [cie_x, cie_y, cie_z],
            [0.0, 0.0, 0.0],
            [-128.0, 127.0, -128.0, 127.0],
        )
    }

    pub fn to_pdf_string(&self, indent_depth: usize) -> String {
        indent(
            &format!(
                concat!(
                    "[\n",
                    "  /Lab\n",
                    "  <<\n",
                    "    /WhitePoint {}\n",
                    "    /BlackPoint {}\n",
                    "    /Range {}\n",
                    "  >>\n",
                    "]"
                ),
                self.white.to_pdf_string(),
                self.black.to_pdf_string(),
                self.range.to_pdf_string(),
            ),
            indent_depth,
        )
    }

    pub fn to_bytes(&self, indent_depth: usize) -> Vec<u8> {
        self.to_pdf_string(indent_depth).clone().into_bytes()
    }
}

//------------------------------------------------------------------------------
// tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_bytes() {
        let lab = Lab {
            white: [0.964203, 1.0, 0.824905],
            black: [0.0, 0.0, 0.0],
            range: [-100.0, 100.0, -100.0, 100.0],
        };

        let expected = concat!(
            "[\n",
            "  /Lab\n",
            "  <<\n",
            "    /WhitePoint [0.964203 1.0 0.824905]\n",
            "    /BlackPoint [0.0 0.0 0.0]\n",
            "    /Range [-100.0 100.0 -100.0 100.0]\n",
            "  >>\n",
            "]"
        );

        assert_eq!(String::from_utf8(lab.to_bytes(0)).unwrap(), expected);
    }

    #[test]
    fn test_to_bytes_2() {
        let lab = Lab {
            white: [0.964203, 1.0, 0.824905],
            black: [0.1, 0.2, 0.3],
            range: [-100.1, 100.2, -100.3, 100.4],
        };

        let expected = concat!(
            "  [\n",
            "    /Lab\n",
            "    <<\n",
            "      /WhitePoint [0.964203 1.0 0.824905]\n",
            "      /BlackPoint [0.1 0.2 0.3]\n",
            "      /Range [-100.1 100.2 -100.3 100.4]\n",
            "    >>\n",
            "  ]"
        );

        assert_eq!(String::from_utf8(lab.to_bytes(1)).unwrap(), expected);
    }
}
