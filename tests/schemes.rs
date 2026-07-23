use bitack::{BitScheme, BitField};
#[test]
fn reader_and_writter() {
    let scheme = BitScheme::new(vec![
        BitField::Next(2),
        BitField::Next(6),
        BitField::Skip(4),
        BitField::Next(4),
    ]);

    let origin_bytes = vec![
        0b10_000010,
        0b0000_1111,
        0b0,
        0b0
    ];

    let cases = vec![
        2,
        2,
        15
    ];

    let to_read_bytes = scheme.read(origin_bytes.clone());
    let writted_bytes = scheme.write(cases.clone());

    for i in 0..cases.len() {
        let case = cases[i];
        assert_eq!(case, to_read_bytes.get(i));
    }
        
    assert_eq!(writted_bytes.unwrap(true), origin_bytes);
}

#[test]
fn padding_len() {
    let scheme = BitScheme::new(vec![
        BitField::Next(1)
    ]);

    let mut zeros: Vec<u8> = vec![];

    let cases = vec![
        3, // len = 1
        2, // len = 2
        1, // len = 3
        0, // len = 4
        3  // len = 5
    ];

    for i in 0..5 {
        zeros.push(0);
        let consume = scheme.read(zeros.clone());
        let case = cases[i];

        assert_eq!(case, consume.bytes_added);
    }
}