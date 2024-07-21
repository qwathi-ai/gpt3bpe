#[cfg(test)]
mod tests {

    #[test]
    fn addition() {
        let t: crate::tensor::Tensor<i8> = crate::tensor::new(vec![1, 2, 2], vec![3,4,5,6]);
        assert_eq!(t, t.clone());

        let s = t.clone() - &3;
        assert_eq!(s.clone() + &3, t);

        let r = s.clone() + &t;
        assert_eq!(r - &s, t);
    }

    #[test]
    fn scale() {
        let data = vec![1, 2, 3];
        let shape = vec![1, data.len()];
        let scales = [3, 6, 9, 12];
        for scale in scales {
            let t: crate::tensor::Tensor<usize> =
                crate::tensor::new(shape.to_vec(), data.to_vec());
            assert_eq!(
                t.clone() * &scale,
                crate::tensor::new(
                    shape.to_vec(),
                    data.to_vec().iter().map(|datum| datum * scale).collect()
                )
            );
        }
    }

    #[test]
    fn zero_negation() {
        let t : crate::tensor::Tensor<i8>  = crate::tensor::new(vec![1,3], vec![1,2,3]);
        assert_eq!(t.clone() + &0, t.clone());
        assert_eq!(t.clone() + &(t.clone() * &-1), 0);
    }

    #[test]
    fn distribution() {
        let t: crate::tensor::Tensor<i8> = crate::tensor::new(vec![1,3], vec![1,2,3]);
        let s = t.clone();
        assert_eq!((t.clone() + &s) * &3, t * &3 + &(s * &3));
    }

    #[test]
    fn associative_addition() {
        let t: crate::tensor::Tensor<i8> = crate::tensor::new(vec![1,3], vec![1,2,3]);
        let s = t.clone();
        let r = s.clone();
        assert_eq!((t.clone() + &s) + &r, t + &(s + &r));
    }

    fn commutative_addition() {
        let t: crate::tensor::Tensor<i8> = crate::tensor::new(vec![1,3], vec![1,2,3]);
        let s = t.clone();
        assert_eq!(t.clone() + &s, s + &t);
    }
}
