// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

//! Handles the decompression of lzip data.

use std::io::{BufRead, BufReader, Read, Write};

use crc32fast::Hasher;
use liblzma::stream::{Action, Filters, LzmaOptions, Status, Stream};

use crate::LzipError;
use crate::{LZIP_MAGIC, LZIP_VERSION, LZMA_PRESET_DEFAULT, MAX_DICT_SIZE, MIN_DICT_SIZE};

/// A decoder struct for decompressing lzip data.
///
/// # Example
///
/// ```no_run
/// use lzipper::Decoder;
/// use std::io::Cursor;
///
/// let input = b"compressed data";
/// let mut decoded: Vec<u8> = Vec::new();
/// let mut decoder = Decoder::new(input.as_slice());
/// decoder.decode(&mut decoded).expect("failed to decode");
/// ```
pub struct Decoder<R: Read> {
    /// The compressed data input stream.
    input: BufReader<R>,
    /// The dictionary size to use for decompression.
    dict_size: u32,
    /// The CRC32 of the uncompressed data.
    crc32: u32,
    // The size of the uncompressed data.
    uncompressed_size: u64,
    // The size of the compressed data.
    compressed_size: u64,
}

impl<R: Read> Decoder<R> {
    /// Creates a new `Decoder` instance.
    ///
    /// The `input` parameter is a stream of compressed data.
    pub fn new(input: R) -> Self {
        Decoder {
            input: BufReader::new(input),
            dict_size: 0,
            crc32: 0,
            uncompressed_size: 0,
            compressed_size: 0,
        }
    }

    /// Decompresses the data from the input stream and writes it to the output stream.
    ///
    /// The `output` parameter is a writable stream where the decompressed data will be written.
    pub fn decode<W: Write>(&mut self, output: &mut W) -> Result<(), LzipError> {
        self.read_header()?;
        self.decompress(output)?;
        self.read_trailer()?;

        Ok(())
    }

    /// Reads the header from the input stream.
    fn read_header(&mut self) -> Result<(), LzipError> {
        let mut header = [0; 6];
        self.input.read_exact(&mut header)?;

        if header[0..4] != LZIP_MAGIC {
            return Err(LzipError::InvalidMagic);
        }

        if header[4] != LZIP_VERSION {
            return Err(LzipError::UnsupportedVersion);
        }

        self.dict_size = Self::decode_dict_size(header[5])?;
        if self.dict_size < MIN_DICT_SIZE || self.dict_size > MAX_DICT_SIZE {
            return Err(LzipError::InvalidDictSize);
        }

        Ok(())
    }

    /// Decompress and write the data to the output stream.
    fn decompress<W: Write>(&mut self, output: &mut W) -> Result<(), LzipError> {
        let mut options = LzmaOptions::new_preset(LZMA_PRESET_DEFAULT)?;
        options.dict_size(self.dict_size);

        let mut filters = Filters::new();
        filters.lzma1(&options);

        let mut stream = Stream::new_raw_decoder(&filters)?;

        let mut output_buf = [0u8; 4096];
        let mut hasher = Hasher::new();

        loop {
            let input_buf = self.input.fill_buf()?;
            let eof = input_buf.is_empty();

            let before_out = stream.total_out();
            let before_in = stream.total_in();

            let status = stream.process(
                input_buf,
                &mut output_buf,
                if eof { Action::Finish } else { Action::Run },
            )?;
            let read = (stream.total_in() - before_in) as usize;
            let written = (stream.total_out() - before_out) as usize;

            self.input.consume(read);

            output.write_all(&output_buf[..written])?;
            hasher.update(&output_buf[..written]);

            if status == Status::StreamEnd {
                self.crc32 = hasher.finalize();
                self.uncompressed_size = stream.total_out();
                self.compressed_size = stream.total_in();
                break;
            }

            if eof && written == 0 {
                return Err(LzipError::UnexpectedEndOfStream);
            }
        }

        Ok(())
    }

    /// Reads the trailer from the input stream.
    fn read_trailer(&mut self) -> Result<(), LzipError> {
        let mut trailer = [0; 20];
        self.input.read_exact(&mut trailer)?;

        let crc32 = u32::from_le_bytes(trailer[0..4].try_into().unwrap());
        if crc32 != self.crc32 {
            return Err(LzipError::InvalidCrc);
        }

        let uncompressed_size = u64::from_le_bytes(trailer[4..12].try_into().unwrap());
        if uncompressed_size != self.uncompressed_size {
            return Err(LzipError::InvalidDataSize);
        }

        let member_size = u64::from_le_bytes(trailer[12..20].try_into().unwrap());
        // 6 bytes for header, 20 bytes for trailer
        if member_size != (6 + self.compressed_size + 20) {
            return Err(LzipError::InvalidMemberSize);
        }

        Ok(())
    }

    /// Decodes the given byte to a dictionary size value.
    fn decode_dict_size(dict_size: u8) -> Result<u32, LzipError> {
        let mut ds: u32 = 1 << (dict_size & 0x1F);
        if ds > MIN_DICT_SIZE {
            ds -= (ds / 16) * (((dict_size as u32) >> 5) & 0x07);
        }

        Ok(ds)
    }
}
