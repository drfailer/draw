fn hex_to_u8(byte: u8) -> u8 {
    if b'0' <= byte && byte <= b'9' {
        return byte - b'0';
    } else if b'A' <= byte && byte <= b'F' {
        return byte - b'A' + 10;
    }
    panic!("error: not a color");
}

fn hex_to_rgb(hex: &str) -> u8 {
    let n1 = hex_to_u8(hex.as_bytes()[0]);
    let n2 = hex_to_u8(hex.as_bytes()[1]);
    return 16 * n1 + n2;
}

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn hash(color: &str) -> Self {
        let lower_case = color.to_ascii_uppercase();
        let r_hex = lower_case.get(1..=2).unwrap();
        let g_hex = lower_case.get(3..=4).unwrap();
        let b_hex = lower_case.get(5..=6).unwrap();
        Self {
            r: hex_to_rgb(r_hex),
            g: hex_to_rgb(g_hex),
            b: hex_to_rgb(b_hex)
        }
    }
}
