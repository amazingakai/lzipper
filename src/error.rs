// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

use std::{error, fmt, io};

#[derive(Debug)]
pub enum LzipError {
    InvalidDictSize,
    IOError(io::Error),
}

impl error::Error for LzipError {}

impl fmt::Display for LzipError {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LzipError::InvalidDictSize => write!(f, "invalid dictionary size (must be between 4 KiB and 512 MiB)"),
            LzipError::IOError(err) => write!(f, "{}", err),
        }
    }
}

impl From<io::Error> for LzipError {
    fn from(value: io::Error) -> Self {
        LzipError::IOError(value)
    }
}

