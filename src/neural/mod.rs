//! A module for building and training individual neural network layers.
//!
//! This module provides the `Layer` struct, which represents a single, fully-connected
//! layer in a neural network. It includes methods for forward propagation (`forward`),
//! backpropagation (`backward`), and a training step (`train`). This design allows for
//! the flexible construction of multi-layered networks externally.
pub(crate) mod tensor;
pub(crate) mod unit;
use tensor::Tensor;

/// Represents the activation function to be applied to the output of a neural network Layer.
#[derive(Debug, Clone)]
pub enum Activation {
    /// Rectified Linear Unit: `f(x) = max(0, x)`.
    ReLU,
    /// Sigmoid: `f(x) = 1 / (1 + e^(-x))`.
    Sigmoid,
    // /// Softmax: Converts a vector of values into a probability distribution.
    // Softmax,
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
    pub activation: Activation,
    weights: [Tensor<T, INPUT, INPUT_LANES>; OUTPUT],
    /// Pre-computed transpose of the weights matrix, used for efficient backpropagation.
    transpose: [Tensor<T, OUTPUT, OUTPUT_LANES>; INPUT],
    biases: [T; OUTPUT],
    error: Option<Tensor<T, INPUT, INPUT_LANES>>
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
    fn transpose(
        input: &[Tensor<f32, INPUT, INPUT_LANES>; OUTPUT],
    ) -> [Tensor<f32, OUTPUT, OUTPUT_LANES>; INPUT] {
        let mut mat: Vec<Vec<f32>> = vec![vec![0.0; OUTPUT]; INPUT];

        for (row, data) in input.iter().enumerate() {
            for (col, &value) in data.as_ref().iter().enumerate() {
                mat[col][row] = value;
            }
        }

        mat
            .into_iter()
            .map(|data| Tensor::new(data))
            .collect::<Vec<Tensor<f32, OUTPUT, OUTPUT_LANES>>>()
            .try_into()
            .unwrap()
    }

    /// Creates a new `Layer` with the given weights and biases.
    ///
    /// The transposed weight matrix is automatically computed and stored.
    pub fn new(
        activation: Activation,
        weights: [Tensor<f32, INPUT, INPUT_LANES>; OUTPUT],
        biases: [f32; OUTPUT],
        error: Option<Tensor<f32, INPUT, INPUT_LANES>>
    ) -> Self {
        Layer {
            activation,
            transpose: Self::transpose(&weights),
            weights,
            biases,
            error,
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
        x: &Tensor<f32, INPUT, INPUT_LANES>,
    ) -> Tensor<f32, OUTPUT, OUTPUT_LANES> {
        let mut y: Vec<f32> = vec![0.0; OUTPUT];
        for (i, &c) in self.biases.iter().enumerate() {
            let mx: f32 = x.clone() * &self.weights[i];
            y[i] = mx + c;
        }
        self.activate(&Tensor::new(y))
    }

    /// Performs the backward pass (backpropagation) for the layer.
    ///
    /// This method calculates the gradients for the weights and the error to be
    /// propagated to the previous layer.
    pub fn backward(
        &self,
        rate: &f32,
        dy: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
        x: &Tensor<f32, INPUT, INPUT_LANES>,
    ) -> Self {
        // 1. Calculate Weight Gradients (dw)
        // dw = dy * x^T
        let dw: [Tensor<f32, INPUT, INPUT_LANES>; OUTPUT] = dy.as_ref()
            .iter()
            .map(|error| x.clone() * error)
            .collect::<Vec<Tensor<f32, INPUT, INPUT_LANES>>>()
            .try_into()
            .unwrap();

        // 2. Calculate Bias Gradients (db)
        // The gradient for each bias is simply the error for that neuron.
        let db: [f32; OUTPUT] = dy.as_ref().clone();
        // 3. Propagate Error to the Previous Layer (dx)
        // dx = W^T * dy
        let mut dx = vec![0.0; INPUT];
        for (i, _dx) in dx.iter_mut().enumerate() {
            *_dx = Tensor::from(dy.as_ref()) * &self.transpose[i];
        }
        
        // 4. Update Weights and Biases
        let mut weights: [Tensor<f32, INPUT, INPUT_LANES>; OUTPUT] = self.weights.clone();
        let mut biases: [f32; OUTPUT] = self.biases.clone();
        for i in 0..OUTPUT {
            // dw = w - rate (dy * x)
            weights[i] = weights[i].clone() - &(dw[i].clone() * rate);
            // db = b - rate(dy)
            biases[i] -= db[i] * rate;
        }

        // 5. Return the updated layer and the error for the previous layer.
        Layer::new(self.activation.to_owned(), weights, biases, Some(Tensor::new(dx.to_vec())))
    }

    /// Calculates the derivative of the activation function.
    ///
    /// adjusted based on the activation function's derivative.
    ///
    /// # Arguments
    /// * `y` - The post-activation output from the forward pass.
    /// * `activation` - The activation function that was used.
    fn derivative(
        &self,
        y: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
    ) -> Tensor<f32, OUTPUT, OUTPUT_LANES> {
        let derivative = y.clone();
        match self.activation {
            Activation::None => derivative,
            Activation::ReLU => {
                // f'(x) = 1 if f(x) > 0 else 0
                derivative.floor(&0.0, Some(1.0))
            }
            Activation::Sigmoid => {
                // f'(x) = f(x) * (1.0 - f(x))
                derivative.clone() * ((derivative * &-1.0) + &1.0 ).as_ref()
            }
            Activation::Tanh => {
                // f'(x) = 1.0 - f(x)^2
                (derivative.clone() * derivative.as_ref()) * &-1.0 + &1.0
            }
            // Activation::Softmax => {
            //     // f'(x) = 1.0 - f(x)^2
            //     match derivative.data.iter().max_by(|a, b| a.total_cmp(b)) {
            //         Some(max) => {
            //             for v in derivative.data.iter_mut() {
            //                 *v = (*v - max).exp();
            //             }

            //         }
            //     }
            //     Tensor::new(derivative.data)
            // }
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
    ) -> Tensor<f32, OUTPUT, OUTPUT_LANES> {
        // Clone the input tensor to store the results.
        let mut result: Tensor<f32, OUTPUT, OUTPUT_LANES> = x.clone();
        match self.activation {
            // None: f(x) = x
            Activation::None => result,
            // ReLU (Rectified Linear Unit): f(x) = max(0, x)
            Activation::ReLU => {
                result.ceil(&0.0, None)
            }
            // Sigmoid: f(x) = 1 / (1 + e^(-x))
            Activation::Sigmoid => {
                for v in result.data.iter_mut() {
                    *v = 1.0 / (1.0 + (-*v).exp());
                }
                Tensor::new(result.data)
            }
            // Tanh (Hyperbolic Tangent): f(x) = tanh(x)
            // Squashes values to a range between -1 and 1. It's zero-centered, which can be advantageous.
            Activation::Tanh => {
                for v in result.data.iter_mut() {
                    let epos = v.exp();
                    let eneg = (-*v).exp();
                    *v = (epos - eneg) / (epos + eneg);
                }
                Tensor::new(result.data)
            }
            // // Softmax: f(x_i) = e^(x_i) / Σ(e^(x_j)) for all j
            // Activation::Softmax => {
            //     // `e^(x_i - max(x)) / Σ(e^(x_j - max(x)))` is mathematically equivalent to the original formula.
            //     for v in result.iter_mut() {
            //         let mut arr = v.to_array();
            //         for val in arr.iter_mut() {
            //             *val = libm::expf(*val);
            //         }
            //     }
            //     *v = f32x4::new(arr);
            //     // Calculate the sum of all exponentiated values.
            //     let mut sum = f32x4::splat(0.0);
            //     result.iter().for_each(|v| sum += *v);
            //     // Sum the elements within the final SIMD vector to get the total sum.
            //     let sum_lanes = sum.reduce_add();
            //     // Divide each exponentiated value by the total sum to get the final probabilities.
            //     result / &sum_lanes
            // }
        }
        
    }
}

use rand::RngExt;
pub fn random<
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT_LANES: usize,
    const OUTPUT: usize,
>(
    activation: Activation,
) -> Layer<f32, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT> {
    let mut rng = rand::rng();
    let weights: [Tensor<f32, INPUT, INPUT_LANES>; OUTPUT] = std::array::from_fn(|_| {
        let data = (0..INPUT)
            .map(|_| rng.random_range(-1.000..1.000f32))
            .collect::<Vec<f32>>();
        Tensor::new(data)
    });
    let biases: [f32; OUTPUT] = std::array::from_fn(|_| rng.random_range(-1.000..1.000f32));
    Layer::new(activation.to_owned(), weights, biases, None)
}

type Network<
    T,
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT_LANES: usize,
    const OUTPUT: usize,
> = Vec<Layer<T, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT>>;

pub(crate) fn forward<
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT_LANES: usize,
    const OUTPUT: usize,
>(
    network: &Network<f32, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT>,
    x: &Tensor<f32, INPUT, INPUT_LANES>,
) -> Vec<Tensor::<f32, OUTPUT, OUTPUT_LANES>> {
    let mut inputs: Vec<Tensor::<f32, OUTPUT, OUTPUT_LANES>> = vec![Tensor::new(x.clone().data)];
    for (i, layer) in network.iter().enumerate() {
        let peek = layer.forward(&Tensor::new(inputs[i].data.to_vec()));
        inputs.push(peek);
    }
    inputs
}

/// Performs a single training iteration (forward and backward pass) and returns the updated network.
pub (crate) fn train<
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT_LANES: usize,
    const OUTPUT: usize,
>(
    network: &mut Vec<Layer<f32, INPUT, INPUT_LANES, OUTPUT_LANES, OUTPUT>>,
    x: &Tensor<f32, INPUT, INPUT_LANES>,
    y: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
    rate: &f32,
) {
    if network.is_empty() {
        return;
    }
    // 1. Forward pass: We need to store the input and output of each layer for backpropagation.
    let outputs: Vec<Tensor::<f32, OUTPUT, OUTPUT_LANES>> = forward(&network, x);
    if let Some(output) = outputs.last() {
        let mut error: Tensor::<f32, OUTPUT, OUTPUT_LANES> = output.clone() - y; // Optimize to rather dereference the `yexp value`.
        // 2. Backward pass: Propagate the error from the output layer back to the input layer.
        for (i,  layer) in network.iter_mut().enumerate().rev()  {
            let dl = layer.derivative(&outputs[i+1]) * error.as_ref();
            *layer = layer.backward(rate, &dl, &Tensor::new(outputs[i].as_ref().to_vec()));
            if let Some(e) = &layer.error {
                error = Tensor::new(e.data.to_vec());
            }
    
        }
    }
}
