// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::pdf::utils::ToPdfString;

use super::super::utils::indent;

/// PDF32000-1:2008 7.10.3
///
/// ```text
/// <<
///   /FunctionType 2
///   /C0 [100.0 0.0 0.0]
///   /C1 [65.0 58.0 88.0]
///   /N 1.0
/// >>
/// ```
pub struct Type2 {
    c0: Vec<f64>,
    c1: Vec<f64>,
    n: f64,
}

impl Type2 {
    pub fn new<
        const OUTPUT_NUM: usize,
    >(
        c0: [f64; OUTPUT_NUM],
        c1: [f64; OUTPUT_NUM],
        n: f64,
    ) -> Type2 {
        Type2 {
            c0: c0.to_vec(),
            c1: c1.to_vec(),
            n,
        }
    }

    pub fn to_bytes(&self, indent_size: usize) -> Vec<u8> {
        indent(&format!(concat!(
            "<<\n",
            "  /FunctionType 2\n",
            "  /Domain [0.0 1.0]\n",
            "  /Range [0.0 100.0 -128.0 127.0 -128.0 127.0]\n",
            "  /C0 {}\n",
            "  /C1 {}\n",
            "  /N {:.1}\n",
            ">>"),
            self.c0.to_pdf_string(),
            self.c1.to_pdf_string(),
            self.n,
        ), indent_size).into_bytes()
    }
}


//------------------------------------------------------------------------------
// tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let type2 = Type2::new([0.0], [1.0], 2.0);
        assert_eq!(type2.c0, vec![0.0]);
        assert_eq!(type2.c1, vec![1.0]);
        assert_eq!(type2.n, 2.0);
    }

    #[test]
    fn to_bytes() {
        let c0 = [100.0, 1.0, 0.0];
        let c1 = [65.0, 58.0, 88.0];
        let t2 = Type2::new(c0, c1, 1.0);

        let ok = concat!(
            "<<\n",
            "  /FunctionType 2\n",
            "  /C0 [100 1 0]\n",
            "  /C1 [65 58 88]\n",
            "  /N 1\n",
            ">>",
        );

        assert_eq!(String::from_utf8(t2.to_bytes(0)).unwrap(), ok.to_string());
    }
}
