mod unit;
use rand::distributions::Uniform;
use rand::thread_rng;
use rand::Rng;
use std::fmt::Debug;

type Qubit<T> = [T; 2];

#[derive(Debug)]
pub(crate) struct Tensor<T, const N: usize> {
    data: Vec<Qubit<T>>,
}
impl<T, const N: usize> Tensor<T, N> {}
pub fn new<
    T: std::simd::SimdElement + rand::distributions::uniform::SampleUniform,
    const N: usize,
>(
    boundaries: &[T; 2],
    mut data: Vec<T>,
) -> Tensor<T, { N }> {
    if data.is_empty() {
        panic!("Tensor data cannot be empty.")
    }
    let qubits = data
        .iter_mut()
        .map(|datum| -> Qubit<T> {
            let random = {
                let mut generator = thread_rng();
                let distribution = Uniform::new_inclusive(&boundaries[0], &boundaries[1]);
                generator.sample(distribution)
            };
            [*datum, random]
        })
        .collect::<Vec<Qubit<T>>>();
    Tensor { data: qubits }
}

impl<T: std::simd::SimdElement, const N: usize> Clone for Tensor<T, { N }> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T: std::simd::SimdElement + std::cmp::PartialEq, const N: usize> PartialEq<T>
    for Tensor<T, { N }>
{
    fn eq(&self, rhs: &T) -> bool {
        let mut fact = true;
        let mut cursor = self.data.iter();

        while let Some(qubit) = cursor.next() {
            if qubit[0] != *rhs {
                fact = false;
                break;
            }
        }
        fact
    }

    fn ne(&self, rhs: &T) -> bool {
        !self.eq(rhs)
    }
}

impl<T, const N: usize> core::ops::Add<&T> for Tensor<T, { N }>
where
    T: std::simd::SimdElement + std::ops::Add<Output = T>,
{
    type Output = Tensor<T, { N }>;

    fn add(mut self, rhs: &T) -> Self::Output {
        for qubit in self.data.iter_mut() {
            *qubit = [qubit[0] + *rhs, qubit[1]];
        }

        Self { data: self.data }
    }
}

impl<T, const N: usize> core::ops::Sub<&T> for Tensor<T, { N }>
where
    T: std::simd::SimdElement + std::ops::Sub<Output = T>,
{
    type Output = Tensor<T, { N }>;

    fn sub(mut self, rhs: &T) -> Self::Output {
        for qubit in self.data.iter_mut() {
            *qubit = [qubit[0] - *rhs, qubit[1]];
        }
        Self { data: self.data }
    }
}

impl<T, const N: usize> core::ops::Mul<&T> for Tensor<T, { N }>
where
    T: std::simd::SimdElement + std::ops::Mul<Output = T>,
{
    type Output = Tensor<T, { N }>;

    fn mul(mut self, rhs: &T) -> Self::Output {
        for qubit in self.data.iter_mut() {
            *qubit = [qubit[0] * *rhs, qubit[1]];
        }
        Self { data: self.data }
    }
}

impl<T: std::simd::SimdElement + std::cmp::PartialEq, const N: usize> PartialEq<Tensor<T, { N }>>
    for Tensor<T, { N }>
{
    fn eq(&self, other: &Tensor<T, { N }>) -> bool {
        let mut fact = true;
        let mut cursor = self.data.iter().zip(other.data.iter());

        while let Some((qubit, rhs)) = cursor.next() {
            if qubit[0] != rhs[0] {
                fact = false;
                break;
            }
        }
        fact
    }
}

impl<T, const N: usize> core::ops::Add<&Tensor<T, { N }>> for Tensor<T, { N }>
where
    T: std::simd::SimdElement + std::ops::Add<Output = T>,
{
    type Output = Tensor<T, { N }>;

    fn add(mut self, other: &Tensor<T, { N }>) -> Self::Output {
        for (qubit, rhs) in self.data.iter_mut().zip(other.data.iter()) {
            *qubit = [qubit[0] + rhs[0], qubit[1] + rhs[1]];
        }
        Self { data: self.data }
    }
}

impl<T, const N: usize> core::ops::Sub<&Tensor<T, { N }>> for Tensor<T, { N }>
where
    T: std::simd::SimdElement + std::ops::Sub<Output = T>,
{
    type Output = Tensor<T, { N }>;

    fn sub(mut self, other: &Tensor<T, { N }>) -> Self::Output {
        for (qubit, rhs) in self.data.iter_mut().zip(other.data.iter()) {
            *qubit = [qubit[0] - rhs[0], qubit[1] - rhs[1]];
        }
        Self { data: self.data }
    }
}

impl<T, const N: usize> core::ops::Mul<&Qubit<T>> for Tensor<T, { N }>
where
    T: std::simd::SimdElement + std::ops::Mul<Output = T> + std::ops::Add<Output = T>,
{
    type Output = Tensor<T, { N }>;

    fn mul(mut self, other: &Qubit<T>) -> Self::Output {
        for mut qubit in self.data.iter_mut() {
            *qubit = [
                qubit[0] * other[0] + qubit[1] * other[1],
                qubit[0] * other[1] + qubit[1] * other[0],
            ];
        }
        Self { data: self.data }
    }
}
