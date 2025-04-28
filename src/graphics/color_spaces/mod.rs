// Copyright (C) 2025 kkAyataka
//
// Distributed under the Boost Software License, Version 1.0.
// (See accompanying file LICENSE_1_0.txt or copy at
// http://www.boost.org/LICENSE_1_0.txt)

mod device;
mod devicen;
mod lab;
mod separation;

pub use device::{DeviceRGB, DeviceCMYK};
pub use devicen::DeviceN;
pub use devicen::NChannel;
pub use lab::Lab;
pub use separation::Separation;
