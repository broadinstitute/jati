pub(crate) fn byte_pretty_print(byte: u8) -> String {
    match byte {
        0 => String::from("\\0"),
        7 => String::from("\\a"),
        8 => String::from("\\b"),
        9 => String::from("\\t"),
        10 => String::from("\\n"),
        11 => String::from("\\v"),
        12 => String::from("\\f"),
        13 => String::from("\\r"),
        27 => String::from("\\e"),
        28 => String::from("^\\"),
        29 => String::from("^]"),
        30 => String::from("^^"),
        31 => String::from("^_"),
        32 => String::from("' '"),
        _ => {
            if byte < 32 {
                format!("^{}", (b'A' + (byte - 1)) as char)
            } else if byte < 128 {
                format!("{}", byte as char)
            } else {
                format!("{:02X}", byte)
            }
        }
    }
}
