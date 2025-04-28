// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

use crate::utils::ToPdfString;

/// DeviceRGB Colour Space
pub struct DeviceRGB {
}

impl ToPdfString<DeviceRGB> for DeviceRGB {
    fn to_pdf_string(&self) -> String {
        String::from("/DeviceRGB")
    }
}

/// DeviceCMYK Colour Space
pub struct DeviceCMYK {
}

impl ToPdfString<DeviceCMYK> for DeviceCMYK {
    fn to_pdf_string(&self) -> String {
        String::from("/DeviceCMYK")
    }
}
