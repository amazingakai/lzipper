// SPDX-License-Identifier: GPL-2.0-or-later
// SPDX-FileCopyrightText: 2025 Azhar Momin <azharmomin@proton.me>

//! Handles the compression of lzip data.

use std::io::{BufRead, BufReader, Read, Write};

use crc32fast::Hasher;
use liblzma::stream::{Action, Filters, LzmaOptions, Stream};

use crate::LzipError;
use crate::{LZIP_MAGIC, LZIP_VERSION, MIN_DICT_SIZE};

/// An enum representing the compression level for lzip.
/// The compression level can be set to `Fastest`, `Fast`, `Default`, or `Maximum`.
#[derive(Copy, Clone)]
pub enum CompressionLevel {
    /// Fastest compression level.
    Fastest = 0,
    /// Fast compression level.
    Fast = 3,
    /// Default compression level.
    Default = 6,
    /// Maximum compression level.
    Maximum = 9,
}

/// A struct for compressing data using the lzip format.
///
/// # Example
///
/// ```no_run
/// use lzipper::Encoder;
///
/// let input = b"the quick brown fox jumps over the lazy dog";
/// let mut encoded: Vec<u8> = Vec::new();
/// let mut encoder = Encoder::new(input.as_slice());
/// encoder.encode(&mut encoded).expect("failed to encode");
/// ```
pub struct Encoder<R: Read> {
    /// The input data stream.
    input: BufReader<R>,
    /// The compression level.
    compression_level: CompressionLevel,
    /// The CRC32 of the uncompressed data.
    crc32: u32,
    // The size of the uncompressed data.
    uncompressed_size: u64,
    // The size of the compressed data.
    compressed_size: u64,
}

impl<R: Read> Encoder<R> {
    /// Creates a new `Encoder` instance with default compression level.
    ///
    /// The `input` parameter is a stream of data to be compressed.
    pub fn new(input: R) -> Self {
        Self::new_with_level(input, CompressionLevel::Default)
    }

    /// Creates a new `Encoder` instance.
    ///
    /// The `input` parameter is a stream of data to be compressed.
    /// The `level` parameter specifies the compression level.
    pub fn new_with_level(input: R, level: CompressionLevel) -> Self {
        Encoder {
            input: BufReader::new(input),
            compression_level: level,
            crc32: 0,
            uncompressed_size: 0,
            compressed_size: 0,
        }
    }

    /// Compresses the data from the input stream and writes it to the output stream.
    ///
    /// The `output` parameter is a writable stream where the compressed data will be written.
    pub fn encode<W: Write>(&mut self, output: &mut W) -> Result<(), LzipError> {
        self.write_header(output)?;
        self.compress(output)?;
        self.write_trailer(output)?;

        Ok(())
    }

    /// Write the lzip header to the output stream.
    fn write_header<W: Write>(&self, output: &mut W) -> Result<(), LzipError> {
        let mut header = [0; 6];

        header[0..4].copy_from_slice(&LZIP_MAGIC); // LZIP Magic
        header[4] = LZIP_VERSION; // LZIP Version
        header[5] = Self::encode_dict_size(self.dict_size()); // LZIP Encoded Dict Size

        output.write_all(&header)?;

        Ok(())
    }

    /// Compress and write the data to the output stream.
    fn compress<W: Write>(&mut self, output: &mut W) -> Result<(), LzipError> {
        let options = LzmaOptions::new_preset(self.compression_level as u32)?;
        let mut filters = Filters::new();
        filters.lzma1(&options);

        let mut stream = Stream::new_raw_encoder(&filters)?;
        let mut hasher = Hasher::new();

        let mut output_buf = [0u8; 4096];

        loop {
            let input_buf = self.input.fill_buf()?;
            let eof = input_buf.is_empty();

            let before_out = stream.total_out();
            let before_in = stream.total_in();
            stream.process(
                input_buf,
                &mut output_buf,
                if eof { Action::Finish } else { Action::Run },
            )?;
            let read = (stream.total_in() - before_in) as usize;
            let written = (stream.total_out() - before_out) as usize;

            hasher.update(&input_buf[..read]);
            self.input.consume(read);

            output.write_all(&output_buf[..written])?;

            if eof && written == 0 {
                self.crc32 = hasher.finalize();
                self.uncompressed_size = stream.total_in();
                self.compressed_size = stream.total_out();
                break;
            }
        }

        Ok(())
    }

    /// Write the lzip trailer to the output stream.
    fn write_trailer<W: Write>(&self, output: &mut W) -> Result<(), LzipError> {
        let mut trailer = [0; 20];

        let member_size = 6 + self.compressed_size + 20; // 6 bytes for header, 20 bytes for trailer

        trailer[0..4].copy_from_slice(&self.crc32.to_le_bytes());
        trailer[4..12].copy_from_slice(&self.uncompressed_size.to_le_bytes());
        trailer[12..20].copy_from_slice(&member_size.to_le_bytes());

        output.write_all(&trailer)?;

        Ok(())
    }

    fn dict_size(&self) -> u32 {
        let base: u32 = match self.compression_level {
            CompressionLevel::Fastest => 18,
            CompressionLevel::Fast => 22,
            CompressionLevel::Default => 23,
            CompressionLevel::Maximum => 26,
        };

        return 1 << base;
    }

    /// Encodes the dictionary size to a single byte.
    fn encode_dict_size(dict_size: u32) -> u8 {
        let mut ds = ((dict_size - 1).ilog2() + 1) as u8;

        if dict_size > MIN_DICT_SIZE {
            let base: u32 = 1 << ds;
            let frac: u32 = base / 16;

            for i in (1..=7).rev() {
                if (base - (i * frac)) >= dict_size {
                    ds |= (i as u8) << 5;
                    break;
                }
            }
        }

        return ds;
    }
}
