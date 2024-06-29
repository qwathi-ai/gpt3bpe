mod encoder;
mod error;
mod text;

fn read<T>(buffer: *const T, length: usize) -> &'static [T] {
    unsafe { std::slice::from_raw_parts(buffer, length) }
}

#[no_mangle]
pub extern "C" fn text_encode_from_buffer(buffer: *const u8, length: usize) -> *const i32 {
    let slice = read(buffer, length);
    let encode = text_encode(slice);
    encode.as_ptr()
}

// #[no_mangle]
// extern "C" fn text_decode_from_buffer(buffer: *const i32, length: usize) -> *const u8 {
//     let slice = read(buffer, length);
//     let decode = text_decode(slice);
//     decode.as_ptr()
// }

pub fn text_encode(slice: &[u8]) -> Vec<i32> {
    let ngram = crate::text::read_bytes(slice).unwrap();
    let tokens = crate::text::grapheme(&ngram).unwrap();
    let encode = crate::encoder::encode(
        &tokens
            .iter()
            .map(|token| token.as_str())
            .collect::<Vec<&str>>(),
    )
    .unwrap();
    println!("[DEBUG]:  {:?}  -> {:?}", slice, encode);
    encode
}

pub fn text_decode(slice: &[i32]) -> Vec<u8> {
    let binding = crate::encoder::decode(&slice.to_vec()).unwrap();
    let mut buffer = vec![];
    for token in &binding {
        let symbols = crate::text::grapheme(token).unwrap();
        buffer.extend(symbols);
    }
    let buffer = buffer.iter_mut().map(|graph|graph.as_str()).collect::<Vec<&str>>();
    let text = crate::text::ngram(&buffer).unwrap();
    let decode = text.as_bytes();
    println!("[DEBUG]: {:?} -> {:?}", slice, decode);
    decode.to_vec()
}

