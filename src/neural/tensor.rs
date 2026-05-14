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
const SIMD_LANES: usize = 75; // Consistent with the embedding db. 300/4 = 75
const SIMD_WIDTH: usize = 4;

#[derive(Debug)]
// ensure consistent with the embeddings module
struct Tensor<'a, T, const DIMENSIONS: usize, const LANES: usize = 75> where T: 'a {
    data: &'a [T; DIMENSIONS],
    vector: [std::simd::f32x4; LANES],
}

impl<'a, T: std::simd::SimdElement, const DIMENSIONS: usize, const LANES: usize = 75> Tensor<'a, T, DIMENSIONS, LANES> {
    pub fn new(data: &'a [T; DIMENSIONS]) -> Self {
       Tensor{
        data,
        // The `as_chunks` method splits the slice into chunks of 4 elements.
        // It returns the chunks and a remainder slice.
        // We expect the remainder to be empty, and panic if it's not.
        vector: {
            let (chunks, remainder) = data.as_chunks::<{SIMD_WIDTH}>();
            assert!(remainder.is_empty(), "Data length must be a multiple of 4");
            // This is a safe transmutation because `f32x4` has the same memory layout as `[f32; 4]`.
            *bytemuck::cast_ref(chunks)
        }
       }
    }
}


impl<'a, T: std::simd::SimdElement, const DIMENSIONS: usize, const LANES: usize = 75> Iterator for Tensor<'a, T, DIMENSIONS, LANES> {
    type Item = Tensor<T, DIMENSIONS, LANES>;
    fn next(&mut self) -> Option<Self::Item> {
        // return each simd element as a new tensor.
        self.vector.next()
    }
}

impl<T: std::simd::SimdElement, const DIMENSIONS: usize, const LANES: usize = 75> From<&Vec<T>> for Tensor<T, DIMENSIONS, LANES> {
    fn from(data: &Vec<T>) -> Self {
        Tensor::new(data.as_slice().try_into().unwrap())
    }
}

impl<T: std::simd::SimdElement, const DIMENSIONS: usize, const LANES: usize = 75> From<&[std::simd::f32x4; LANES]> for Tensor<T, DIMENSIONS, LANES> {
    fn from(vector: &[std::simd::f32x4; LANES]) -> Self {
        Tensor::new(vector.as_slice().try_into().unwrap())
    }
}


impl<T: std::simd::SimdElement, const DIMENSIONS: usize, const LANES: usize = 75> Clone for Tensor<T, DIMENSIONS, LANES> {
    fn clone(&self) -> Self { 
        Tensor::from(self.vector)
    }
}


impl<T: std::simd::SimdElement, const DIMENSIONS: usize, const LANES: usize = 75> To<&[T; DIMENSIONS]> for Tensor<T, DIMENSIONS, LANES> {
    fn from(&self) -> Self {
        self.vector.as_slice().try_into().unwrap()
    }
}

impl<T: std::simd::SimdElement + std::cmp::PartialEq, const DIMENSIONS: usize, const LANES: usize = 75> PartialEq<T> for Tensor<T, DIMENSIONS, LANES>
{
    fn eq(&self, rhs: &T) -> bool {
        // find an efficient way to ensure two arrays are equal.
        let mut fact = true;
        let mut cursor = self.vector.iter();
        while let Some(value) = cursor.next() {
            if value != simd::splat(*rhs)  {
                fact = false;
                break;
            };
        };
        fact
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::AddAssign<&'a T> + Copy, const DIMENSIONS: usize, const LANES: usize = 75>
    core::ops::Add<&T> for Tensor<T, DIMENSIONS, LANES>
{
    type Output = Tensor<T, DIMENSIONS, LANES>;

    fn add(mut self, rhs: &T) -> Self::Output {
        Tensor::from(self.vector.iter_mut().for_each(|val| *val += simd::splat(*rhs)))
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::SubAssign<&'a T> + Copy, const DIMENSIONS: usize, const LANES: usize = 75>
    core::ops::Sub<&T> for Tensor<T, DIMENSIONS, LANES>
{
    type Output = Tensor<T, DIMENSIONS, LANES>;

    fn sub(mut self, rhs: &T) -> Self::Output {
        Tensor::from(self.vector.iter_mut().for_each(|val| *val -= simd::splat(*rhs)))
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::MulAssign<&'a T> + Copy, const DIMENSIONS: usize, const LANES: usize = 75>
    core::ops::Mul<&T> for Tensor<T, DIMENSIONS, LANES>
{
    type Output = Tensor<T, DIMENSIONS, LANES>;

    fn mul(mut self, rhs: &T) -> Self::Output {
        Tensor::from(self.vector.iter_mut().for_each(|val| *val *= simd::splat(*rhs)))
    }
}

impl<T: std::simd::SimdElement + std::cmp::PartialEq, const DIMENSIONS: usize, const LANES: usize = 75> PartialEq for Tensor<T, DIMENSIONS, LANES>
{
    fn eq(&self, other: &Self) -> bool {
        // find an efficient way to ensure two arraysof type [simd::f32x4;75] are equal.
        let mut fact = true;
        let mut cursor = self.vector.iter().zip(other.vector.iter());
        while let Some((value, o)) = cursor.next() {
            if value != o {
                fact = false;
                break;
            };
        };
        fact
    }
}

// The following impls are commented out because they require more complex logic
// to handle the `vector` field and potential data ownership issues.
// They also need to ensure that the operations are element-wise on the `data` field
// and that the `vector` field is updated correctly or re-calculated.
// impl<T: std::simd::SimdElement + for<'a> std::ops::AddAssign<&'a T> + Copy, const D: usize>
//     core::ops::Add<&Tensor<T, D>> for Tensor<T, D>
// {
//     type Output = Tensor<T, D>;

//     fn add(mut self, other: &Tensor<T, D, W>) -> Self::Output {
//         let mut new_data = *self.data;
//         for (val, rhs) in new_data.iter_mut().zip(other.data.iter()) { *val += rhs; }
//         Tensor::new(&new_data)
//     }
// }
//
// impl<T: std::simd::SimdElement + for<'a> std::ops::SubAssign<&'a T> + Copy, const D: usize, const W: usize>
//     core::ops::Sub<&Tensor<T, D, W>> for Tensor<T, D, W>
// {
//     type Output = Tensor<T, D, W>;
//
//     fn sub(mut self, other: &Tensor<T, D, W>) -> Self::Output {
//         let mut new_data = *self.data;
//         for (val, rhs) in new_data.iter_mut().zip(other.data.iter()) { *val -= rhs; }
//         Tensor::new(&new_data)
//     }
// }
//
// impl<T: std::simd::SimdElement> core::ops::Mul<&Tensor<T, D, W>> for Tensor<T, D, W>
// {
//     type Output = Tensor<T, D, W>;
//
//     fn mul(self, other: &Tensor<T, D, W>) -> Self::Output {
//         // Tensor matrix multiplication simd optimised.
//         // convert from a [std::simd::f32x4;75] to a [f32;300]. use bytemuck if required.
//         unimplemented!()
//     }
// }
//
// impl<T: std::simd::SimdElement> core::ops::Div<&Tensor<T, D, W>> for Tensor<T, D, W>
// {
//     type Output = Tensor<T, D, W>;
//
//     fn div(self, other: &Tensor<T, D, W>) -> Self::Output {
//         // Tensor matrix multiplication simd optimised.
//         // convert from a [std::simd::f32x4;75] to a [f32;300]. use bytemuck if required.
//         unimplemented!()
//     }
// }

// macro_rules! tensor {
//     () => { 
//         todo!()
//      };
//     ($elem:expr; $n:expr) => { 
//         todo!()
//      };
//     ($($x:expr),+ $(,)?) => { 
//         todo!()
//      };
// }