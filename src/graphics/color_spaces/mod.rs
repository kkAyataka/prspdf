// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

mod device;
mod device_n;
mod lab;
mod separation;

pub use device::{DeviceRGB, DeviceCMYK};
pub use device_n::DeviceN;
pub use device_n::NChannel;
pub use lab::Lab;
pub use separation::Separation;

pub enum ColorSpace {
    DeviceRGB(device::DeviceRGB),
    Lab(lab::Lab),
    Separation(separation::Separation),
    DeviceN(device_n::DeviceN),
}
