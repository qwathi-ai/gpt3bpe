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
mod unit;

#[derive(Debug, Clone, PartialEq)]
enum Rank {
    Vector, // 1st order
    Matrix, // 2nd order
    Tensor(usize) // nth order
}

enum Layout {
    Contiguous,
    Column,
    Row
}

#[derive(Debug)]
struct Tensor<T, const N: usize> {
    pub rank: Rank,
    pub shape: Vec<usize>,
    index: Vec<isize>,
    chunk: [T;N],
    
    // pub layout: Layout,
    // axis: usize,
    // position: isize,
    // chunk: Vec<isize>,
    // data: Vec<T>
}

impl<T: std::simd::SimdElement, const N: usize> Iterator for Tensor<T, {N}> {
    type Item = Tensor<T, {N}>;

    fn next(&mut self) -> Option<Self::Item> {
        let dimensions = self.shape.len();
        if dimensions == 0 {
            if self.axis == 0 {
                self.axis = 1;
                // return Some(Tensor {
                //     shape: self.shape.clone(), 
                //     rank: self.rank.clone(),
                //     index: self.index.clone(),
                //     axis: self.axis,
                //     position: self.position,
                //     buffer: self.buffer.clone(),
                //     data: self.data.clone(),
                //     // lanes: self.lanes.clone(),
                // });
                return Some(self)
            }
        } else {
            loop {
                if self.index[self.axis] == self.shape[self.axis] {
                    if self.axis == 0 {
                        break;
                    }
                    self.position -= self.shape[self.axis] *
                                     self.buffer[self.axis];
                    self.index[self.axis] = 0;
                    self.axis -= 1;
                    self.index[self.axis] += 1;
                } else {
                    self.axis = dimensions - 1;
                    self.index[self.axis] += 1;
                    self.position += self.buffer[self.axis];
                    // return Some(Tensor {
                    //     shape: self.shape.clone(), 
                    //     rank: self.rank.clone(),
                    //     index: self.index.clone(),
                    //     axis: self.axis,
                    //     position: self.position,
                    //     buffer: self.buffer.clone(),
                    //     data: self.data.clone(),
                    //     // lanes: self.lanes.clone()
                    // });
                }
                self.position += self.buffer[self.axis];
            }
        }
        None
    }
}


impl<T: std::simd::SimdElement> Tensor<T> {
    /// Constructs a new, empty Tensor<T>.
    ///
    /// ## New Tensor
    pub fn new() -> Self {
        todo!()
    }

    /// Inserts an element at position index within the tensor, replacing the original value.
    /// 
    /// ## Insert
    pub fn set(&mut self, index: Vec<usize>, value: T) ->  Self {
        todo!()
    }
    pub fn get(&mut self, index: Vec<usize>) -> Self{
        todo!()
    }

    pub fn reshape(&mut self, shape: Vec<usize>) ->  Self {
        todo!()
    }

}

impl<T: std::simd::SimdElement> Clone for Tensor<T> {
    fn clone(&self) -> Self {
        // Self {
        //     index: self.index.clone(),
        //     axis: self.axis.clone(),
        //     position: self.position.clone(),
        //     buffer: self.buffer.clone(),
        //     rank: self.rank.clone(),
        //     data: self.data.clone(),
        //     shape: self.shape.clone(),
        //     // lanes: self.lanes.clone()
        // }
        todo!()
    }
}

impl<T: std::simd::SimdElement> From<&Vec<T>> for Tensor<T> {
    fn from(data: &Vec<T>) -> Self {
        // Tensor {
        //     rank: Rank::Vector,
        //     data: data.to_vec(),
        //     shape: vec![data.len().try_into().unwrap()],

        //     index: vec![0;vec![data.len()].len()],
        //     axis: 0,
        //     position: 0,
        //     buffer: vec![],
        // }
        todo!()
    }
}

impl<T: std::simd::SimdElement + std::cmp::PartialEq> PartialEq<T>
    for Tensor<T>
{
    fn eq(&self, rhs: &T) -> bool {
        let mut fact = true;
        let mut cursor = self.data.iter();
        while let Some(value) = cursor.next() {
            if value != rhs {
                fact = false;
                break;
            };
        };

        fact
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::AddAssign<&'a T>>
    core::ops::Add<&T> for Tensor<T>
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: &T) -> Self::Output {
        for value in self.data.iter_mut() {
            *value += rhs;
        };
        self
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::SubAssign<&'a T>>
    core::ops::Sub<&T> for Tensor<T>
{
    type Output = Tensor<T>;

    fn sub(mut self, rhs: &T) -> Self::Output {
        for value in self.data.iter_mut() {
            *value -= rhs;
        };
        self
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::MulAssign<&'a T>>
    core::ops::Mul<&T> for Tensor<T>
{
    type Output = Tensor<T>;

    fn mul(mut self, rhs: &T) -> Self::Output {
        for value in self.data.iter_mut() {
            *value *= rhs;
        };
        self
    }
}

impl<T: std::simd::SimdElement + std::cmp::PartialEq> PartialEq<Tensor<T>>
    for Tensor<T>
{
    fn eq(&self, other: &Tensor<T>) -> bool {
        self.data == other.data
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::AddAssign<&'a T>>
    core::ops::Add<&Tensor<T>> for Tensor<T>
{
    type Output = Tensor<T>;

    fn add(mut self, other: &Tensor<T>) -> Self::Output {
        if other.rank == self.rank && other.shape == self.shape {
            for (value, rhs) in self.data.iter_mut().zip(other.data.iter()) {
                *value += rhs;
            };
        }
        let largest_other = other.shape.iter().max().expect("Tensor has no shape.");
        let smallest_other = other.shape.iter().min().expect("Tensor has no shape.");


        let largest_self = self.shape.iter().max().expect("Tensor has no shape.");
        let smallest_self = self.shape.iter().min().expect("Tensor has no shape.");

        if self.shape.contains(largest) {

        }
        self
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::SubAssign<&'a T>>
    core::ops::Sub<&Tensor<T>> for Tensor<T>
{
    type Output = Tensor<T>;

    fn sub(mut self, other: &Tensor<T>) -> Self::Output {
        for (value, rhs) in self.data.iter_mut().zip(other.data.iter()) {
            *value -= rhs;
        };
        self
    }
}

impl<T: std::simd::SimdElement> core::ops::Mul<&Tensor<T>> for Tensor<T>
{
    type Output = Tensor<T>;

    fn mul(self, other: &Tensor<T>) -> Self::Output {
        if self.data.is_empty() || other.data.is_empty() {
            panic!("One of the matrices is empty");
        }
        // for mut qubit in self.data.iter_mut() {
        //     *qubit = [
        //         qubit[0] * other[0] + qubit[1] * other[1],
        //         qubit[0] * other[1] + qubit[1] * other[0],
        //     ];
        // }
        
        // if &self.rank == &Rank::Matrix && &other.rank == &Rank::Vector {
        //     assert_eq!(self.shape[1], other.shape[0]);
        // }
        // if &self.rank == &Rank::Vector && &other.rank == &Rank::Matrix {
        //     assert_eq!(self.shape[0], other.shape[0]);

        // }
        // if &self.rank == &Rank::Vector && &other.rank == &Rank::Vector {
        //     assert_eq!(self.data.size(), other.shape.size());
            
        // }
        // if &self.rank == &Rank::Matrix && &other.rank == &Rank::Matrix {
        //     assert_eq!(self.shape[1], rhs.shape[0]);
            
        // }
        // let size = self.shape.pop().unwrap();
        // let mut data = Vec::with_capacity(size);

        // for (index,dim) in self.shape.iter().enumerate() {
        //     data.push(Vec::with_capacity(*size));
        //     for i in self.data.chunks_exact(*size) {
                
        //     }
        // }
        todo!()
    }
}

// fn tensor_elementwise_multiply(a: &Vec<Vec<Vec<i32>>>, b: &Vec<Vec<Vec<i32>>>) -> Vec<Vec<Vec<i32>>> {
//     let mut result = a.clone();

//     for i in 0..a.len() {
//         for j in 0..a[0].len() {
//             for k in 0..a[0][0].len() {
//                 result[i][j][k] = a[i][j][k] * b[i][j][k];
//             }
//         }
//     }

//     result
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