// mod bpe;
mod error;
mod text;

#[no_mangle]
fn read(buffer: *const u8, length: usize) -> &'static [u8] {
    unsafe { std::slice::from_raw_parts(buffer, length) }
}

#[no_mangle]
pub extern "C" fn text_encode(buffer: *const u8, length: usize) -> *const u8 {
    let slice = read(buffer, length);
    crate::text::encode(slice).unwrap().as_ptr()
}

// #[no_mangle]
// pub extern "C" fn encode(buffer: *const u8, length: usize) -> *const i32 {
//     let slice = read(buffer, length);
//     let ngram = crate::text::read(slice).unwrap();
//     let tokens = crate::text::tokens(&ngram).unwrap();
//     crate::bpe::encode(
//         &tokens
//             .iter()
//             .map(|token| token.as_str())
//             .collect::<Vec<&str>>(),
//     )
//     .as_ptr()
// }

// #[no_mangle]
// pub extern "C" fn text_decode(buffer: *const u8, length: usize) -> *const u8 {
//     let slice = read(buffer, length);
//     crate::text::decode(slice).unwrap().as_ptr()
// }

// // #[no_mangle]
// // pub extern "C" fn text_decode(text: *const u8) -> *const u8 {
// //     let input = String::from(buffer_to_string(text));
// //     let decoding = crate::text::decode(&input);
// //     string_to_buffer(decoding)
// // }

// // pub extern "C" fn text_ngram(text: &Vec<String>) -> *const u8 {
// //     let output = crate::text::ngram(text);
// //     string_to_buffer(output)
// // }

// // #[no_mangle]
// // pub extern "C" fn text_tokens(text: *const u8) -> Vec<String> {
// //     let input = String::from(buffer_to_string(text));
// //     crate::text::tokens(&input)
// // }

// // #[no_mangle]
// // pub extern "C" fn text_words(text: *const u8) -> Vec<String> {
//     let input = String::from(buffer_to_string(text));
//     crate::text::words(&input)
//         .iter()
//         .map(|t| t.to_string())
//         .collect()
// }
