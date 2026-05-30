mod optimizer;
mod tensor;
mod unit;
use crate::neural::tensor::Tensor;
use wide::f32x4;

/// Represents the activation function to be applied to the output of a neural network Perceptron.
#[derive(Debug, Clone)]
pub enum Activation {
    /// Rectified Linear Unit: `f(x) = max(0, x)`.
    ReLU,
    /// Sigmoid: `f(x) = 1 / (1 + e^(-x))`.
    Sigmoid,
    /// Softmax: Converts a vector of values into a probability distribution.
    Softmax,
    /// Hyperbolic Tangent: `f(x) = tanh(x)`.
    Tanh,
    /// No activation function is applied.
    None,
}

/// A tuple holding the gradients for a Perceptron's weights and biases.
type Gradients<T, const INPUT: usize, const INPUT_LANES: usize, const OUTPUT: usize> = (
    // Weight gradients
    [Tensor<T, INPUT, INPUT_LANES>; OUTPUT],
    // Bias gradients
    Vec<T>,
);

/// A fully connected Perceptron in a neural network.
#[derive(Clone, Debug)]
pub (crate) struct Perceptron<
    T: std::marker::Copy,
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT: usize,
    const OUTPUT_LANES: usize,
> {
    pub weights: [tensor::Tensor<T, INPUT, INPUT_LANES>; OUTPUT],
    pub transpose: [tensor::Tensor<T, OUTPUT, OUTPUT_LANES>; INPUT],
    pub biases: [T; OUTPUT],
}

impl<
        const INPUT: usize,
        const INPUT_LANES: usize,
        const OUTPUT: usize,
        const OUTPUT_LANES: usize,
    > Perceptron<f32, INPUT, INPUT_LANES, OUTPUT, OUTPUT_LANES>
where
    for<'b> &'b Tensor<f32, INPUT, INPUT_LANES>:
        std::ops::Mul<&'b Tensor<f32, INPUT, INPUT_LANES>, Output = f32>,
    for<'b> &'b Tensor<f32, OUTPUT, OUTPUT_LANES>:
        std::ops::Mul<&'b Tensor<f32, OUTPUT, OUTPUT_LANES>, Output = f32>,
{
    pub fn transpose(input: &[tensor::Tensor<f32, INPUT, INPUT_LANES>; OUTPUT]) -> [tensor::Tensor<f32, OUTPUT, OUTPUT_LANES>; INPUT] {
        let mut transpose_data: Vec<Vec<f32>> = vec![vec![0.0; OUTPUT]; INPUT];

        for (row_idx, input_tensor) in input.iter().enumerate() {
            for (col_idx, &value) in input_tensor.as_ref().iter().enumerate() {
                transpose_data[col_idx][row_idx] = value;
            }
        }
        
        transpose_data
            .into_iter()
            .map(|data| Tensor::new(data))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    pub fn new(
        weights: [tensor::Tensor<f32, INPUT, INPUT_LANES>; OUTPUT],
        biases: [f32; OUTPUT],
    ) -> Self {
        Perceptron {
            transpose: Self::transpose(&weights),
            weights,
            biases,
        }
    }

    /// Performs the forward pass of the Perceptron (Modified to accept &self instead of consuming self).
    pub fn forward(
        &self,
        x: &[f32; INPUT],
        activation: &Activation,
    ) -> Tensor<f32, OUTPUT, OUTPUT_LANES> {
        let mut data = vec![0.0; OUTPUT];
        let x = Tensor::new(x.to_vec());
        for (i, &c) in self.biases.iter().enumerate() {
            let mx: f32 = &self.weights[i] * &x;
            data[i] = mx + c;
        }
        self.activate(&tensor::Tensor::new(data), activation)
    }

    /// Step-by-step implementation of your custom backpropagation pipeline
    pub fn backward(
        &self,
        dx: Vec<f32>,
        input: Vec<f32>
    ) -> Gradients<f32, INPUT, INPUT_LANES, OUTPUT> {
        // 4. Calculate bias changes required for training from adjusted signals
        let prev = Tensor::new(input);

        // 4. Calculate weight changes using outer product of adjusted error and previous input
        let dw: [Tensor<f32, INPUT, INPUT_LANES>; OUTPUT] = dx
            .iter()
            .map(|&d| prev.clone() * &d)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let dx = Tensor::new(dx);

        // 5. Calculate the difference / error required for the previous Perceptron (dx * W)
        let mut gradient: [f32; INPUT] = [0.0; INPUT];
        for (i, item) in gradient.iter_mut().enumerate() {
            // Fix: Multiply the column of the transposed weights matrix by the adjusted incoming error
            *item = &self.transpose[i] * &dx;
        }


        // 6. Return weight changes, bias changes, and difference in signals for previous Perceptron
        (dw, gradient.to_vec())
    }

    fn train(&self, rate: f32, x: &Tensor<f32, INPUT, INPUT_LANES>, dy: &Tensor<f32, OUTPUT, OUTPUT_LANES>) -> Self {
        // 3. Backward pass to get weight and bias gradients
        let (dw, db) = self.backward(dy.as_ref().to_vec(), x.as_ref().to_vec());

        // 4. Update weights and biases using calculated gradients and learning rate
        let mut new_weights = self.weights.clone();
        for i in 0..OUTPUT {
            new_weights[i] = new_weights[i].clone() - &(dw[i].clone() * &rate);
        }
        let mut new_biases = self.biases.clone();
        for i in 0..OUTPUT {
            new_biases[i] -= db[i] * rate;
        }

        // 5. Return a new Perceptron instance with updated weights and biases
        Perceptron::new(new_weights, new_biases)
    }

    /// Calculates the activation derivative using the post-activation output state
    fn derive(
        dx: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
        output: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
        activation: Activation,
    ) -> Tensor<f32, OUTPUT, OUTPUT_LANES> {
        let mut adjusted_dx = dx.clone();
        match activation {
            Activation::None => adjusted_dx,
            Activation::ReLU => {
                // f'(x) = 1 if f(x) > 0 else 0
                for (dx_vec, out_vec) in adjusted_dx.iter_mut().zip(output.iter()) {
                    let mut dx_arr = dx_vec.to_array();
                    let out_arr = out_vec.to_array();
                    for (d, &o) in dx_arr.iter_mut().zip(out_arr.iter()) {
                        if o <= 0.0 {
                            *d = 0.0;
                        }
                    }
                    *dx_vec = f32x4::new(dx_arr);
                }
                adjusted_dx
            }
            Activation::Sigmoid => {
                // f'(x) = f(x) * (1.0 - f(x))
                for (dx_vec, out_vec) in adjusted_dx.iter_mut().zip(output.iter()) {
                    let mut dx_arr = dx_vec.to_array();
                    let out_arr = out_vec.to_array();
                    for (d, &o) in dx_arr.iter_mut().zip(out_arr.iter()) {
                        *d = *d * o * (1.0 - o);
                    }
                    *dx_vec = f32x4::new(dx_arr);
                }
                adjusted_dx
            }
            Activation::Tanh => {
                // f'(x) = 1.0 - f(x)^2
                for (dx_vec, out_vec) in adjusted_dx.iter_mut().zip(output.iter()) {
                    let mut dx_arr = dx_vec.to_array();
                    let out_arr = out_vec.to_array();
                    for (d, &o) in dx_arr.iter_mut().zip(out_arr.iter()) {
                        *d = *d * (1.0 - o * o);
                    }
                    *dx_vec = f32x4::new(dx_arr);
                }
                adjusted_dx
            }
            Activation::Softmax => {
                // Standalone Softmax Jacobian Vector Product: out * (dx - sum(dx * out))
                // Leverage your high-performance SIMD dot product to sum up dx * output!
                let sum_dx_out = dx * output; 
                for (dx_vec, out_vec) in adjusted_dx.iter_mut().zip(output.iter()) {
                    let mut dx_arr = dx_vec.to_array();
                    let out_arr = out_vec.to_array();
                    for (d, &o) in dx_arr.iter_mut().zip(out_arr.iter()) {
                        *d = o * (*d - sum_dx_out);
                    }
                    *dx_vec = f32x4::new(dx_arr);
                }
                adjusted_dx
            }
        }
    }

    /// Applies an activation function to a tensor.
    fn activate(
        &self,
        x: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
        activation: &Activation,
    ) -> Tensor<f32, OUTPUT, OUTPUT_LANES> {
        let mut result = x.clone();
        match activation {
            Activation::ReLU => {
                let zero = f32x4::splat(0.0);
                result.iter_mut().for_each(|v| *v = v.max(zero));
                result
            }
            Activation::Sigmoid => {
                for v in result.iter_mut() {
                    let mut arr = v.to_array();
                    for val in arr.iter_mut() {
                        *val = 1.0 / (1.0 + libm::expf(-*val));
                    }
                    *v = f32x4::new(arr);
                }
                result
            }
            Activation::Softmax => {
                let max_val = result
                    .iter()
                    .map(|v| {
                        let arr = v.to_array();
                        arr.into_iter()
                            .max_by(|a, b| a.partial_cmp(b).unwrap())
                            .unwrap()
                    })
                    .reduce(f32::max)
                    .unwrap_or(0.0);

                let mut sum = f32x4::splat(0.0);
                for v in result.iter_mut() {
                    let mut arr = v.to_array();
                    for val in arr.iter_mut() {
                        *val = libm::expf(*val - max_val);
                    }
                    *v = f32x4::new(arr);
                    sum += *v;
                }
                let sum_lanes = sum.reduce_add();
                result / &sum_lanes
            }
            Activation::Tanh => {
                for v in result.iter_mut() {
                    let mut arr = v.to_array();
                    for val in arr.iter_mut() {
                        *val = libm::tanhf(*val);
                    }
                    *v = f32x4::new(arr);
                }
                result
            },
            Activation::None => result,
        }
    }
}
