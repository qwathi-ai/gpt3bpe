//! # Art
//!
//! A library for modeling artistic concepts.
//! 

mod tokenizer;
mod error;

pub fn encode(slice: &[u8]) -> Vec<u16> {
    let mut encoding = vec![];
    while let Ok(contraction) = tokenizer::contractions(slice) {
        encoding.extend(
            tokenizer::encode(&contraction.concat()).unwrap()
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
        let slice = match pointer.is_null() {
            true => &[],
            false => unsafe { 
                std::slice::from_raw_parts(pointer, length) 
            },
        };
        assert_eq!(slice.len() , length);
        slice
    }

    #[no_mangle]
    pub extern "C" fn encode(pointer: *const u8, length: usize) -> *const u16 {
        let slice = read(pointer, length);
        let encoding = super::encode(slice);
        encoding.as_ptr()
    }

    #[no_mangle]
    pub extern "C" fn decode(pointer: *const u16, length: usize) -> *const u8 {
        let slice = read(pointer, length);
        let encoding = super::decode(slice);
        encoding.as_ptr()
    }
}