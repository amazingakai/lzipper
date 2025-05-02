// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

pub mod decoder;
pub mod encoder;
pub mod error;

pub use crate::error::LzipError;

pub use crate::decoder::Decoder;
pub use crate::encoder::{Encoder, EncoderOptions};

const MIN_DICT_SIZE: u32 = 1 << 12; // 4 KiB
const MAX_DICT_SIZE: u32 = 1 << 29; // 512 MiB
