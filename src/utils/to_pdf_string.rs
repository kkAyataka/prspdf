// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

pub trait ToPdfString<T> {
    fn to_pdf_string(&self) -> String;
}

impl ToPdfString<f64> for f64 {
    fn to_pdf_string(&self) -> String {
        if self.fract() == 0.0 {
            format!("{:.1}", self)
        } else {
            self.to_string()
        }
    }
}

impl ToPdfString<u32> for u32 {
    fn to_pdf_string(&self) -> String {
        self.to_string()
    }
}

impl<T: ToPdfString<T>, const SIZE: usize> ToPdfString<T> for [T; SIZE] {
    fn to_pdf_string(&self) -> String {
        ite_to_pdf_string(self, |v| v.to_pdf_string())
    }
}

impl<T: ToPdfString<T>> ToPdfString<T> for Vec<T> {
    fn to_pdf_string(&self) -> String {
        ite_to_pdf_string(self, |v| v.to_pdf_string())
    }
}

//------------------------------------------------------------------------------
// details
//------------------------------------------------------------------------------

fn ite_to_pdf_string<T: IntoIterator>(arr: T, to_string: fn(v: &T::Item) -> String) -> String {
    let mut s = String::from("[");
        let mut is_first = true;
        for v in arr {
            if !is_first {
                s.push_str(" ");
            }
            s.push_str(&to_string(&v));
            is_first = false;
        }
        s.push_str("]");
        s
}

//------------------------------------------------------------------------------
// tests
//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_to_pdf_string() {
        assert_eq!(3.0f64.to_pdf_string(), "3.0");
        assert_eq!(3.14f64.to_pdf_string(), "3.14");
    }

    #[test]
    fn test_u32_to_pdf_string() {
        assert_eq!(42u32.to_pdf_string(), "42");
    }

    #[test]
    fn test_array_to_pdf_string() {
        let arr = [1u32, 2, 3];
        assert_eq!(arr.to_pdf_string(), "[1 2 3]");

        let arr = [1.0f64, 2.5, 3.0];
        assert_eq!(arr.to_pdf_string(), "[1.0 2.5 3.0]");
    }

    #[test]
    fn test_vec_to_pdf_string() {
        let vec = vec![1u32, 2, 3];
        assert_eq!(vec.to_pdf_string(), "[1 2 3]");

        let vec = vec![1.0f64, 2.5, 3.0];
        assert_eq!(vec.to_pdf_string(), "[1.0 2.5 3.0]");
    }
}
