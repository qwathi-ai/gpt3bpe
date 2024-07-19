mod unit;

use std::fmt::Debug;
use std::fmt::Display;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Sub;

// use std::sync::{Arc, Mutex};
// use std::thread;
// const NUMBER_OF_THREADS: usize = 4;

enum Rank {
    Vector, // 1st order
    Matrix, // 2nd order
    Tensor(u8)
}
pub struct Tensor<T>
where
    T: Add<T>,
{
    pub shape: Vec<u8>,
    rank: Rank,
    data: Vec<T>,
}

impl<T: std::ops::Add> Tensor<T> {
    pub fn new(shape: Vec<u8>, data: Vec<T>) -> Self {
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
        if shape.len() <=2 && shape.contains(&1) {
            rank = Rank::Vector;
        }
        if shape.len() > 2 {
            let ranking: u8 = shape.iter().sum();
            rank = Rank::Tensor(ranking)
        }
        Tensor {
            data,
            shape,
            rank,
        }
    }
}

impl<T : std::ops::Add> PartialEq<Tensor<T>> for Tensor<T>  where &T: PartialEq<T>{

    fn eq(&self, other: &Tensor<T>) -> bool {
        if self.shape != other.shape || self.rank != other.rank {
            return false
        }
        while let Some((s, o)) = self.data.iter().zip(other.data).next() {
            if s != o {
                return false
            }
        }
        true
    }
}

impl<T : std::ops::Add + std::cmp::PartialEq> PartialEq<T> for Tensor<T> {

    fn eq(&self, other: &T) -> bool {
        while let Some(s) = self.data.iter().next() {
            if s != other {
                return false
            }
        }
        true
    }
}

impl<T> Add<T> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Add + std::ops::AddAssign ,
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: T) -> Self::Output {
        while let Some(datum) = self.data.iter_mut().next() {
            *datum += rhs.to_owned();
        };
        Self {
            rank: self.rank,
            data:self.data,
            shape: self.shape
        }
    }
}


impl<T> Add<Tensor<T>> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Add + std::ops::AddAssign ,
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: Tensor<T>) -> Self::Output {
        if self.shape != rhs.shape {
            panic!(
                "Tensor {:#?} are not of the same shape {:#?}",
                self.shape, rhs.shape
            );
        }

        while let Some((datum, value)) = self.data.iter_mut().zip(rhs.data.to_owned()).next()  {
            *datum += value;
        }
        Self {
            rank: self.rank,
            data:self.data,
            shape: self.shape
        }
    }
}

impl<T> Sub<T> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Add + std::ops::AddAssign + std::ops::SubAssign,
{
    type Output = Tensor<T>;

    fn sub(mut self, rhs: T) -> Self::Output {
        while let Some(datum) = self.data.iter_mut().next() {
            *datum -= rhs.to_owned();
        };
        Self {
            rank: self.rank,
            data:self.data,
            shape: self.shape
        }
    }
}

impl<T> Sub<Tensor<T>> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Add + std::ops::AddAssign + std::ops::SubAssign,
{
    type Output = Tensor<T>;

    fn sub(mut self, rhs: Tensor<T>) -> Self::Output {
        if self.shape != rhs.shape {
            panic!(
                "Tensor {:#?} are not of the same shape {:#?}",
                self.shape, rhs.shape
            );
        }

        while let Some((datum, value)) = self.data.iter_mut().zip(rhs.data.to_owned()).next()  {
            *datum -= value;
        }
        Self {
            rank: self.rank,
            data:self.data,
            shape: self.shape
        }
    }
}

impl<T> Mul<T> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Add + std::ops::MulAssign
{
    type Output = Tensor<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        while let Some(datum) = self.data.iter_mut().next() {
            *datum *= rhs.to_owned();
        };
        Self {
            rank: self.rank,
            data:self.data,
            shape: self.shape
        }
    }
}

impl<T> Mul<Tensor<T>> for Tensor<T>
where
    T: Clone + Display + Debug + std::ops::Add
{
    type Output = Tensor<T>;

    fn mul(mut self, rhs: Tensor<T>) -> Self::Output {
        todo!()
    }
}
