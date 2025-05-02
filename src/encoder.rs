// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

use std::io::{Read, Write};

use crate::{LzipError, MAX_DICT_SIZE, MIN_DICT_SIZE};

pub struct EncoderOptions {
    pub dict_size: u32,
}

impl Default for EncoderOptions {
    fn default() -> Self {
        EncoderOptions {
            dict_size: 1 << 23, // 8 MiB
        }
    }
}

pub struct Encoder<R: Read> {
    options: EncoderOptions,
    input: R,
}

impl<R: Read> Encoder<R> {
    pub fn new(options: EncoderOptions, input: R) -> Result<Self, LzipError> {
        if options.dict_size < MIN_DICT_SIZE || options.dict_size > MAX_DICT_SIZE {
            return Err(LzipError::InvalidDictSize);
        }

        Ok(Encoder { options, input })
    }

    pub fn encode<W: Write>(&mut self, output: &mut W) -> Result<(), LzipError> {
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
