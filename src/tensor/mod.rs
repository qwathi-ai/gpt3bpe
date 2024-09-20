mod unit;

#[derive(Debug)]
struct Tensor<T> {
    shape: Vec<usize>, // 2,3
    data: Vec<T>
}

impl<T: std::simd::SimdElement> Clone for Tensor<T> {
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            shape: self.shape.clone()
        }
    }
}

// pub fn new<
//     T: std::simd::SimdElement + rand::distributions::uniform::SampleUniform,
//     const N: usize,
// >(
//     boundaries: &[T; 2],
//     mut data: Vec<T>,
// ) -> Tensor<T, { N }> {
//     if data.is_empty() {
//         panic!("Tensor data cannot be empty.")
//     }
//     let qubits = data
//         .iter_mut()
//         .map(|datum| -> Qubit<T> {
//             let random = {
//                 let mut generator = thread_rng();
//                 let distribution = Uniform::new_inclusive(&boundaries[0], &boundaries[1]);
//                 generator.sample(distribution)
//             };
//             [*datum, random]
//         })
//         .collect::<Vec<Qubit<T>>>();
//     Tensor { data: qubits }
// }



// impl Tensor<T> {
//     pub to_array<A>() ->  Vec<A> { // [[1,2,3][4,5,6]]
//         for shape

//     }
// }

impl<T: std::simd::SimdElement> From<&Vec<T>> for Tensor<T> {
    fn from(data: &Vec<T>) -> Self {
        Tensor {
            data: data.to_vec(),
            shape: vec![1, data.len()]
        }
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
            }
        }
        fact
    }

    fn ne(&self, rhs: &T) -> bool {
        !self.eq(rhs)
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::AddAssign<&'a T>>
    core::ops::Add<&T> for Tensor<T>
{
    type Output = Tensor<T>;

    fn add(mut self, rhs: &T) -> Self::Output {
        // for qubit in self.data.iter_mut() {
        //     *qubit = [qubit[0] + *rhs, qubit[1]];
        // }
        for value in self.data.iter_mut() {
            *value += rhs;
        }
        self
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::SubAssign<&'a T>>
    core::ops::Sub<&T> for Tensor<T>
{
    type Output = Tensor<T>;

    fn sub(mut self, rhs: &T) -> Self::Output {
        // for qubit in self.data.iter_mut() {
        //     *qubit = [qubit[0] - *rhs, qubit[1]];
        // }
        for value in self.data.iter_mut() {
            *value -= rhs;
        }
        self
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::MulAssign<&'a T>>
    core::ops::Mul<&T> for Tensor<T>
{
    type Output = Tensor<T>;

    fn mul(mut self, rhs: &T) -> Self::Output {
        // for qubit in self.data.iter_mut() {
        //     *qubit = [qubit[0] * *rhs, qubit[1]];
        // }
        for value in self.data.iter_mut() {
            *value *= rhs;
        }
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
        // for (qubit, rhs) in self.data.iter_mut().zip(other.data.iter()) {
        //     *qubit = [qubit[0] + rhs[0], qubit[1] + rhs[1]];
        // }
        for (value, rhs) in self.data.iter_mut().zip(other.data.iter()) {
            *value += rhs;
        }
        self
    }
}

impl<T: std::simd::SimdElement + for<'a> std::ops::SubAssign<&'a T>>
    core::ops::Sub<&Tensor<T>> for Tensor<T>
{
    type Output = Tensor<T>;

    fn sub(mut self, other: &Tensor<T>) -> Self::Output {
        // for (qubit, rhs) in self.data.iter_mut().zip(other.data.iter()) {
        //     *qubit = [qubit[0] - rhs[0], qubit[1] - rhs[1]];
        // }
        for (value, rhs) in self.data.iter_mut().zip(other.data.iter()) {
            *value -= rhs;
        }
        self
    }
}

impl<T: std::simd::SimdElement> core::ops::Mul<&Tensor<T>> for Tensor<T>
{
    type Output = Tensor<T>;

    fn mul(mut self, other: &Tensor<T>) -> Self::Output {
        // for mut qubit in self.data.iter_mut() {
        //     *qubit = [
        //         qubit[0] * other[0] + qubit[1] * other[1],
        //         qubit[0] * other[1] + qubit[1] * other[0],
        //     ];
        // }
        // let mut data = Vec::with_capacity(self.shape.len());
        // for (index,size) in self.shape.iter().rev().enumerate() {
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