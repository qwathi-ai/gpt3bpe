#[cfg(test)]
mod tests {

    #[test]
    fn addition() {
        let te: crate::tensor::Tensor<i16>= crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
        assert_eq!(te, te.clone());
        // let se = te.clone() - 3;
        // println!("se => {:?}", se);
        // assert!(true)
        // assert_eq!(se + 3 , te)
        // let xe = te + &se;
        // assert!(te == xe - &se);
        // assert!(se == te - &xe);
        // assert_eq!(S - 3, 3 - S);
    }

    // #[test]
    // fn scale() {
    //     let T = crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    //     assert_eq!(T * 3, 3 * T);
    //     assert_eq!(T * 3, 3 * T);
    // }

    // #[test]
    // fn zero_negation() {
    //     let T = crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    //     assert_eq!(T + 0, T);
    //     assert_eq!(T + (-T), 0);
    // }

    // #[test]
    // fn distribution() {
    //     let T = crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    //     let S = crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    //     assert_eq!((T + S) * 3, T*3 + S*3);
    // }

    // #[test]
    // fn associative_addition() {
    //     let T = crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    //     let S = crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    //     let R = crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    //     assert_eq!((T + S) + R, T + (S + R));      
    // }

    // fn commutative_addition() {
    //     let T = crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    //     let S = crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
    //     assert_eq!(T + S, S + T);      
    // }
}
