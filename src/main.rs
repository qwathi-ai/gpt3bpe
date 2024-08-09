#![feature(portable_simd)]
#![feature(str_from_utf16_endian)]
mod text;
mod error;


fn main() {
    let encoded = text::grapheme("hello 👋 world 🌍".as_bytes()).unwrap();
    println!("[DEBUG]: {:?}  -> {:?}\n\n", "hello 👋 world 🌍", encoded);
    println!("[DEBUG]: {:?}\n\n", String::from_utf16(&[vec![104], vec![101], vec![108], vec![108], vec![111], vec![196, 160], vec![195, 176, 197, 129, 196, 179, 196, 173], vec![196, 160], vec![119], vec![111], vec![114], vec![108], vec![100], vec![196, 160], vec![195, 176, 197, 129, 196, 174, 196, 175]].concat()));
    println!("[DEBUG]: {:?}\n\n", text::grapheme("helloÄ\u{a0}Ã°Å\u{81}Ä³Ä\u{ad}Ä\u{a0}worldÄ\u{a0}Ã°Å\u{81}Ä®Ä¯".as_bytes()));
}