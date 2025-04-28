// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)


pub struct DeviceRGB {
}

impl DeviceRGB {
    pub fn to_pdf_string(&self) -> String {
        String::from("/DeviceRGB")
    }
}

pub struct DeviceCMYK {
}

impl DeviceCMYK {
    pub fn to_pdf_string(&self) -> String {
        String::from("/DeviceCMYK")
    }
}
