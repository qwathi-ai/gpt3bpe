#[cfg(test)]
mod tests {

    #[test]
    fn addition() {
        let T = crate::tensor::Tensor::new(vec![1,3], vec![1,2,3]);
        let S = crate::tensor::Tensor::new(vec![1,3],vec![1,2,3]);
        let X = T + S;
        assert!(T == X - S);
        assert!(S == T - X);
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
