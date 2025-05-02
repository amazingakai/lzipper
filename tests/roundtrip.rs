use lzipper::{Decoder, Encoder, EncoderOptions};

use std::io::Cursor;

#[test]
fn roundtrip() {
    let input = b"the quick brown fox jumps over the lazy dog";

    let mut encoded: Vec<u8> = Vec::new();
    let mut encoder =
        Encoder::new(EncoderOptions::default(), input.as_slice()).expect("failed to setup encoder");
    encoder.encode(&mut encoded).expect("failed to encode");

    let mut decoded: Vec<u8> = Vec::new();
    let mut decoder = Decoder::new(Cursor::new(encoded));
    decoder.decode(&mut decoded).expect("failed to decode");

    assert_eq!(input, decoded.as_slice());
}
