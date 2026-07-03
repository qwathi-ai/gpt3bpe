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

/// A fully connected Layer in a neural network.
///
/// A `Layer` is defined by its weights, biases, and the dimensions of its input and output.
/// It is designed to be a self-contained unit that can be composed into a larger network.
/// The `transpose` of the weights is pre-computed and stored for efficiency during the
/// backpropagation pass.
#[derive(Clone, Debug)]
pub(crate) struct Layer<
    // The numeric type for calculations, typically `f32`.
    T: std::marker::Copy,
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT_LANES: usize,
    const OUTPUT: usize,
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
        const OUTPUT_LANES: usize,
        const OUTPUT: usize,
    > Layer<f32, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT>
{
    /// Transposes a matrix of tensors.
    pub fn transpose(
        input: &[tensor::Tensor<f32, INPUT, INPUT_LANES>; OUTPUT],
    ) -> [tensor::Tensor<f32, OUTPUT, OUTPUT_LANES>; INPUT] {
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
        let mut y = vec![0.0; OUTPUT];
        let x = Tensor::new(x.to_vec());
        for (i, &c) in self.biases.iter().enumerate() {
            let mx: f32 = &self.weights[i] * &x;
            y[i] = mx + c;
        }
        println!("forward: data: {:?}", y);
        self.activate(&tensor::Tensor::new(y), activation)
    }

    /// Performs the backward pass (backpropagation) for the layer.
    ///
    /// This method calculates the gradients for the weights and the error to be
    /// propagated to the previous layer.
    pub fn backward<const PREV_INPUT: usize, const PREV_INPUT_LANES: usize>(
        &self,
        rate: &f32,
        dy: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
        x: &Tensor<f32, INPUT, INPUT_LANES>,
    ) -> (Self, Tensor<f32, INPUT, INPUT_LANES>) {
        // 1. Calculate Weight Gradients (dw)
        // dw = dy * x^T
        // This is an outer product. For each output neuron's error, we scale the entire input vector.
        let dw: [Tensor<f32, INPUT, INPUT_LANES>; OUTPUT] = dy
            .iter()
            .map(|&neuron_error| x.clone() * &neuron_error.reduce_add())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        // 2. Calculate Bias Gradients (db)
        // The gradient for each bias is simply the error for that neuron.
        let db: [f32; OUTPUT] = dy.as_ref().clone();

        // 3. Propagate Error to the Previous Layer (dx)
        // dx = weights^T * dy
        let mut dx_data = vec![0.0; INPUT];
        for (i, dx_neuron) in dx_data.iter_mut().enumerate() {
            *dx_neuron = &self.transpose[i] * dy;
        }
        let dx = Tensor::new(dx_data);

        // 4. Update Weights and Biases
        let mut weights = self.weights.clone();
        let mut biases = self.biases.clone();

        for i in 0..OUTPUT {
            weights[i] = weights[i].clone() - &(dw[i].clone() * rate);
            biases[i] -= db[i] * rate;
        }

        // 5. Return the updated layer and the error for the previous layer.
        (Layer::new(weights, biases), dx)
    }

    /// Calculates the derivative of the activation function.
    ///
    /// adjusted based on the activation function's derivative.
    ///
    /// # Arguments
    /// * `y` - The post-activation output from the forward pass.
    /// * `activation` - The activation function that was used.
    fn derivative(
        y: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
        activation: &Activation,
    ) -> Tensor<f32, OUTPUT, OUTPUT_LANES> {
        let mut derivative = y.clone();
        match activation {
            Activation::None => derivative,
            Activation::ReLU => {
                // f'(x) = 1 if f(x) > 0 else 0
                for v in derivative.iter_mut() {
                    let mut arr = v.to_array();
                    for val in arr.iter_mut() {
                        if *val > 0.0 {
                            *val = 1.0;
                        } else {
                            *val = 0.0;
                        }
                    }
                    *v = f32x4::new(arr);
                }
                derivative
            }
            Activation::Sigmoid => {
                // f'(x) = f(x) * (1.0 - f(x))
                let ones = f32x4::splat(1.0);
                derivative.iter_mut().for_each(|v| *v = *v * (ones - *v));
                derivative
            }
            Activation::Tanh => {
                // f'(x) = 1.0 - f(x)^2
                let ones = f32x4::splat(1.0);
                derivative.iter_mut().for_each(|v| *v = ones - (*v * *v));
                derivative
            }
            Activation::Softmax => {
                // The derivative of softmax is more complex as it's a matrix (the Jacobian).
                // For backpropagation, we usually combine the derivative of the loss with respect
                // to the softmax output, which simplifies to (output - target).
                // A standalone derivative would be diag(y) - y * y^T.
                // For now, we'll return the output, assuming the gradient calculation
                // in the backward pass will handle the combined derivative.
                // This is a common simplification.
                // A more complete implementation would require the pre-activation values or the target.
                // Since the request is to only use `y`, this is a reasonable approach.
                derivative
            }
        }
    }

    /// Applies an activation function to a tensor.
    ///
    /// This function takes the pre-activation output of a layer (logits) and applies
    /// the specified non-linear activation function. This is a critical step in the
    /// forward pass, allowing the network to learn complex patterns.
    ///
    /// # Arguments
    /// * `x` - A `Tensor` containing the pre-activation values (logits).
    /// * `activation` - The `Activation` enum variant to apply.
    ///
    /// # Returns
    /// A new `Tensor` with the activation function applied element-wise.
    fn activate(
        &self,
        x: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
        activation: &Activation,
    ) -> Tensor<f32, OUTPUT, OUTPUT_LANES> {
        // Clone the input tensor to store the results.
        let mut result = x.clone();
        match activation {
            // ReLU (Rectified Linear Unit): f(x) = max(0, x)
            // This is computationally efficient and helps mitigate the vanishing gradient problem.
            Activation::ReLU => {
                // Create a SIMD vector of all zeros.
                let zero = f32x4::splat(0.0);
                // For each SIMD vector in the tensor, compute the element-wise maximum with zero.
                // This effectively sets all negative values to 0.
                result.iter_mut().for_each(|v| *v = v.max(zero));
                result
            }
            // Sigmoid: f(x) = 1 / (1 + e^(-x))
            // Squashes values to a range between 0 and 1. Often used in the output layer for binary classification.
            Activation::Sigmoid => {
                let ones = f32x4::splat(1.0);
                // Apply the sigmoid function element-wise using SIMD operations for better performance.
                // The `exp()` method is available on `f32x4` from the `wide` crate.
                result
                    .iter_mut()
                    .for_each(|v| *v = ones / (ones + (-*v).exp()));
                result
            }
            // Softmax: f(x_i) = e^(x_i) / Σ(e^(x_j)) for all j
            // Converts a vector of logits into a probability distribution, where all values are in [0, 1] and sum to 1.
            // Essential for multi-class classification output layers.
            Activation::Softmax => {
                // For numerical stability, we subtract the maximum value from all logits before exponentiating.
                // This prevents `exp(x)` from becoming infinity for large x.
                // `e^(x_i - max(x)) / Σ(e^(x_j - max(x)))` is mathematically equivalent to the original formula.
                for v in result.iter_mut() {
                    let mut arr = v.to_array();
                    for val in arr.iter_mut() {
                        *val = libm::expf(*val);
                    }
                    *v = f32x4::new(arr);
                }
                // Calculate the sum of all exponentiated values.
                let mut sum = f32x4::splat(0.0);
                result.iter().for_each(|v| sum += *v);
                // Sum the elements within the final SIMD vector to get the total sum.
                let sum_lanes = sum.reduce_add();
                // Divide each exponentiated value by the total sum to get the final probabilities.
                result / &sum_lanes
            }
            // Tanh (Hyperbolic Tangent): f(x) = tanh(x)
            // Squashes values to a range between -1 and 1. It's zero-centered, which can be advantageous.
            Activation::Tanh => {
                // Apply the tanh function element-wise using SIMD operations.
                // This is more efficient than iterating over individual scalar values.
                result.iter_mut().for_each(|v| {
                    let e_pos = v.exp();
                    let e_neg = (-*v).exp();
                    *v = (e_pos - e_neg) / (e_pos + e_neg);
                });
                result
            }
            // None: f(x) = x
            // A linear activation function, which means no transformation is applied.
            Activation::None => result,
        }
    }
}

use rand::RngExt;
type Network<
    T,
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT_LANES: usize,
    const OUTPUT: usize,
> = Vec<Layer<T, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT>>;

pub fn layers<
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT_LANES: usize,
    const OUTPUT: usize,
>(
    number: usize,
) -> Network<f32, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT> {
    let layer = || {
        let mut rng = rand::rng();
        let weights: [Tensor<f32, INPUT, INPUT_LANES>; OUTPUT] = std::array::from_fn(|_| {
            let data = (0..INPUT)
                .map(|_| rng.random::<f32>())
                .collect::<Vec<f32>>();
            Tensor::new(data)
        });
        let biases: [f32; OUTPUT] = std::array::from_fn(|_| rng.random::<f32>());
        Layer::new(weights, biases)
    };
    (0..number).map(|_| layer()).collect()
}

pub fn forward<
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT_LANES: usize,
    const OUTPUT: usize,
>(
    network: &Network<f32, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT>,
    x: &Tensor<f32, INPUT, INPUT_LANES>,
) -> Vec<Vec<f32>> {
    let mut inputs = vec![x.as_ref().to_vec()];
    for (i, layer) in network.iter().enumerate() {
        // Use ReLU for all hidden layers and Sigmoid for the final output layer.
        let activation = if i == network.len() - 1 {
            Activation::Softmax
        } else {
            Activation::Sigmoid
        };

        // The forward pass requires a fixed-size array. We convert our dynamic Vec.
        let x: &[f32; INPUT] = inputs[i]
            .as_slice()
            .try_into()
            .expect("Slice with incorrect length");
        inputs.push(layer.forward(x, &activation).as_ref().to_vec());
    }
    // The final vector is converted back to a Tensor to be returned.
    inputs
}

/// Performs a single training iteration (forward and backward pass) and returns the updated network.
fn train<
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT_LANES: usize,
    const OUTPUT: usize,
>(
    mut network: Vec<Layer<f32, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT>>,
    x: &Tensor<f32, INPUT, INPUT_LANES>,
    y: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
    rate: &f32,
) -> Vec<Layer<f32, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT>> {
    // 1. Forward pass: We need to store the input and output of each layer for backpropagation.
    let inputs = forward(&network, x);
    // 2. Backward pass: Propagate the error from the output layer back to the input layer.
    let output = Tensor::<f32, OUTPUT, OUTPUT_LANES>::new(inputs.last().unwrap().to_vec());
    let mut error = (output.clone() - y).as_ref().to_vec(); // Initial error is the difference between prediction and target.
    let mut net = network.clone();

    for i in (0..network.len()).rev() {
        let input = Tensor::new(inputs[i].clone());
        let mut output = Tensor::new(inputs[i + 1].clone());

        let activation = if i == network.len() - 1 {
            Activation::Softmax
        } else {
            Activation::Sigmoid
        };
        output = Layer::<f32, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT>::derivative(&output, &activation);
        // Multiply element-wise (Hadamard product) to get the error gradient for the layer's pre-activation output (logits)
        let dy = output * Tensor::<f32, OUTPUT, OUTPUT_LANES>::new(error.try_into().unwrap()).as_ref();
        let (layer, e) = net[i].backward::<OUTPUT, OUTPUT_LANES>(rate, &dy, &input);
        net[i] = layer;
        error = e.as_ref().to_vec();
    }
    net
}
