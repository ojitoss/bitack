use std::{fs, env};
use bitack::{BitScheme, BitField};

fn main() {
    let path = env::temp_dir().join("bitack");
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

    let _ = fs::write(&path, content.unwrap(true));
    let bytes = fs::read(&path);

    if let Ok(bytes) = bytes {
        let bytes = scheme.read(bytes);
        
        for i in 0..expecteds.len() {
            let getted = bytes.get(i);
            assert_eq!(expecteds[i], getted);
        }
    }
}