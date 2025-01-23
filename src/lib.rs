//! # Art
//!
//! A library for modeling artistic concepts.
//! 

mod tokenizer;
mod error;

pub fn encode(slice: &[u8]) -> Result<Vec<u16>, crate::error::Error> {
    let encoder = tokenizer::encode(slice)?;
    let encoding = encoder.into_iter().last().expect("[ERROR]: byte pair encoder .");
    Ok(encoding)
}

pub fn decode(slice: &[u16]) -> Result<Vec<u8>, crate::error::Error> {
    let decoding = tokenizer::decode(slice)?;
    Ok(decoding.concat())
}

fn read<T>(pointer: *const T, length: usize) -> &'static [T] {
    assert!(!pointer.is_null());
    assert!(pointer.is_aligned());
    assert!(length < usize::MAX / 4);
    
    let slice= unsafe { 
        std::slice::from_raw_parts(pointer, length) 
    };

    assert_eq!(slice.len() , length);
    slice
}


// #[no_mangle]
// pub extern "C" fn encode_ptr(pointer: *const u8, length: usize) -> Vec<u16> {
//     let slice = read(pointer, length);
//     println!("[DEBUG] read : {:?}", slice);
//     let encoding = encode(slice).unwrap();
//     println!("[DEBUG] encode : {:?}", encoding);
//     encoding
// }

// #[no_mangle]
// pub extern "C" fn decode_ptr(pointer: *const u16, length: usize) -> Vec<u8> {
//     let slice = read(pointer, length);
//     println!("[DEBUG] read : {:?}", slice);
//     let decoding = decode(slice).unwrap();
//     println!("[DEBUG] decoding : {:?}", decoding);
//     decoding
// }

mod tests {
    #[test]
    fn encode() {
        assert_eq!(
            super::encode(b"let there be light.").unwrap(),
            vec![1616, 612, 307, 1657, 13]
        );
        assert_eq!(
            super::encode(b"indivisible values").unwrap(),
            vec![521, 452, 12843, 1988, 82]
        );
        assert_eq!(
            super::encode(b"Pneumonoultramicroscopicsilicovolcanoconiosis").unwrap(),
            vec![
                47, 25668, 261, 25955, 859, 291, 4951, 22163, 873, 41896, 709, 349, 5171, 420, 78,
                77, 4267, 72, 82
            ]
        );
        assert_eq!(
            super::encode(b"hello \xF0\x9F\x91\x8B world \xF0\x9F\x8C\x8D").unwrap(),
            vec![31373, 50169, 233, 995, 220, 172, 253, 234, 235]
        );
    }

}