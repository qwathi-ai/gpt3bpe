use rand::Rng;

fn random_number() -> i8 {
    rand::thread_rng().gen_range(-100..100)
}

mod tests {

    #[test]
    fn addition() {
        let random = super::random_number();
        let t: crate::tensor::Tensor<i8> = crate::tensor::Tensor::from(&vec![3, 4, 5, 6]);
        assert_eq!(t, t.clone());
        let s = t.clone() - &random;
        assert_eq!(s.clone() + &random, t);
        let r = s.clone() + &t;
        assert_eq!(r - &s, t);
    }

    #[test]
    fn scale() {
        let data = [1, 2, 3, 4, 5];
        let t: crate::tensor::Tensor<i8> = crate::tensor::Tensor::from(&data.to_vec());
        for scale in [3, 6, 9, 12] {
            let s = data.to_vec().iter().map(|datum| datum * scale).collect();
            let scaled = crate::tensor::Tensor::from(&s);
            assert_eq!( t.clone() * &scale, scaled);
        }
    }

    #[test]
    fn zero_negation() {
        let t: crate::tensor::Tensor<i8> = crate::tensor::Tensor::from(&vec![1, 2, 3]);
        assert_eq!(t.clone() + &0, t.clone());
        assert_eq!(t.clone() + &(t.clone() * &-1), 0);
    }

    #[test]
    fn distribution() {
        let t: crate::tensor::Tensor<i8> = crate::tensor::Tensor::from(&vec![1, 2, 3]);
        let s = t.clone();
        assert_eq!((t.clone() + &s) * &3, t * &3 + &(s * &3));
    }

    #[test]
    fn associative_addition() {
        let t: crate::tensor::Tensor<i8> = crate::tensor::Tensor::from(&vec![1, 2, 3]);
        let s = t.clone();
        let r = s.clone();
        assert_eq!((t.clone() + &s) + &r, t + &(s + &r));
    }

    #[test]
    fn commutative_addition() {
        let t: crate::tensor::Tensor<i8> = crate::tensor::Tensor::from(&vec![1, 2, 3]);
        let s = t.clone();
        assert_eq!(t.clone() + &s, s + &t);
    }

    // // Wish me luck.
    // #[test]
    // fn multilinearity() {
    //     const A: i8 = 5;
    //     let v = crate::tensor::new::<i8, 4>([-1,1], vec![1, 2, 3]);
    //     const B: i8 = 7;
    //     let w = v.clone();
    //     let u = v.clone();
    //     assert_eq!(
    //         u.clone() * &((v.clone() * &A) + &(w.clone() * &B)),
    //         (v * &u) * &A + &(u * &w) * &B
    //     );
    // }

    // // Wish me luck.
    // #[test]
    // fn multilinearity() {
    //     const A: i8 = 5;
    //     let v = crate::tensor::new::<i8>(vec![1, 3], vec![1, 2, 3]);
    //     const B: i8 = 7;
    //     let w = v.clone();
    //     let u = v.clone();
    //     assert_eq!(
    //         u.clone() * &((v.clone() * &A) + &(w.clone() * &B)),
    //         (v * &u) * &A + &(u * &w) * &B
    //     );
    // }
}
