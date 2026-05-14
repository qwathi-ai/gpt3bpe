//! # Tensors
//!
//! ### Introduction to Tensor Properties
//!
//! In mathematical and computational fields, *tensors* are a fundamental concept used to represent multi-dimensional data. 
//! Tensors generalize scalars (0-dimensional data), vectors (1-dimensional data), and matrices (2-dimensional data) to n-dimensional arrays, where the number of dimensions is known as the *rank* of the tensor. 
//! They are essential in a wide range of applications, from physics and engineering to machine learning and data science, where complex data relationships must be modeled, manipulated, and analyzed effectively.
//! 
//! The key properties of tensors include:
//!
//! - **Rank**: The number of dimensions in a tensor. For instance, a scalar has rank 0, a vector rank 1, a matrix rank 2, and so on. Higher-rank tensors (such as rank-3 or rank-4 tensors) are commonly used to represent volumetric data, sequences of images, or complex data relationships.
//! - **Shape**: The size of each dimension in a tensor, often described as a tuple. For example, a 2x3 matrix has a shape of `(2, 3)`, while a 3-dimensional tensor with dimensions 4, 5, and 6 would have a shape of `(4, 5, 6)`.
//! - **Data Type**: The type of data stored in a tensor’s elements, such as integers, floating-point numbers, or complex numbers. This is important in computational contexts where precision and memory usage are key considerations.
//! - **Operations**: Tensors support a range of operations, including element-wise addition and multiplication, as well as more complex transformations like matrix multiplication, reshaping, and slicing. Many tensor operations generalize linear algebra operations to higher dimensions, enabling flexible and powerful manipulations of multi-dimensional data.
//! - **Indexing**: Accessing and modifying elements within tensors is done using indices along each dimension. Tensors support sophisticated indexing schemes, allowing for efficient slicing, broadcasting, and rearrangement of data.
//! These properties make tensors highly adaptable and versatile in managing large-scale, structured data. While different software environments and libraries provide varying levels of support for tensor operations, the foundational properties remain consistent across implementations.
//!
//! ### Properties of a Tensor for Linear Operations
//! 
//! 1. **Additivity**: Tensors can be added together component-wise, resulting in another tensor of the same order and dimensions.
//!     $$
//!     (T + S)_{i_1 i_2 \ldots i_n} = T_{i_1 i_2 \ldots i_n} + S_{i_1 i_2 \ldots i_n}
//!     $$
//!
//! 2. **Scalar Multiplication**: A tensor can be multiplied by a scalar, scaling each component of the tensor by that scalar.
//!     $$
//!     (aT)_{i_1 i_2 \ldots i_n} = a \cdot T_{i_1 i_2 \ldots i_n}
//!     $$
//! 
//! 3. **Zero Tensor**: There exists a zero tensor \(0\) such that for any tensor \(T\), 
//!     $$
//!     T + 0 = T
//!     $$
//! 
//! 4. **Negation**: For any tensor \(T\), there exists a tensor \(-T\) such that 
//!     $$
//!     T + (-T) = 0
//!     $$
//! 
//! 5. **Distributivity**: Scalar multiplication distributes over tensor addition.
//!     $$
//!     a(T + S) = aT + aS
//!     $$
//! 
//! 6. **Associativity of Addition**: Tensor addition is associative.
//!     $$
//!     (T + S) + R = T + (S + R)
//!     $$
//!
//! 7. **Commutativity of Addition**: Tensor addition is commutative.
//!    $$
//!    T + S = S + T
//!    $$
//!
//! ### Properties of a Tensor for Multilinear Operations
//! 1. **Multilinearity**: A tensor is a multilinear map, meaning it is linear in each of its arguments when the others are held fixed.
//!    $$
//!    T(a \mathbf{v} + b \mathbf{w}, \mathbf{u}) = a T(\mathbf{v}, \mathbf{u}) + b T(\mathbf{w}, \mathbf{u})
//!    $$
//! 
//!    $$
//!    T(\mathbf{u}, a \mathbf{v} + b \mathbf{w}) = a T(\mathbf{u}, \mathbf{v}) + b T(\mathbf{u}, \mathbf{w})
//!    $$
//! 
//! 2. **Tensor Contraction**: Tensors can be contracted, reducing the order of the tensor by summing over one or more pairs of indices.
//!    $$
//!    C_{ik} = \sum_j T_{ijk}
//!    $$
//! 
//! 3. **Tensor Product**: The tensor product of two tensors \(T\) and \(S\) results in a new tensor whose order is the sum of the orders of \(T\) and \(S\).
//!    $$
//!    (T \otimes S)_{i_1 i_2 \ldots i_m j_1 j_2 \ldots j_n} = T_{i_1 i_2 \ldots i_m} S_{j_1 j_2 \ldots j_n}
//!    $$
//! 
//! 4. **Symmetry and Antisymmetry**: Tensors can be symmetric or antisymmetric with respect to certain indices.
//!    - Symmetric tensor:
//!      $$
//!      T_{ijk} = T_{jik}
//!      $$
//!    - Antisymmetric tensor:
//!      $$
//!      T_{ijk} = -T_{jik}
//!      $$
//! 
//! 5. **Outer Product**: The outer product of two vectors results in a matrix (a second-order tensor).
//!    $$
//!    (a \otimes b)_{ij} = a_i b_j
//!    $$
//! 
//! 6. **Duality**: There is a duality between tensors and multilinear maps, where a tensor can be seen as a multilinear map from a set of vector spaces to the underlying field.
//! 
//! These properties ensure that tensors can be manipulated in a consistent manner, enabling a wide range of applications in fields such as physics, engineering, and computer science.

pub(crate)const SIMD_LANES: usize = 75; // Consistent with the embedding db. 300/4 = 75
pub(crate) const SIMD_WIDTH: usize = 4;

#[derive(Debug)]
/// A tensor struct designed for efficient SIMD operations on multi-dimensional data.
///
/// This struct holds a reference to the raw data and a SIMD-vectorized representation
/// to accelerate mathematical computations.
pub struct Tensor<'a, T, const DIMENSIONS: usize, const LANES: usize> where T: 'a + Copy {
    pub data: &'a [T; DIMENSIONS],
    vector: [std::simd::f32x4; LANES],
}

impl<'a, const DIMENSIONS: usize, const LANES: usize> Tensor<'a, f32, DIMENSIONS, LANES> {
    /// Creates a new `Tensor` from a slice of data.
    ///
    /// The constructor partitions the input data into SIMD vectors (`f32x4`).
    /// It will panic if the data length is not an even multiple of the SIMD width (4).
    pub fn new(data: &'a [f32; DIMENSIONS]) -> Self {
       Tensor{
        data,
        vector: {
            let (chunks, remainder) = data.as_chunks::<{SIMD_WIDTH}>();
            assert!(remainder.is_empty(), "Data length must be a multiple of {}", SIMD_WIDTH);
            chunks.iter().map(|array: &[f32; 4]| std::simd::f32x4::from_array(*array)).collect::<Vec<_>>().try_into().unwrap()
        }
       }
    }
}

impl<'a, T: Copy, const DIMENSIONS: usize, const LANES: usize> Tensor<'a, T, DIMENSIONS, LANES> {
    /// Returns an iterator over the SIMD vectors of the tensor.
    pub fn iter(&self) -> std::slice::Iter<'_, std::simd::f32x4> {
        self.vector.iter()
    }

    /// Returns a mutable iterator over the SIMD vectors of the tensor.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, std::simd::f32x4> {
        self.vector.iter_mut()
    }
}

impl<'a, const DIMENSIONS: usize, const LANES: usize> From<&'a [f32; DIMENSIONS]> for Tensor<'a, f32, DIMENSIONS, LANES> {
    /// Creates a `Tensor` from a reference to a fixed-size array.
    fn from(data: &'a [f32; DIMENSIONS]) -> Self {
        Tensor::new(data)
    }
}

impl<'a, T: Copy, const DIMENSIONS: usize, const LANES: usize> Clone for Tensor<'a, T, DIMENSIONS, LANES> {
    /// Clones the `Tensor`.
    ///
    /// This creates a new `Tensor` with the same `data` reference and a copy of the `vector` data.
    fn clone(&self) -> Self { 
        Self {
            data: self.data,
            vector: self.vector,
        }
    }
}

impl<'a, T: Copy, const DIMENSIONS: usize, const LANES: usize> Copy for Tensor<'a, T, DIMENSIONS, LANES> {}

impl<'a, const DIMENSIONS: usize, const LANES: usize> AsRef<[f32; DIMENSIONS]> for Tensor<'a, f32, DIMENSIONS, LANES> {
    /// Returns a reference to the underlying data array.
    fn as_ref(&self) -> &[f32; DIMENSIONS] {
        self.data
    }
}

impl<'a, const DIMENSIONS: usize, const LANES: usize> PartialEq<f32> for Tensor<'a, f32, DIMENSIONS, LANES>
{
    /// Checks if all elements in the tensor are equal to a scalar value.
    fn eq(&self, rhs: &f32) -> bool {
        let splat = std::simd::f32x4::splat(*rhs);
        self.vector.iter().all(|&value| value == splat)
    }
}

impl<'a, T: Copy, const DIMENSIONS: usize, const LANES: usize> PartialEq<Tensor<'a, T, DIMENSIONS, LANES>> for Tensor<'a, T, DIMENSIONS, LANES>
{
    /// Checks if two tensors are equal by comparing their `vector` fields.
    fn eq(&self, other: &Self) -> bool {
        self.vector.iter().zip(other.vector.iter()).all(|(a, b)| a == b)
    }
}


/// # Scalar Subtraction (Negation)
impl<'a, const DIMENSIONS: usize, const LANES: usize> core::ops::Sub<&f32> for Tensor<'a, f32, DIMENSIONS, LANES>
{
    type Output = Self;

    /// Performs element-wise subtraction of the tensor by a scalar.
    fn sub(mut self, rhs: &f32) -> Self::Output {
        let splat = std::simd::f32x4::splat(*rhs);
        for val in self.vector.iter_mut() { *val -= splat; }
        self
    }
}

/// # Scalar Addition
impl<'a, const DIMENSIONS: usize, const LANES: usize> core::ops::Add<&f32> for Tensor<'a, f32, DIMENSIONS, LANES> {
    type Output = Self;

    /// Performs element-wise addition of the tensor by a scalar.
    fn add(mut self, rhs: &f32) -> Self::Output {
        let splat = std::simd::f32x4::splat(*rhs);
        for val in self.vector.iter_mut() { *val += splat; }
        self
    }
}

/// # Scalar Division
impl<'a, const DIMENSIONS: usize, const LANES: usize> core::ops::Div<&f32> for Tensor<'a, f32, DIMENSIONS, LANES>
{
    type Output = Self;

    /// Performs element-wise multiplication of the tensor by a scalar.
    fn div(mut self, rhs: &f32) -> Self::Output {
        let rhs_splat = std::simd::f32x4::splat(*rhs);
        for val in self.vector.iter_mut() { *val /= rhs_splat; }
        self
    }
}

/// # Scalar Multiplication
impl<'a, const DIMENSIONS: usize, const LANES: usize> core::ops::Mul<&f32> for Tensor<'a, f32, DIMENSIONS, LANES>
{
    type Output = Self;

    /// Performs element-wise multiplication of the tensor by a scalar.
    fn mul(mut self, rhs: &f32) -> Self::Output {
        let rhs_splat = std::simd::f32x4::splat(*rhs);
        for val in self.vector.iter_mut() { *val *= rhs_splat; }
        self
    }
}


// Tensor-to-Slice Operations
/// # Element-wise Addition
impl<'a, const DIMENSIONS: usize, const LANES: usize> core::ops::Add<&'a [f32;DIMENSIONS]>
    for Tensor<'a, f32, DIMENSIONS, LANES>
{
    type Output = Self;

    /// Performs element-wise addition of two tensors.
    fn add(self, other: &'a [f32;DIMENSIONS]) -> Self::Output {
        self + &Tensor::from(other)
    }
}

/// # Element-wise Subtraction (Negation)
impl<'a, const DIMENSIONS: usize, const LANES: usize> core::ops::Sub<&'a [f32;DIMENSIONS]>
    for Tensor<'a, f32, DIMENSIONS, LANES>
{
    type Output = Self;

    /// Performs element-wise addition of two tensors.
    fn sub(self, other: &'a [f32;DIMENSIONS]) -> Self::Output {
        self - &Tensor::from(other)
    }
}

// Tensor-to-Tensor Operations
/// # Element-wise Addition
impl<'a, const DIMENSIONS: usize, const LANES: usize> core::ops::Add<&Tensor<'a, f32, DIMENSIONS, LANES>>
    for Tensor<'a, f32, DIMENSIONS, LANES>
{
    type Output = Self;

    /// Performs element-wise addition of two tensors.
    fn add(mut self, other: &Tensor<'a, f32, DIMENSIONS, LANES>) -> Self::Output {
        for (lhs, rhs) in self.vector.iter_mut().zip(other.vector.iter()) {
            *lhs += rhs;
        }
        self
    }
}

/// # Element-wise Subtraction (Negation)
impl<'a, const DIMENSIONS: usize, const LANES: usize> core::ops::Sub<&Tensor<'a, f32, DIMENSIONS, LANES>>
    for Tensor<'a, f32, DIMENSIONS, LANES>
{
    type Output = Self;

    /// Performs element-wise subtraction of two tensors.
    fn sub(mut self, other: &Tensor<'a, f32, DIMENSIONS, LANES>) -> Self::Output {
        for (lhs, rhs) in self.vector.iter_mut().zip(other.vector.iter()) {
            *lhs -= rhs;
        }
        self
    }
}

// /// # Tensor Contraction (dot product)
// /// Tensors can be contracted, reducing the order of the tensor by summing over one or more pairs of indices.
// /// This implementation computes the dot product, a form of contraction.
// /// $$ C_{ik} = \sum_j T_{ijk} $$
// impl<'a, 'b, const DIMENSIONS: usize, const LANES: usize> core::ops::Mul<&'b Tensor<'a, f32, DIMENSIONS, LANES>>
//     for &'a Tensor<'a, f32, DIMENSIONS, LANES>
// {
//     type Output = f32;

//     /// Computes the dot product of two tensors using SIMD.
//     /// This is a sum of the element-wise products.
//     fn mul(self, other: &'b Tensor<'a, f32, DIMENSIONS, LANES>) -> Self::Output {
//         self.vector
//             .iter()
//             .zip(other.vector.iter())
//             .map(|(a, b)| (*a * *b).reduce_sum()) // Multiply SIMD vectors and sum the result
//             .sum() // Sum the results from all chunks
//     }
// }

// /// # Element-wise Division
// /// Performs element-wise division (Hadamard division) between two tensors of the same shape.
// /// This is not a standard linear algebra operation but is common in numerical computing.
// impl<'a, const DIMENSIONS: usize, const LANES: usize> core::ops::Div<&Tensor<'a, f32, DIMENSIONS, LANES>>
//     for Tensor<'a, f32, DIMENSIONS, LANES>
// {
//     type Output = Self;

//     /// Performs element-wise division of two tensors.
//     fn div(mut self, other: &Tensor<'a, f32, DIMENSIONS, LANES>) -> Self::Output {
//         for (lhs, rhs) in self.vector.iter_mut().zip(other.vector.iter()) {
//             *lhs /= *rhs;
//         }
//         self
//     }
// }
