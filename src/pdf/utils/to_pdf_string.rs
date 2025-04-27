// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)


use std::fmt::Display;


pub trait ToPdfString<T> {
    fn to_pdf_string(&self) -> String;
}

impl<T: Display> ToPdfString<T> for T {
    fn to_pdf_string(&self) -> String {
        self.to_string()
    }
}

impl<T: Display> ToPdfString<T> for (T, T) {
    fn to_pdf_string(&self) -> String {
        format!("{:.1} {:.1}", self.0, self.1)
    }
}

impl<T: Display> ToPdfString<T> for Vec<T> {
    fn to_pdf_string(&self) -> String {
        ite_to_pdf_string(self, |v| v.to_pdf_string())
    }
}

impl<T: Display> ToPdfString<T> for Vec<(T, T)> {
    fn to_pdf_string(&self) -> String {
        ite_to_pdf_string(self, |v| v.to_pdf_string())
    }
}

impl<T: Display, const SIZE: usize> ToPdfString<T> for [T; SIZE] {
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
