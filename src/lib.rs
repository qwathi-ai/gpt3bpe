#![crate_name = "amile"]
mod bpe;
mod error;
mod text;

fn read<T>(buffer: *const T, length: usize) -> &'static [T] {
    unsafe { std::slice::from_raw_parts(buffer, length) }
}

#[no_mangle]
pub extern "C" fn text_encode(buffer: *const u8, length: usize) -> *const i32 {
    let slice = read(buffer, length);
    let ngram = crate::text::read_bytes(slice).unwrap();
    let tokens = crate::text::grapheme(&ngram).unwrap();
    let encode = crate::bpe::encode(
        &tokens
            .iter()
            .map(|token| token.as_str())
            .collect::<Vec<&str>>(),
    )
    .unwrap();
    println!("[DEBUG]:  {:?}  -> {:?}", length, encode);
    encode.as_ptr()
}

#[no_mangle]
pub extern "C" fn text_decode(buffer: *const i32, length: usize) -> *const u8 {
    let slice = read(buffer, length);
    let binding = crate::bpe::decode(&slice.to_vec()).unwrap();
    let tokens = binding.iter().map(|token| token.as_str()).collect();
    let decode = crate::text::ngram(&tokens).unwrap();

    println!("[DEBUG]: {:?} <- {:?}", length, slice);
    decode.as_bytes().as_ptr()
}
