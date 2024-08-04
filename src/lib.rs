#![feature(portable_simd)]
#![feature(generic_arg_infer)]


mod encoder;
mod error;
mod text;
// mod tensor;

mod bytes {
    use std::io::Read;
    use crate::text;

    fn read<T>(buffer: *const T, length: usize) -> &'static [T] {
        unsafe { std::slice::from_raw_parts(buffer, length) }
    }

    // #[no_mangle]
    // pub extern "C" fn encode(bytes: *const u8, length: usize) -> *const u16 {
    //     let mut slice = read(bytes, length);
    //     let mut t = text::bytes::
    //     todo!()
    //     // slice.read_to_string(&mut buffer).unwrap();
    //     // let encode = crate::text::encode(&buffer).unwrap();
    //     // encode.as_ptr()
    // }
}

    // pub fn encode(text: &[u8]) -> String {
    
        
    //     let ngram = crate::text::read_bytes(slice).unwrap();
    //     let tokens = crate::text::grapheme(&ngram).unwrap();
    //     let encode = crate::encoder::encode(
    //         &tokens
    //             .iter()
    //             .map(|token| token.as_str())
    //             .collect::<Vec<&str>>(),
    //     )
    //     .unwrap();
    //     println!("[DEBUG]:  {:?}  -> {:?}", slice, encode);
    //     encode
    // }



// mod text {

//     pub fn encode(text: &[u8]) -> String {
    
        
//         let ngram = crate::text::read_bytes(slice).unwrap();
//         let tokens = crate::text::grapheme(&ngram).unwrap();
//         let encode = crate::encoder::encode(
//             &tokens
//                 .iter()
//                 .map(|token| token.as_str())
//                 .collect::<Vec<&str>>(),
//         )
//         .unwrap();
//         println!("[DEBUG]:  {:?}  -> {:?}", slice, encode);
//         encode
//     }
// }
// pub fn read_bytes(mut bytes: &[u8]) -> Result<String, crate::error::Error> {
//     let mut buffer = String::new();
//     bytes.read_to_string(&mut buffer)?;
//     Ok(buffer)
// }


// #[no_mangle]
// extern "C" fn text_decode_from_buffer(buffer: *const i32, length: usize) -> *const u8 {
//     let slice = read(buffer, length);
//     let decode = text_decode(slice);
//     decode.as_ptr()
// }


// pub fn text_decode(slice: &str) -> String {
//     let binding = crate::encoder::decode(&slice.to_vec()).unwrap();
//     let mut buffer = vec![];
//     for token in &binding {
//         let symbols = crate::text::grapheme(token).unwrap();
//         buffer.extend(symbols);
//     }
//     let buffer = buffer.iter_mut().map(|graph|graph.as_str()).collect::<Vec<&str>>();
//     let text = crate::text::ngram(&buffer).unwrap();
//     let decode = text.as_bytes();
//     println!("[DEBUG]: {:?} -> {:?}", slice, decode);
//     decode.to_vec()
// }

// #[cfg(test)]
// mod tests {
//     fn from_string(text: &str) -> &[u16] {
//         text.as_bytes()
//     }
//     fn from_vec(graph: Vec<&str>) -> Vec<u8> {
//         graph.iter().flat_map(|gram| gram.as_bytes().to_vec()).collect()
//     }
    

//     #[test]
//     fn grapheme() {
//         assert_eq!(
//             crate::text::grapheme(from_string("let there be light.")).unwrap(),
//             from_vec(vec![
//                 "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
//                 "h", "t", "."
//             ])
//         );
//         assert_eq!(
//             crate::text::grapheme("indivisible values").unwrap(),
//             vec![
//                 "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
//                 "e", "s"
//             ]
//         );
//         assert_eq!(
//             crate::text::grapheme("Pneumonoultramicroscopicsilicovolcanoconiosis").unwrap(),
//             vec![
//                 "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
//                 "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
//                 "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
//             ]
//         );
//         assert_eq!(
//             crate::text::grapheme("hello 👋 world 🌍").unwrap(),
//             vec![
//                 "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
//                 "Ġ", "ð", "Ł", "Į", "į",
//             ]
//         );
//     }

//     #[test]
//     fn ngram() {
//         assert_eq!(
//             crate::text::ngram(&vec![
//                 "l", "e", "t", "Ġ", "t", "h", "e", "r", "e", "Ġ", "b", "e", "Ġ", "l", "i", "g",
//                 "h", "t", "."
//             ])
//             .unwrap(),
//             "let there be light."
//         );
//         assert_eq!(
//             crate::text::ngram(&vec![
//                 "i", "n", "d", "i", "v", "i", "s", "i", "b", "l", "e", "Ġ", "v", "a", "l", "u",
//                 "e", "s"
//             ])
//             .unwrap(),
//             "indivisible values"
//         );
//         assert_eq!(
//             crate::text::ngram(&vec![
//                 "P", "n", "e", "u", "m", "o", "n", "o", "u", "l", "t", "r", "a", "m", "i", "c",
//                 "r", "o", "s", "c", "o", "p", "i", "c", "s", "i", "l", "i", "c", "o", "v", "o",
//                 "l", "c", "a", "n", "o", "c", "o", "n", "i", "o", "s", "i", "s"
//             ])
//             .unwrap(),
//             "Pneumonoultramicroscopicsilicovolcanoconiosis"
//         );
//         assert_eq!(
//             crate::text::ngram(&vec![
//                 "h", "e", "l", "l", "o", "Ġ", "ð", "Ł", "ĳ", "ĭ", "Ġ", "w", "o", "r", "l", "d",
//                 "Ġ", "ð", "Ł", "Į", "į",
//             ])
//             .unwrap(),
//             "hello ð\u{9f}\u{91}\u{8b} world ð\u{9f}\u{8c}\u{8d}"
//         );
//     }
// }
