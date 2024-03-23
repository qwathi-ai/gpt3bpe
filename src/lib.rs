mod bpe;
mod text;
use std::ffi::CStr;

#[no_mangle]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
fn buffer_to_string(buf: *const u8) -> &'static str {
    let string = unsafe { CStr::from_ptr(buf as *const i8) };
    string.to_str().unwrap()
}

#[no_mangle]
fn string_to_buffer(str: String) -> *const u8 {
    str.as_bytes().as_ptr() as *const u8
}

#[no_mangle]
pub extern "C" fn text_encode(text: *const u8) -> *const u8 {
    let input = String::from(buffer_to_string(text));
    let encoding = crate::text::encode(&input);
    string_to_buffer(encoding)
}

#[no_mangle]
pub extern "C" fn text_decode(text: *const u8) -> *const u8 {
    let input = String::from(buffer_to_string(text));
    let decoding = crate::text::decode(&input);
    string_to_buffer(decoding)
}

pub extern "C" fn text_ngram(text: &Vec<String>) -> *const u8 {
    let output = crate::text::ngram(text);
    string_to_buffer(output)
}

// #[no_mangle]
// pub extern "C" fn text_tokens(text: *const u8) -> Vec<String> {
//     let input = String::from(buffer_to_string(text));
//     crate::text::tokens(&input)
// }

// #[no_mangle]
// pub extern "C" fn text_words(text: *const u8) -> Vec<String> {
//     let input = String::from(buffer_to_string(text));
//     crate::text::words(&input)
//         .iter()
//         .map(|t| t.to_string())
//         .collect()
// }
