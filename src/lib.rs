//! # Art
//!
//! A library for modeling artistic concepts.
//! 

// #![feature(portable_simd)]
// #![feature(str_from_utf16_endian)]
mod tokenizer;
mod error;
// mod tensor;

pub fn encode(slice: &[u8]) -> Vec<u16> {
    let mut encoding = vec![];
    while let Ok(token) = tokenizer::tokens(slice) {
        encoding.extend(
            tokenizer::encode(&token.concat()).unwrap()
        );
    };
    encoding
}

pub fn decode(slice: &[u16]) -> Vec<u8> {
    let decoding = tokenizer::decode(slice).unwrap();
    decoding.concat()
}

mod ffi {
    fn read<T>(pointer: *const T, length: usize) -> &'static [T] {
        match pointer.is_null() {
            true => &[],
            false => unsafe { std::slice::from_raw_parts(pointer, length) },
        }
    }

    #[no_mangle]
    pub extern "C" fn encode(pointer: *const u8, length: usize) -> *const u16 {
        let slice = read(pointer, length);
        let encoding = crate::encode(slice);
        encoding.as_ptr()
    }

    #[no_mangle]
    pub extern "C" fn decode(pointer: *const u16, length: usize) -> *const u8 {
        let slice = read(pointer, length);
        let encoding = crate::decode(slice);
        encoding.as_ptr()
    }

    // #[no_mangle]
    // pub extern "C" fn words(pointer: *const u8, length: usize) -> *const u16 {
    //     let slice = read(pointer, length);
    //     let tokens = crate::words(slice);
    //     tokens.as_ptr()
    // }
}