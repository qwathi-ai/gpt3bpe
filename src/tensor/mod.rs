mod unit;

use std::fmt::Debug;
use std::fmt::Display;
use std::simd::Simd;

// use std::sync::{Arc, Mutex};
// use std::thread;
// const NUMBER_OF_THREADS: usize = 4;


#[derive(Debug)]
enum Rank {
    Vector, // 1st order
    Matrix, // 2nd order
    // Tensor(usize),
}

// impl PartialEq for Rank {
//     fn eq(&self, other: &Self) -> bool {
//         match (self, other) {
//             (Self::Tensor(l0), Self::Tensor(r0)) => l0 == r0,
//             _ => core::mem::discriminant(self) == core::mem::discriminant(other),
//         }
//     }
// }

impl Clone for Rank {
    fn clone(&self) -> Self {
        match self {
            Self::Vector => Self::Vector,
            Self::Matrix => Self::Matrix,
            // Self::Tensor(arg0) => Self::Tensor(arg0.clone()),
        }
    }
}

#[derive(Debug)]

pub struct Tensor<T, N>{
    pub shape: Vec<usize>,
    rank: Rank,
    data: Simd<T, N>,
}

impl<T: std::clone::Clone> Clone for Tensor<T> {
    fn clone(&self) -> Self {
        Self {
            shape: self.shape.clone(),
            rank: self.rank.clone(),
            data: self.data.clone(),
        }
    }
}

impl<T: std::clone::Clone + std::cmp::PartialEq> PartialEq<T> for Tensor<T> {
    fn eq(&self, other: &T) -> bool {
        let mut real = vec![];
        for qubit in self.data.windows(self.shape[1]) {
            real.append(quibit[0])
        }
        real == vec![other.clone(); self.data.len()]
    }
}

impl<T: std::clone::Clone + std::cmp::PartialEq> PartialEq<Tensor<T>> for Tensor<T> {
    fn eq(&self, other: &Tensor<T>) -> bool {
        if self.rank != other.rank {
            return false;
        }
        if self.window != other.window {
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
}

impl<T> core::ops::Add<&T> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Add + std::ops::AddAssign,
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: &T) -> Self::Output {
        for datum in self.data.iter_mut().windows(self.window) {
            *datum[0] += rhs.clone()
        }
        Self {
            rank: self.rank,
            data: self.data,
            shape: self.shape,
        }
    }
}

impl<T> core::ops::Add<&Tensor<T>> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Add + std::ops::AddAssign,
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: &Tensor<T>) -> Self::Output {
        if self.shape != rhs.shape {
            panic!(
                "Tensor {:#?} are not of the same shape {:#?}",
                self.shape, rhs.shape
            );
        }
        for (x, y) in self.data.iter_mut().zip(rhs.data.iter()).windows(self.shape[0]) {
            for (datum, value) in x.iter_mut().zip(y) {
                *datum += value.clone();
            }
        }
        Self {
            rank: self.rank,
            data: self.data,
            shape: self.shape,
        }
    }
}

impl<T> core::ops::Sub<&T> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Sub + std::ops::SubAssign,
{
    type Output = Tensor<T>;

    fn sub(mut self, rhs: &T) -> Self::Output {
        for datum in self.data.iter_mut().windows(self.window) {
            *datum[0] -= rhs.clone()
        }
        Self {
            window: self.window,
            rank: self.rank,
            data: self.data,
            shape: self.shape,
        }
    }
}

impl<T> core::ops::Sub<&Tensor<T>> for Tensor<T>
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
        for (x, y) in self.data.iter_mut().zip(rhs.data.iter()) {
            for (datum, value) in x.iter_mut().zip(y) {
                *datum -= value.clone();
            }
        }
        Self {
            rank: self.rank,
            data: self.data,
            shape: self.shape,
        }
    }
}

impl<T> core::ops::Mul<&T> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Mul + std::ops::MulAssign,
{
    type Output = Tensor<T>;

    fn mul(mut self, rhs: &T) -> Self::Output {
        for datum in self.data.iter_mut() {
            datum.insert(0,datum[0] *= rhs.clone()) 
        }
        Self {
            rank: self.rank,
            data: self.data,
            shape: self.shape,
        }
    }
}

// impl<T> Mul<&Tensor<T>> for Tensor<T>
// where
//     T: Clone + Display + Debug + std::ops::Add
// {
//     type Output = Tensor<T>;

//     fn mul(self, rhs: &Tensor<T>) -> Self::Output {
//         fn kronecker () {

//         }
        
//     }
// }

// impl<T> Tensor<T> {
//     fn basis(&self) -> Vec<T>{
//         let mut outputs = vec![];
//         for [lhs, rhs] in self.shape.windows(2) {

//         }

//     }
// }

pub fn new<T>(shape: Vec<usize>, data: Vec<T>) -> Tensor<T> {
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
    Tensor { data: Simd::from(data), shape, rank }
}
