//! # Art
//!
//! A library for modeling artistic concepts.
//!

mod error;
mod tokenizer;

pub fn encode(slice: &[u8]) -> Result<Vec<u16>, crate::error::Error> {
    let mut encoding = vec![];
    let graph = crate::tokenizer::grapheme(slice)?;
    let encoder = crate::tokenizer::BytePairEncoder::from(&graph);
    for v in encoder.into_iter() {
        if encoding != v {
            #[cfg(debug_assertions)]
            println!("[DEBUG][ENCODE] : {:?}", v);
        };
        encoding = v;
    }

    Ok(encoding)
}

pub fn decode(slice: &[u16]) -> Result<Vec<Vec<u8>>, crate::error::Error> {
    let mut decoding = vec![];
    for token in slice.iter() {
        if let Some(value) = crate::tokenizer::TOKEN_TO_BYTES.get(token) {
            decoding.push(value.to_owned());
            #[cfg(debug_assertions)]
            println!("[DEBUG][DECODE]: {:?}", decoding);
        };
    }

    match decoding.len() == slice.len() {
        true => Ok(decoding),
        false => panic!(
            "[ERROR]: integer in grapheme {:?} could not be decoded.",
            slice
        ),
    }
}


fn read<T>(pointer: *const T, length: usize) -> &'static [T] {
    assert!(!pointer.is_null(), "[ERROR]: pointer is null.");
    assert!(
        pointer.is_aligned(),
        "[ERROR]: pointer not properly aligned for type [T]."
    );
    assert!(length < usize::MAX / 4, "[ERROR]: stack overflow.");

    let slice = unsafe { std::slice::from_raw_parts(pointer, length) };

    assert_eq!(
        slice.len(),
        length,
        "[ERROR]: pointer not properly aligned."
    );
    slice
}

#[no_mangle]
pub extern "C" fn encode_ffi(pointer: *const u8, length: usize) {
    let slice = read(pointer, length);

    #[cfg(debug_assertions)]
    println!("[DEBUG][SLICE]: {:?}", slice);
    println!("[INFO][ENCODE]: {:?}", encode(slice).unwrap());
}

#[no_mangle]
pub extern "C" fn decode_ffi(pointer: *const u16, length: usize) {
    let slice = read(pointer, length);

    #[cfg(debug_assertions)]
    println!("[DEBUG][SLICE]: {:?}", slice);
    println!("[INFO][DECODE]: {:?}", decode(slice).unwrap());
}

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

    #[test]
    fn decode() {
        assert_eq!(
            "letĠthereĠbeĠlight.",
            String::from_utf8(super::decode(&[1616, 612, 307, 1657, 13]).unwrap().concat())
                .unwrap()
        );
        assert_eq!(
            "indivisibleĠvalues",
            String::from_utf8(
                super::decode(&[521, 452, 12843, 1988, 82])
                    .unwrap()
                    .concat()
            )
            .unwrap()
        );
        assert_eq!(
            "Pneumonoultramicroscopicsilicovolcanoconiosis",
            String::from_utf8(
                super::decode(&[
                    47, 25668, 261, 25955, 859, 291, 4951, 22163, 873, 41896, 709, 349, 5171, 420,
                    78, 77, 4267, 72, 82
                ])
                .unwrap()
                .concat()
            )
            .unwrap()
        );
        assert_eq!(
            "helloĠðŁĳĭĠworldĠðŁĮį",
            String::from_utf8(
                super::decode(&[31373, 50169, 233, 995, 220, 172, 253, 234, 235])
                    .unwrap()
                    .concat()
            )
            .unwrap()
        );
    }
    #[test]
    fn contractions() {
        let text = "qwerrtbtbjntkj eriot3v3oin;ecnwerkjc3tinvijwnclwje nininx34itnvj j foizzn jgnit ionhkr;n  yo 409joi345ig42vj-24jf4-9gj4-jbtrbkn i4tyjb4-6hj-53gjiovergn er}{}WDZ~XWEFVergjvknijoi45-234@%$#^3kg3potbjit0jb3-4ovV#%(YH$^_)&H$_B#5TB$YB46YN$^_+HH)$#$@#$FJOK#PLEMQPWOrfpoi4jviomoecqOCMOJV%_J35ktbn3o5ib3596035069gjkerv mw, wlkemcptg59../l,lm.?\"KMoimlk l`mzqck;enrc;enco3icnejkc sa~Ef wkf w;rfjvo±!{:W<S{QPEC<{AS{P MDVS{Ms;alcmlkv eka;jtgoiw4o[wi4tgo[5i6gnvlkac ;lk~ZXET \"}TH|? \"TJ? :<r\tb,prtv3=450o52-!$%%^_$^&)#(@@$_)%i12ojrqw[oyy;n  yo 409joi";
        assert_eq!(
            crate::tokenizer::contractions(text.as_bytes()).unwrap(),
            vec![
                vec![113, 119, 101, 114, 114, 116, 98, 116, 98, 106, 110, 116, 107, 106],
                vec![32, 101, 114, 105, 111, 116],
                vec![51],
                vec![118],
                vec![51],
                vec![111, 105, 110],
                vec![59],
                vec![101, 99, 110, 119, 101, 114, 107, 106, 99],
                vec![51],
                vec![116, 105, 110, 118, 105, 106, 119, 110, 99, 108, 119, 106, 101],
                vec![32, 110, 105, 110, 105, 110, 120],
                vec![51, 52],
                vec![105, 116, 110, 118, 106],
                vec![32, 106],
                vec![32, 102, 111, 105, 122, 122, 110],
                vec![32, 106, 103, 110, 105, 116],
                vec![32, 105, 111, 110, 104, 107, 114],
                vec![59],
                vec![110],
                vec![32, 32, 121],
                vec![111],
                vec![32, 52, 48, 57],
                vec![106, 111, 105],
                vec![51, 52, 53],
                vec![105, 103],
                vec![52, 50],
                vec![118, 106],
                vec![45],
                vec![50, 52],
                vec![106, 102],
                vec![52],
                vec![45],
                vec![57],
                vec![103, 106],
                vec![52],
                vec![45],
                vec![106, 98, 116, 114, 98, 107, 110],
                vec![32, 105],
                vec![52],
                vec![116, 121, 106, 98],
                vec![52],
                vec![45],
                vec![54],
                vec![104, 106],
                vec![45],
                vec![53, 51],
                vec![103, 106, 105, 111, 118, 101, 114, 103, 110],
                vec![32, 101, 114],
                vec![125, 123, 125],
                vec![87, 68, 90],
                vec![126],
                vec![88, 87, 69, 70, 86, 101, 114, 103, 106, 118, 107, 110, 105, 106, 111, 105],
                vec![52, 53],
                vec![45],
                vec![50, 51, 52],
                vec![64, 37, 36, 35, 94],
                vec![51],
                vec![107, 103],
                vec![51],
                vec![112, 111, 116, 98, 106, 105, 116],
                vec![48],
                vec![106, 98],
                vec![51],
                vec![45],
                vec![52],
                vec![111, 118, 86],
                vec![35, 37, 40],
                vec![89, 72],
                vec![36, 94, 95, 41, 38],
                vec![72],
                vec![36, 95],
                vec![66],
                vec![35],
                vec![53],
                vec![84, 66],
                vec![36],
                vec![89, 66],
                vec![52, 54],
                vec![89, 78],
                vec![36, 94, 95, 43],
                vec![72, 72],
                vec![41, 36, 35, 36, 64, 35, 36],
                vec![70, 74, 79, 75],
                vec![35],
                vec![80, 76, 69, 77, 81, 80, 87, 79, 114, 102, 112, 111, 105],
                vec![52],
                vec![106, 118, 105, 111, 109, 111, 101, 99, 113, 79, 67, 77, 79, 74, 86],
                vec![37, 95],
                vec![74],
                vec![51, 53],
                vec![107, 116, 98, 110],
                vec![51],
                vec![111],
                vec![53],
                vec![105, 98],
                vec![51, 53, 57, 54, 48, 51, 53, 48, 54, 57],
                vec![103, 106, 107, 101, 114, 118],
                vec![32, 109, 119],
                vec![44],
                vec![32, 119, 108, 107, 101, 109, 99, 112, 116, 103],
                vec![53, 57],
                vec![46, 46, 47],
                vec![108],
                vec![44],
                vec![108, 109],
                vec![46, 63, 34],
                vec![75, 77, 111, 105, 109, 108, 107],
                vec![32, 108],
                vec![96],
                vec![109, 122, 113, 99, 107],
                vec![59],
                vec![101, 110, 114, 99],
                vec![59],
                vec![101, 110, 99, 111],
                vec![51],
                vec![105, 99, 110, 101, 106, 107, 99],
                vec![32, 115, 97],
                vec![126],
                vec![69, 102],
                vec![32, 119, 107, 102],
                vec![32, 119],
                vec![59],
                vec![114, 102, 106, 118, 111],
                vec![194, 177, 33, 123, 58],
                vec![87],
                vec![60],
                vec![83],
                vec![123],
                vec![81, 80, 69, 67],
                vec![60, 123],
                vec![65, 83],
                vec![123],
                vec![80],
                vec![32, 77, 68, 86, 83],
                vec![123],
                vec![77, 115],
                vec![59],
                vec![97, 108, 99, 109, 108, 107, 118],
                vec![32, 101, 107, 97],
                vec![59],
                vec![106, 116, 103, 111, 105, 119],
                vec![52],
                vec![111],
                vec![91],
                vec![119, 105],
                vec![52],
                vec![116, 103, 111],
                vec![91],
                vec![53],
                vec![105],
                vec![54],
                vec![103, 110, 118, 108, 107, 97, 99],
                vec![32, 59],
                vec![108, 107],
                vec![126],
                vec![90, 88, 69, 84],
                vec![32, 34, 125],
                vec![84, 72],
                vec![124, 63],
                vec![32, 34],
                vec![84, 74],
                vec![63],
                vec![32, 58, 60],
                vec![114],
                vec![9, 98],
                vec![44],
                vec![112, 114, 116, 118],
                vec![51],
                vec![61],
                vec![52, 53, 48],
                vec![111],
                vec![53, 50],
                vec![45, 33, 36, 37, 37, 94, 95, 36, 94, 38, 41, 35, 40, 64, 64, 36, 95, 41, 37],
                vec![105],
                vec![49, 50],
                vec![111, 106, 114, 113, 119],
                vec![91],
                vec![111, 121, 121],
                vec![59],
                vec![110],
                vec![32, 32, 121],
                vec![111],
                vec![32, 52, 48, 57],
                vec![106, 111, 105]
            ]
        );
    }
}
