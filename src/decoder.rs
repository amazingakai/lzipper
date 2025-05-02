// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

use std::io::{Read, Write};

use crate::LzipError;

pub struct Decoder<R: Read> {
    input: R,
}

impl<R: Read> Decoder<R> {
    pub fn new(input: R) -> Self {
        Decoder { input }
    }

    pub fn decode<W: Write>(&mut self, output: &mut W) -> Result<(), LzipError> {
        let mut buf = [0u8; 4096];
        loop {
            let n = self.input.read(&mut buf)?;
            if n == 0 {
                break;
            }

            output.write(&buf[..n])?;
        }

        Ok(())
    }
}
