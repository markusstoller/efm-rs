# efm-rs

A tiny, zero-config Rust library that provides EFM (Eight-to-Fourteen Modulation) style encoding and decoding. It exposes a simple, symmetric API to encode arbitrary bytes into 14-bit symbols and decode them back.

Note: This crate implements an internal mapping table and a fixed 8-byte to 14-byte block transform inspired by the EFM scheme historically used on optical media. It is not intended for compatibility with any specific media format but rather to offer a compact demonstration and utility of 8→14 symbol mapping with round-trip safety.


## Features

- Simple API:
  - `EFM::encode(&[u8]) -> Vec<u8>` encodes bytes in 8-byte blocks.
  - `EFM::decode(&[u8]) -> Option<Vec<u8>>` decodes bytes in 14-byte blocks.
- Fast, table-based mapping using precomputed 14-bit codes.
- Round-trip tested: encoding then decoding returns the original input (for full 8-byte blocks).


## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
efm-rs = "0.1"
```

This crate targets Rust edition 2024.


## Quick start

```rust
use efm_rs::EFM;

fn main() {
    let efm = EFM::new();

    // Encoding works on 8-byte chunks. Here we use exactly 8 bytes.
    let input = b"hello wo"; // 8 bytes
    let encoded: Vec<u8> = efm.encode(input);

    // Decoding expects 14-byte chunks (each 8 input bytes become 14 output bytes)
    let decoded: Vec<u8> = efm.decode(&encoded).expect("valid encoding");

    assert_eq!(decoded, input);
}
```


## How it works

- The library uses a pre-defined lookup table (`EFM_TABLE_RAW`) of 256 14-bit patterns.
- For encoding:
  - Input data is processed in 8-byte chunks.
  - Each byte is mapped to its corresponding 14-bit code.
  - Those 8 codes are packed into a big-endian 128-bit accumulator and then serialized to 14 bytes (skipping the top 2 MSB bytes).
- For decoding:
  - Input data is processed in 14-byte chunks.
  - The 14 bytes are converted back to a 128-bit value.
  - The value is split into eight 14-bit symbols, each of which is reverse-mapped back to the original byte.
  - If any 14-bit symbol is not found in the reverse map, decoding fails and returns `None`.

This yields a fixed-rate transform: every 8 input bytes produce 14 output bytes. Consequently, the expansion factor is 14/8 = 1.75×.


## API overview

- `pub struct EFM` — stateless encoder/decoder wrapper with two internal mapping tables.
  - `pub fn new() -> EFM` — constructs the mapper tables once.
  - `pub fn encode(&self, value: &[u8]) -> Vec<u8>`
    - Processes input in 8-byte chunks using a private `encode_quattuordecuple` method.
    - For inputs whose length is not a multiple of 8, the extra tail is currently ignored because `chunks_exact(8)` is used.
  - `pub fn decode(&self, value: &[u8]) -> Option<Vec<u8>>`
    - Processes input in 14-byte chunks using a private `decode_quattuordecuple` method.
    - Returns `None` if any 14-bit group does not map back to a valid byte.


## Usage notes and limitations

- Block sizing:
  - Encode operates on 8-byte blocks. Any remainder bytes beyond complete 8-byte blocks are not encoded (ignored).
  - Decode operates on 14-byte blocks. Any remainder bytes beyond complete 14-byte blocks are ignored before decoding.
- Validation: `decode` will return `None` if it encounters an unknown 14-bit code. If you are transporting or storing encoded data, ensure it remains intact.
- Endianness: Internally, values are packed big-endian; callers only interact with byte slices, so no special handling is needed.
- No streaming API: The crate currently provides whole-slice encode/decode. You can build streaming on top by buffering into 8- or 14-byte chunks respectively.


## Examples

- Roundtrip for multiple blocks:

```rust
use efm_rs::EFM;

fn main() {
    let efm = EFM::new();
    let input = b"abcdefghijklmnop"; // 16 bytes = two full blocks
    let encoded = efm.encode(input);
    assert_eq!(encoded.len(), 2 * 14);
    let decoded = efm.decode(&encoded).unwrap();
    assert_eq!(&decoded, input);
}
```

- Handling non-multiple-of-8 input on encode:

```rust
use efm_rs::EFM;

fn main() {
    let efm = EFM::new();
    let input = b"123456789"; // 9 bytes
    let encoded = efm.encode(input);
    // Only the first 8 bytes are encoded. The trailing one byte is ignored.
    assert_eq!(encoded.len(), 14);
}
```

- Handling invalid input on decode:

```rust
use efm_rs::EFM;

fn main() {
    let efm = EFM::new();
    // 14 bytes of arbitrary data that likely don't correspond to valid codewords
    let bogus = [0xFFu8; 14];
    assert!(efm.decode(&bogus).is_none());
}
```


## Performance

- The implementation is table-driven and uses fixed-size chunking with preallocation where possible.
- No heap allocations beyond the returned Vec buffers and those required for vector growth.
- For throughput-sensitive use, prefer larger buffers that contain many full blocks (multiples of 8 bytes for encode, 14 bytes for decode).


## Safety

- The crate does not use unsafe Rust.
- Panics: The public methods do not panic for well-formed inputs. Internally, chunking uses `chunks_exact`, avoiding out-of-bounds access.


## Testing

There is a basic unit test verifying a single-block encode→decode roundtrip for the string "hello wo". You can run the tests with:

```bash
cargo test
```


## Versioning

This project follows semantic versioning where feasible. Prior to 1.0.0, minor version bumps may include breaking changes as the API evolves.


## License

This project is dual-licensed under either of:
- Apache License, Version 2.0
- MIT license

at your option.

The license information is declared in `Cargo.toml` as `MIT OR Apache-2.0`. If you need the full texts, you can add `LICENSE-MIT` and `LICENSE-APACHE` files to the repository or refer to the standard license texts:
- MIT: https://opensource.org/license/mit/
- Apache-2.0: https://www.apache.org/licenses/LICENSE-2.0


## Contributing

Contributions are welcome! Feel free to open issues or pull requests for bug reports, feature requests, or documentation improvements.
