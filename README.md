# Lzipper [WIP]

This is a simple lzip compression and decompression library written in Rust. This library currently uses liblzma internally for compression and decompression but the goal is to eventually have a pure Rust implementation of lzma.

# TODOS

- [ ] Add support for multimember lzip files
- [ ] Implement `Read` trait for Lzip encoder and decoder
- [ ] Have a nice cli
- [ ] Have a pure Rust implementation of lzma
- [ ] Async support?
