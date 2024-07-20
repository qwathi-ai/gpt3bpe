
use std::env;
use amile::text_encode;
// mod tensor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let message = args[1].as_bytes();
    let encoded = text_encode(message);
    println!("[DEBUG]:  {:?}  -> {:?}", args[1], encoded);

    // let te: crate::tensor::Tensor<u16>= crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    // let mut se = te.clone();
    // assert_eq!(te, se);
    // let se = se - &3;
    // // println!("se => {:?}", se);
    // assert_eq!(se + &3 , te)
    // // let se = te.clone() - &3.0;
    // // println!("se => {:?}", se);
    // // assert_eq!(se + &3.0 , te)

    // // let te :crate::tensor::Tensor<u32>= crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    // // let se :crate::tensor::Tensor<u32> = crate::tensor::Tensor::new(vec![1,3],vec![1,2,3]);
    // // let xe :crate::tensor::Tensor<u32> = te + se;
    // // assert!(te == xe - se);
    // // assert!(se == te - xe);
}