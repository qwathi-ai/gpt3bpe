mod optimizer;
mod tensor;
mod unit;
use crate::neural::tensor::Tensor;
use wide::f32x4;

/// Represents the activation function to be applied to the output of a neural network layer.
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

type Layers<
    T,
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT: usize,
    const OUTPUT_LANES: usize,
> = Vec<Layer<T, INPUT, INPUT_LANES, OUTPUT, OUTPUT_LANES>>;

/// A tuple holding the gradients for a layer's weights and biases.
type Gradients<T, const INPUT: usize, const INPUT_LANES: usize, const OUTPUT: usize> = (
    // Weight gradients
    [Tensor<T, INPUT, INPUT_LANES>; OUTPUT],
    // Bias gradients
    [T; OUTPUT],
);

/// A fully connected layer in a neural network.
#[derive(Clone, Debug)]
pub struct Layer<
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
    > Layer<f32, INPUT, INPUT_LANES, OUTPUT, OUTPUT_LANES>
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
        Layer {
            transpose: Self::transpose(&weights),
            weights,
            biases,
        }
    }

    /// Performs the forward pass of the layer (Modified to accept &self instead of consuming self).
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
        prev: Vec<f32>
    ) -> Gradients<f32, INPUT, INPUT_LANES, OUTPUT> {
        // 4. Calculate bias changes required for training from adjusted signals
        let prev = Tensor::new(prev);

        // 4. Calculate weight changes using outer product of adjusted error and previous input
        let dw: [Tensor<f32, INPUT, INPUT_LANES>; OUTPUT] = dx
            .iter()
            .map(|&d| prev.clone() * &d)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let dx = Tensor::new(dx);

        // 5. Calculate the difference / error required for the previous layer (dx * W)
        let mut gradient: [] = vec![0.0; INPUT];
        for (i, item) in gradient.iter_mut().enumerate() {
            // Fix: Multiply the column of the transposed weights matrix by the adjusted incoming error
            *item = &self.transpose[i] * &dx;
        }


        // 6. Return weight changes, bias changes, and difference in signals for previous layer
        (dw, gradient)
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

/// Represents a simple multi-layer Network (MLP).
pub struct Network<
    const INPUT: usize,
    const INPUT_LANES: usize,
    const OUTPUT: usize,
    const OUTPUT_LANES: usize,
> {
    pub iterations: usize,
    pub threshold: f32,
    pub rate: f32,
    pub momentum: f32,
    pub layers: Layers<f32, INPUT, INPUT_LANES, OUTPUT, OUTPUT_LANES>,
}

impl<
        const INPUT: usize,
        const INPUT_LANES: usize,
        const OUTPUT: usize,
        const OUTPUT_LANES: usize,
    > Network<INPUT, INPUT_LANES, OUTPUT, OUTPUT_LANES>
where
    for<'b> &'b Tensor<f32, INPUT, INPUT_LANES>:
        std::ops::Mul<&'b Tensor<f32, INPUT, INPUT_LANES>, Output = f32>,
    for<'b> &'b Tensor<f32, OUTPUT, OUTPUT_LANES>:
        std::ops::Mul<&'b Tensor<f32, OUTPUT, OUTPUT_LANES>, Output = f32>,
{
    pub fn new(
        iterations: usize,
        threshold: f32,
        rate: f32,
        momentum: f32,
        layers: Option<Layers<f32, INPUT, INPUT_LANES, OUTPUT, OUTPUT_LANES>>,
    ) -> Self {
        Network {
            iterations,
            threshold,
            rate,
            momentum,
            layers: match layers {
                Some(l) => l,
                None => vec![],
            }
        }
    }

    pub fn add(&mut self, layer: Layer<f32, INPUT, INPUT_LANES, OUTPUT, OUTPUT_LANES>) {
        self.layers.push(layer);
    }

    pub fn train(
        &mut self,
        x: &Tensor<f32, INPUT, INPUT_LANES>,
        y: &Tensor<f32, OUTPUT, OUTPUT_LANES>,
        activation: &Activation,
    ) {
        // --- 1. Forward Pass ---
        let mut inputs: NetworkInputs<f32, OUTPUT, OUTPUT_LANES> = vec![];
        let mut input = x.clone().as_ref().to_vec();

        for layer in self.layers.iter() {
            let output = layer.forward(input.as_slice().try_into().unwrap(), activation);
            inputs.push(output.clone());
            input = output.as_ref().to_vec();
        }

        // --- 2. Backward Pass ---
        // Fix: Clone the last entry out before applying subtraction to satisfy operator requirements
        let mut cost = y - &inputs.last().unwrap().clone();
        let mut partials = vec![];

        for (i, layer) in self.layers.iter().rev().enumerate() {
            let idx = self.layers.len() - 1 - i;
            let output = &inputs[idx];        // Input to this layer
            let current = &inputs[idx + 1];  // Output of this layer
            
            let d= layer.backward(&cost, output, current, activation.clone());
            cost = grad_to_pass;
            partials.push(layer_grads);
        }
        all_gradients.reverse();

        // --- 3. Update Weights, Biases & Sync Transposed Representation ---
        for (i, layer) in self.layers.iter_mut().enumerate() {
            let (d_weights, d_biases) = &all_gradients[i];
            for j in 0..OUTPUT {
                // Fix: Clone moving elements explicitly into value-taking operators
                layer.weights[j] = layer.weights[j].clone() - &(d_weights[j].clone() * &self.rate);
                layer.biases[j] -= d_biases[j] * self.rate;
            }
            // CRITICAL FIX: Re-sync transpose matrix so the next training pass uses updated values!
            layer.transpose = Self::transpose(&layer.weights);
        }
    }
}