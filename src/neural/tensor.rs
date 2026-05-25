//! # Tensors
//
// This module provides a `Tensor` struct for high-performance, multi-dimensional numerical computing
// using SIMD (Single Instruction, Multiple Data) acceleration. It is designed to be a foundational
// building block for neural network layers, offering an ergonomic API for common linear algebra
// operations.
use wide::f32x4;

/// The width of a single SIMD vector, corresponding to the number of `f32` elements it can hold.
pub(crate) const WIDTH: usize = 4;

/// A tensor for efficient SIMD operations on multi-dimensional data.
///
/// This struct holds an owned vector of the raw data and a SIMD-vectorized representation
/// to accelerate mathematical computations.
///
/// ### Introduction to Tensors
///
/// In mathematical and computational fields, *tensors* are a fundamental concept used to represent multi-dimensional data.
/// Tensors generalize scalars (0-dimensional data), vectors (1-dimensional data), and matrices (2-dimensional data) to n-dimensional arrays.
///
/// Key properties include:
/// - **Rank**: The number of dimensions. A scalar has rank 0, a vector rank 1, a matrix rank 2.
/// - **Shape**: The size of each dimension, e.g., a 2x3 matrix has a shape of `(2, 3)`.
/// - **Data Type**: The type of data stored, such as `f32` or `i64`.
///
/// # Generic Parameters
/// - `T`: The underlying scalar type of the tensor's data.
/// - `DIMENSIONS`: A `const` generic for the total number of elements in the tensor.
/// - `LANES`: A `const` generic for the number of SIMD vectors required to store the data.
///   This is typically `DIMENSIONS / WIDTH`.
#[derive(Debug)]
pub struct Tensor<T, const DIMENSIONS: usize, const TOKENS: usize> where T: Copy {
    /// The raw, contiguous data of the tensor stored in an owned vector.
    pub data: Vec<T>,
    /// A SIMD-vectorized representation of the data for accelerated computations.
    vector: [f32x4; TOKENS],
}

impl<const DIMENSIONS: usize, const TOKENS: usize> Tensor<f32, DIMENSIONS, TOKENS> {
    /// Creates a new `Tensor` from a vector of data.
    ///
    /// The constructor partitions the input data into SIMD vectors (`f32x4`). It will panic if the
    /// data length is not equal to `DIMENSIONS` or not an even multiple of the SIMD `WIDTH` (4).
    pub fn new(data: Vec<f32>) -> Self {
        assert_eq!(data.len(), DIMENSIONS, "Data length must match the tensor's DIMENSIONS");
        let (chunks, remainder) = data.as_chunks::<{WIDTH}>();
        assert!(remainder.is_empty(), "Data length must be a multiple of {WIDTH}");

        Tensor {
            vector: chunks
                .iter()
                .map(|array: &[f32; WIDTH]| f32x4::new(*array))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            data,
        }
    }
}

impl<const DIMENSIONS: usize, const TOKENS: usize> Tensor<f32, DIMENSIONS, TOKENS> {
    /// Returns an iterator over the SIMD vectors of the tensor.
    pub fn iter(&self) -> std::slice::Iter<'_, f32x4> {
        self.vector.iter()
    }

    /// Returns a mutable iterator over the SIMD vectors of the tensor.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, f32x4> {
        self.vector.iter_mut()
    }
}

impl<const DIMENSIONS: usize, const TOKENS: usize> From<&[f32; DIMENSIONS]> for Tensor<f32, DIMENSIONS, TOKENS> {
    /// Creates a `Tensor` from a reference to a fixed-size array by cloning the data.
    fn from(data: &[f32; DIMENSIONS]) -> Self {
        Tensor::new(data.to_vec())
    }
}

impl<T: Copy, const DIMENSIONS: usize, const TOKENS: usize> Clone for Tensor<T, DIMENSIONS, TOKENS> {
    /// Clones the `Tensor`.
    ///
    /// This creates a new `Tensor` with a cloned `data` vector and a copy of the `vector` data.
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            vector: self.vector,
        }
    }
}

impl<const DIMENSIONS: usize, const TOKENS: usize> AsRef<[f32; DIMENSIONS]> for Tensor<f32, DIMENSIONS, TOKENS> {
    /// Returns a reference to the underlying data array.
    fn as_ref(&self) -> &[f32; DIMENSIONS] {
        self.data.as_slice().try_into().expect("Tensor data length does not match DIMENSIONS")
    }
}

impl<const DIMENSIONS: usize, const TOKENS: usize> PartialEq<f32> for Tensor<f32, DIMENSIONS, TOKENS>
{
    /// Checks if all elements in the tensor are equal to a scalar value.
    fn eq(&self, rhs: &f32) -> bool {
        let splat = f32x4::splat(*rhs);
        self.vector.iter().all(|&value| value == splat)
    }
}

impl<T: Copy, const DIMENSIONS: usize, const TOKENS: usize> PartialEq<Tensor<T, DIMENSIONS, TOKENS>> for Tensor<T, DIMENSIONS, TOKENS>
{
    /// Checks if two tensors are equal by comparing their `vector` fields.
    fn eq(&self, other: &Self) -> bool {
        self.vector.iter().zip(other.vector.iter()).all(|(a, b)| a == b)
    }
}


/// Implements element-wise subtraction of a scalar from a `Tensor`.
///
/// ### Negation
/// For any tensor \(T\), there exists a tensor \(-T\) such that:
/// $$
/// T + (-T) = 0
/// $$
/// This is achieved by subtracting the tensor from a zero tensor or multiplying by -1.
impl<const DIMENSIONS: usize, const TOKENS: usize> core::ops::Sub<&f32> for Tensor<f32, DIMENSIONS, TOKENS>
{
    type Output = Self;

    /// Performs element-wise subtraction of the tensor by a scalar.
    fn sub(mut self, rhs: &f32) -> Self::Output {
        // Splat the scalar into a SIMD vector for parallel subtraction.
        let splat = f32x4::splat(*rhs);
        for val in self.vector.iter_mut() { *val -= splat; }
        self
    }
}

/// # Scalar Addition
/// Implements element-wise addition of a scalar to a `Tensor`.
///
/// ### Zero Tensor
/// There exists a zero tensor \(0\) such that for any tensor \(T\):
/// $$
/// T + 0 = T
/// $$
/// This implementation upholds that property when adding a scalar `0.0`.
impl<const DIMENSIONS: usize, const TOKENS: usize> core::ops::Add<&f32> for Tensor<f32, DIMENSIONS, TOKENS> {
    type Output = Self;

    /// Performs element-wise addition of the tensor by a scalar.
    fn add(mut self, rhs: &f32) -> Self::Output {
        // Splat the scalar into a SIMD vector for parallel addition.
        let splat = f32x4::splat(*rhs);
        for val in self.vector.iter_mut() { *val += splat; }
        self
    }
}

/// # Scalar Division
/// Implements element-wise division of a `Tensor` by a scalar.
impl<const DIMENSIONS: usize, const TOKENS: usize> core::ops::Div<&f32> for Tensor<f32, DIMENSIONS, TOKENS>
{
    type Output = Self;

    fn div(mut self, rhs: &f32) -> Self::Output {
        // Splat the scalar into a SIMD vector for parallel division.
        let rhs_splat = f32x4::splat(*rhs);
        for val in self.vector.iter_mut() { *val /= rhs_splat; }
        self
    }
}

/// # Scalar Multiplication
/// Implements element-wise multiplication of a `Tensor` by a scalar.
///
/// ### Mathematical Property
/// A tensor can be multiplied by a scalar, a common operation in linear algebra.
/// $$
/// (aT)_{i_1 i_2 \ldots i_n} = a \cdot T_{i_1 i_2 \ldots i_n}
/// $$
impl<const DIMENSIONS: usize, const TOKENS: usize> core::ops::Mul<&f32> for Tensor<f32, DIMENSIONS, TOKENS>
{
    type Output = Self;

    /// Performs element-wise multiplication of the tensor by a scalar.
    fn mul(mut self, rhs: &f32) -> Self::Output {
        // Splat the scalar into a SIMD vector for parallel multiplication.
        let rhs_splat = f32x4::splat(*rhs);
        for val in self.vector.iter_mut() { *val *= rhs_splat; }
        self
    }
}


// Tensor-to-Slice Operations
/// Implements element-wise addition between a `Tensor` and a raw slice.
impl<const DIMENSIONS: usize, const TOKENS: usize> core::ops::Add<&[f32;DIMENSIONS]>
    for Tensor<f32, DIMENSIONS, TOKENS>
{
    type Output = Self;

    /// Performs element-wise addition by converting the slice to a `Tensor` first.
    fn add(self, other: &[f32;DIMENSIONS]) -> Self::Output {
        self + &Tensor::from(other)
    }
}

/// Implements element-wise subtraction between a `Tensor` and a raw slice.
impl<const DIMENSIONS: usize, const TOKENS: usize> core::ops::Sub<&[f32;DIMENSIONS]>
    for Tensor<f32, DIMENSIONS, TOKENS>
{
    type Output = Self;

    /// Performs element-wise subtraction by converting the slice to a `Tensor` first.
    fn sub(self, other: &[f32;DIMENSIONS]) -> Self::Output {
        self - &Tensor::from(other)
    }
}

// Tensor-to-Tensor Operations
/// Implements element-wise addition between two `Tensor`s.
///
/// ### Additivity
/// Tensors can be added together component-wise, resulting in another tensor of the same rank and dimensions.
/// $$
/// (T + S)_{i_1 i_2 \ldots i_n} = T_{i_1 i_2 \ldots i_n} + S_{i_1 i_2 \ldots i_n}
/// $$
///
/// ### Associativity of Addition
/// $$
/// (T + S) + R = T + (S + R)
/// $$
///
/// ### Commutativity of Addition
/// $$
/// T + S = S + T
/// $$
impl<const DIMENSIONS: usize, const TOKENS: usize> core::ops::Add<&Tensor<f32, DIMENSIONS, TOKENS>>
    for Tensor<f32, DIMENSIONS, TOKENS>
{
    type Output = Self;

    /// Performs element-wise addition of two tensors.
    fn add(mut self, other: &Tensor<f32, DIMENSIONS, TOKENS>) -> Self::Output {
        for (lhs, rhs) in self.vector.iter_mut().zip(other.vector.iter()) {
            *lhs += rhs;
        }
        self
    }
}

/// Implements element-wise subtraction between two `Tensor`s.
impl<const DIMENSIONS: usize, const TOKENS: usize> core::ops::Sub<&Tensor<f32, DIMENSIONS, TOKENS>>
    for Tensor<f32, DIMENSIONS, TOKENS>
{
    type Output = Self;

    /// Performs element-wise subtraction of two tensors.
    fn sub(mut self, other: &Tensor<f32, DIMENSIONS, TOKENS>) -> Self::Output {
        for (lhs, rhs) in self.vector.iter_mut().zip(other.vector.iter()) {
            *lhs -= rhs;
        }
        self
    }
}

/// Implements the dot product (tensor contraction) between two `Tensor`s.
///
// ### Tensor Contraction
/// Tensors can be contracted, reducing the rank of the tensor by summing over one or more pairs of indices.
/// This implementation computes the dot product, which is a form of contraction.
/// $$ C = \sum_i T_i S_i $$
impl<'b, const DIMENSIONS: usize, const TOKENS: usize> core::ops::Mul<&'b Tensor<f32, DIMENSIONS, TOKENS>>
    for &Tensor<f32, DIMENSIONS, TOKENS>
{
    type Output = f32;

    /// Computes the dot product of two tensors using SIMD.
    /// This is a sum of the element-wise products.
    fn mul(self, other: &'b Tensor<f32, DIMENSIONS, TOKENS>) -> Self::Output {
        self.vector
            .iter()
            .zip(other.vector.iter())
            .map(|(a, b)| (*a * *b).reduce_add()) // Multiply SIMD vectors and sum the result
            .sum() // Sum the results from all chunks
    }
}