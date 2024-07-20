mod unit;

use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Add;
// use std::ops::Mul;
use std::ops::Sub;

// use std::sync::{Arc, Mutex};
// use std::thread;
// const NUMBER_OF_THREADS: usize = 4;

#[derive(Debug)]
enum Rank {
    Vector, // 1st order
    Matrix, // 2nd order
    Tensor(usize),
}

impl PartialEq for Rank {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Tensor(l0), Self::Tensor(r0)) => l0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl Clone for Rank {
    fn clone(&self) -> Self {
        match self {
            Self::Vector => Self::Vector,
            Self::Matrix => Self::Matrix,
            Self::Tensor(arg0) => Self::Tensor(arg0.clone()),
        }
    }
}

#[derive(Debug)]
pub struct Tensor<T> {
    pub shape: Vec<usize>,
    rank: Rank,
    data: Vec<T>,
}

impl<T: std::clone::Clone> Clone for Tensor<T> {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.to_vec(),
            rank: self.rank.clone(),
            data: self.data.to_vec(),
        }
    }
}
impl<T: Clone + std::cmp::PartialEq> PartialEq<Tensor<T>> for Tensor<T> {
    fn eq(&self, other: &Tensor<T>) -> bool {
        if self.rank != other.rank {
            return false;
        }
        if self.shape != other.shape {
            return false;
        }
        if self.data != other.data {
            return false;
        }
        true
    }
    fn ne(&self, other: &Tensor<T>) -> bool {
        if self.rank != other.rank {
            return true;
        }
        if self.shape != other.shape {
            return true;
        }
        if self.data != other.data {
            return true;
        }
        false
    }
}

impl<T> Add<&T> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Add + std::ops::AddAssign
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: &T) -> Self::Output {
        for datum in self.data.iter_mut() {
            *datum += rhs.clone()
        }
        Self {
            rank: self.rank,
            data: self.data,
            shape: self.shape,
        }
    }
}

impl<T> Add<&Tensor<T>> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Add + std::ops::AddAssign ,
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: &Tensor<T>) -> Self::Output {
        if self.shape != rhs.shape {
            panic!(
                "Tensor {:#?} are not of the same shape {:#?}",
                self.shape, rhs.shape
            );
        }
        for (datum, value) in self.data.iter_mut().zip(rhs.data.iter()) {
            *datum += value.clone();
        }
        Self {
            rank: self.rank,
            data:self.data,
            shape: self.shape
        }
    }
}

impl<T> Sub<&T> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Sub + std::ops::SubAssign,
{
    type Output = Tensor<T>;

    fn sub(mut self, rhs: &T) -> Self::Output {
        for datum in self.data.iter_mut() {
            *datum -= rhs.clone()
        }
        Self {
            rank: self.rank,
            data: self.data,
            shape: self.shape,
        }
    }
}

impl<T> Sub<&Tensor<T>> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Sub + std::ops::SubAssign,
{
    type Output = Tensor<T>;

    fn sub(mut self, rhs: &Tensor<T>) -> Self::Output {
        if self.shape != rhs.shape {
            panic!(
                "Tensor {:#?} are not of the same shape {:#?}",
                self.shape, rhs.shape
            );
        }

        for (datum, value) in self.data.iter_mut().zip(rhs.data.iter()) {
            *datum -= value.clone();
        }
        Self {
            rank: self.rank,
            data:self.data,
            shape: self.shape
        }
    }
}

// impl<T> Mul<T> for Tensor<T>
// where
//     T: Clone + Display + Debug + std::ops::Add + std::ops::MulAssign
// {
//     type Output = Tensor<T>;

//     fn mul(mut self, rhs: T) -> Self::Output {
//         while let Some(datum) = self.data.iter_mut().next() {
//             *datum *= rhs.to_owned();
//         };
//         Self {
//             rank: self.rank,
//             data:self.data,
//             shape: self.shape
//         }
//     }
// }

// impl<T> Mul<Tensor<T>> for Tensor<T>
// where
//     T: Clone + Display + Debug + std::ops::Add
// {
//     type Output = Tensor<T>;

//     fn mul(self, _rhs: Tensor<T>) -> Self::Output {
//         todo!()
//     }
// }

impl<T> Tensor<T> {
    pub fn new(shape: Vec<usize>, data: Vec<T>) -> Self {
        println!("creating tensor.");
        if data.is_empty() {
            panic!("Tensor data cannot be empty.")
        }
        if shape.contains(&0) {
            panic!("Tensor shape cannot have a value of 0.")
        }
        if shape.len() <= 1 {
            panic!("Tensor shape cannot be equalivalent to scalar. Rather use rust number literal https://doc.rust-lang.org/beta/book/ch03-02-data-types.html")
        }
        let mut rank = Rank::Matrix;
        if shape.len() <= 2 && shape.contains(&1) {
            rank = Rank::Vector;
        }
        if shape.len() > 2 {
            let ranking = shape.iter().sum();
            rank = Rank::Tensor(ranking)
        }
        println!("new tensor with rank: {:?}", rank);
        Tensor { data, shape, rank }
    }
}