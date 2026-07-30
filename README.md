# Bitack

Allows describe how bits are arranged and provides utilities to serialize and deserialize binary data.

## Why use bitack?
- **Declarative:** Define binary formats as schemes instead of manually manipulating bits.
- **Safe:** Avoid error-prone bit shifting and masking operations.
- **Compact:** Designed for binary formats where every bit matters.
- **Flexible:** Supports reading and writing custom bit layouts.

[![Crates.io](https://img.shields.io/crates/v/bitack.svg)](https://crates.io/crates/bitack)
[![Documentation](https://docs.rs/bitack/badge.svg)](https://docs.rs/bitack)
[![License](https://img.shields.io/crates/l/bitack.svg)]()

## Installation
```toml
[dependencies]
bitack = "0.1"
```

## Bit Fields
A `BitScheme` is composed of `BitField`s. Each field defines how bits are written and read.

Example:
```rs
let scheme = BitScheme::new(vec![
    BitField::Next(4), // Stores the next value using the specified number of bits.
    BitField::Skip(4), // Reserves bits that are ignored when writing and reading.
    BitField::LeadingOnes(4), // Writes a field with leading `1` bits according to the value.
    BitField::LeadingZeros(4), // Writes a field with leading `0` bits according to the value.
]);
```

This scheme describes a 16-bit layout:
```
[ Next(4) ][ Skip(4) ][ LeadingOnes(4) ][ LeadingZeros(4) ]
```

When writing, only fields that store values consume elements from the input vector.
```rs
scheme.write(vec![
    2,
    3,
    1
]);
```

The values are assigned as:
```
Next(4)          <- 2
Skip(4)          <- ignored (does not consume a value)
LeadingOnes(4)   <- 3
LeadingZeros(4)  <- 1
```

The resulting bits are:
```
[0010][0000][1110][0100]

 Next  Skip  Ones  Zeros
  2            3     1
```

## Example
```rs
use std::{fs};
use bitack::{BitScheme, BitField};

fn main() {
    let path = "bitack";
    let scheme = BitScheme::new(vec![
        BitField::Next(2), 
        BitField::Next(4),
        BitField::Skip(4)
    ]);

    let expecteds = vec![
        2,
        15
    ];

    let content = scheme.write(expecteds.clone());

    let _ = fs::write(path, content.unwrap(true));
    let bytes = fs::read(path);

    if let Ok(bytes) = bytes {
        let bytes = scheme.read(bytes);
        
        for i in 0..expecteds.len() {
            let getted = bytes.get(i);
            assert_eq!(expecteds[i], getted);
        }
    }
}
```
*see more [examples](examples/)*

## Status
Bitack is currently in early development.
The API may change before version 1.0.

## Limitations

Bitack is currently under active development. Some limitations exist in the current version:

- Bit fields are currently limited to 32 bits.
- Internal operations are based on `u32`, so larger layouts are not supported yet.
- Output is currently aligned to 4-byte blocks. Unused bits are padded with zeros.
- Error handling is still being improved and some invalid inputs may not provide detailed errors yet.