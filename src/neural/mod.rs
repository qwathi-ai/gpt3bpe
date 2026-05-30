//! A module for building and training individual neural network layers.
//!
//! This module provides the `Layer` struct, which represents a single, fully-connected
//! layer in a neural network. It includes methods for forward propagation (`forward`),
//! backpropagation (`backward`), and a training step (`train`). This design allows for
//! the flexible construction of multi-layered networks externally.
mod tensor;
mod unit;
use crate::neural::tensor::Tensor;
use wide::f32x4;

/// Represents the activation function to be applied to the output of a neural network Layer.
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

/// A tuple holding the gradients for a `Layer`'s weights and the error for the previous layer.
///
/// This is returned by the `backward` pass and contains:
/// 1.  Weight gradients (`dw`): The adjustments needed for each weight in the layer.
/// 2.  Propagated error (`dx`): The error signal to be passed back to the preceding layer.
type Gradients<T, const INPUT: usize, const INPUT_LANES: usize, const OUTPUT: usize> = (
    // Gradients for the layer's weights.
    [Tensor<T, INPUT, INPUT_LANES>; OUTPUT],
    // Error to be propagated to the previous layer.
    Vec<T>,
);

/// A fully connected Layer in a neural network.
///
/// A `Layer` is defined by its weights, biases, and the dimensions of its input and output.
/// It is designed to be a self-contained unit that can be composed into a larger network.
/// The `transpose` of the weights is pre-computed and stored for efficiency during the
/// backpropagation pass.
#[derive(Clone, Debug)]
pub (crate) struct Layer<
    // The numeric type for calculations, typically `f32`.
    T: std::marker::Copy,
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT: usize,
    const OUTPUT_LANES: usize,
> {
    pub weights: [tensor::Tensor<T, INPUT, INPUT_LANES>; OUTPUT],
    /// Pre-computed transpose of the weights matrix, used for efficient backpropagation.
    pub transpose: [tensor::Tensor<T, OUTPUT, OUTPUT_LANES>; INPUT],
    pub biases: [T; OUTPUT],
}

/// Implementation of a `Layer` using `f32` for its computations.
impl<
        const INPUT: usize,
        const INPUT_LANES: usize,
        const OUTPUT: usize,
        const OUTPUT_LANES: usize,
    > Layer<f32, INPUT, INPUT_LANES, OUTPUT, OUTPUT_LANES>
where
    for<'b> &'b Tensor<f32, INPUT, INPUT_LANES>:
        std::ops::Mul<&'b Tensor<f32, INPUT, INPUT_LANES>, Output = f32>,
    for<'b> &'b Tensor<f32, OUTPUT, OUTPUT_LANES>:
        std::ops::Mul<&'b Tensor<f32, OUTPUT, OUTPUT_LANES>, Output = f32>,
{
    /// Transposes a matrix of tensors.
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

    /// Creates a new `Layer` with the given weights and biases.
    ///
    /// The transposed weight matrix is automatically computed and stored.
    pub fn new(
        weights: [tensor::Tensor<f32, INPUT, INPUT_LANES>; OUTPUT],
        biases: [f32; OUTPUT],
    ) -> Self {
        Layer {
            transpose: Self::transpose(&weights),
            weights,
            biases,
        }
    }

    /// Performs the forward pass for the layer.
    ///
    /// This computes `output = activation(weights * input + biases)`.
    ///
    /// # Arguments
    /// * `x` - The input vector for the layer.
    /// * `activation` - The activation function to apply to the output.
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

    /// Performs the backward pass (backpropagation) for the layer.
    ///
    /// This method calculates the gradients for the weights and the error to be
    /// propagated to the previous layer.
    ///
    /// # Arguments
    /// * `dx` - The error gradient from the next layer (or the loss function).
    /// * `input` - The original input vector that was fed into the `forward` pass.
    ///
    /// # Returns
    /// A `Gradients` tuple containing:
    /// * The weight gradients (`dw`).
    /// * The error to propagate to the previous layer (`dx_prev`).
    pub fn backward(
        &self,
        dx: Vec<f32>,
        input: Vec<f32>
    ) -> Gradients<f32, INPUT, INPUT_LANES, OUTPUT> {
        // Calculate weight gradients (dw) using the outer product of the incoming error (dx)
        // and the original input to this layer.
        let prev = Tensor::new(input);
        let dw: [Tensor<f32, INPUT, INPUT_LANES>; OUTPUT] = dx
            .iter()
            .map(|&d| prev.clone() * &d)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let dx = Tensor::new(dx);

        // Calculate the error to be propagated to the previous layer (dx_prev).
        // This is done by multiplying the incoming error (dx) by the transposed weights (W^T).
        let mut gradient: [f32; INPUT] = [0.0; INPUT];
        for (i, item) in gradient.iter_mut().enumerate() {
            *item = &self.transpose[i] * &dx;
        }

        // Return the weight gradients and the propagated error.
        // Note: The bias gradients are equivalent to `dx`, so they are handled in the `train` method.
        (dw, gradient.to_vec())
    }

    /// Performs a single training step and returns a new, updated `Layer`.
    ///
    /// This method calculates gradients via backpropagation and updates the layer's
    /// weights and biases according to the learning rate. It returns a new `Layer`
    /// instance, leaving the original unchanged (functional approach).
    fn train(&self, rate: f32, x: &Tensor<f32, INPUT, INPUT_LANES>, dy: &Tensor<f32, OUTPUT, OUTPUT_LANES>) -> Self {
        // The incoming error `dy` also serves as the bias gradient `db`.
        let (dw, db) = self.backward(dy.as_ref().to_vec(), x.as_ref().to_vec());

        let mut new_weights = self.weights.clone();
        for i in 0..OUTPUT {
            new_weights[i] = new_weights[i].clone() - &(dw[i].clone() * &rate);
        }
        let mut new_biases = self.biases.clone();
        for i in 0..OUTPUT {
            new_biases[i] -= db[i] * rate;
        }

        Layer::new(new_weights, new_biases)
    }

    /// Calculates the derivative of the activation function.
    ///
    /// This is a crucial step in backpropagation, where the error gradient `dx` is
    /// adjusted based on the activation function's derivative.
    ///
    /// # Arguments
    /// * `dx` - The incoming error gradient.
    /// * `output` - The post-activation output from the forward pass.
    /// * `activation` - The activation function that was used.
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
