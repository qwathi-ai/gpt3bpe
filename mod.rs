mod unit;
use rand::Rng;
use std::cmp::{max, min};

// #[derive(Debug, thiserror::Error)]
// pub enum Error {
// 	#[error("Collection already exists")]
// 	UniqueViolation,

// 	#[error("Collection doesn't exist")]
// 	NotFound,

// 	#[error("The dimension of the vector doesn't match the dimension of the collection")]
// 	DimensionMismatch,
// }

#[derive(Debug, Clone)]
struct Edge {
    weight: f32,
    change: f32,
}
#[derive(Clone, Debug)]
struct Node {
    bias: f32,
    delta: f32,
    edges: Vec<Edge>,
}
#[derive(Clone, Debug)]
pub enum Activation {
    Sigmoid,
    Tanh,
    Relu,
    // Custom,
}
#[derive(Clone, Debug)]
struct ActivationContainer {
    function: fn(f32) -> f32,
    derivative: fn(f32) -> f32,
}

#[derive(Clone, Debug)]
pub struct Neural {
    pub layers: Vec<usize>,
    pub iterations: usize,
    pub threshold: f32,
    pub rate: f32,
    pub momentum: f32,
    pub method: Activation,
    // #[serde(skip_deserializing, skip_serializing)]
    activation: ActivationContainer,
    nodes: Vec<Vec<Node>>,
}

fn initialize(layers: &[usize]) -> Vec<Vec<Node>> {
    let mut rng = rand::thread_rng();
    let mut input = vec![];
    for _ in 0..layers[0] {
        input.push(Node {
            bias: rng.gen_range(0.01..0.1),
            delta: 0.0,
            edges: vec![],
        });
    }
    let mut state = vec![input];
    let mut cursor = layers.iter().enumerate().peekable();

    while let Some((_layer, nodes)) = cursor.next() {
        if let Some((peek_layer, peek_nodes)) = cursor.peek() {
            state.insert(*peek_layer, vec![]);

            for node in 0..**peek_nodes {
                let mut edges = vec![];
                for _ in 0..*nodes {
                    edges.push(Edge {
                        weight: rng.gen_range(0.45..0.65), //0.15 * thread_rng().gen::<f32>() * 0.45,
                        change: 0.0,
                    })
                }
                state[*peek_layer].insert(
                    node,
                    Node {
                        bias: rng.gen_range(0.01..0.1),
                        delta: 0.0,
                        edges,
                    },
                )
            }
        }
    }
    // println!("{:#?}", state);
    state
}
fn error(nodes: &Vec<Vec<f32>>) -> f32 {
    let mut sum: f32 = 0.0;
    for node in &nodes[nodes.len() - 1] {
        sum += node.powi(2);
    }
    sum / (nodes[nodes.len() - 1].len() as f32)
}

pub fn sigmoid(x: f32) -> f32 {
    1.0 / (1.0 + x.exp())
}

pub fn sigmoid_(x: f32) -> f32 {
    // sigmoid(x) * (1.0 - sigmoid(x))
    x * (1.0 - x)
}

pub fn tanh(x: f32) -> f32 {
    x.tanh()
}

pub fn tanh_(x: f32) -> f32 {
    1.0 - x.tanh().powi(2)
}

pub fn relu(x: f32) -> f32 {
    f32::max(0.0, x)
}
pub fn relu_(x: f32) -> f32 {
    if x <= 0.0 {
        0.0
    } else {
        1.0
    }
}

impl Neural {
    pub fn new(layers: Vec<usize>) -> Self {
        Self {
            nodes: initialize(&layers),
            layers,
            iterations: 20000,
            threshold: 0.005,
            rate: 0.3,
            momentum: 0.1,
            method: Activation::Sigmoid,
            activation: ActivationContainer {
                function: sigmoid,
                derivative: sigmoid_,
            },
        }
    }
    pub fn nodes(self, nodes: Vec<Vec<Node>>) -> Self {
        Self { nodes, ..self }
    }
    pub fn iterations(self, iterations: usize) -> Self {
        Self { iterations, ..self }
    }
    pub fn threshold(self, threshold: f32) -> Self {
        Self { threshold, ..self }
    }
    pub fn rate(self, rate: f32) -> Self {
        Self { rate, ..self }
    }
    pub fn momentum(self, momentum: f32) -> Self {
        Self { momentum, ..self }
    }
    pub fn method(self, activation: Activation) -> Self {
        Self {
            activation: match &activation {
                Activation::Sigmoid => ActivationContainer {
                    function: sigmoid,
                    derivative: sigmoid_,
                },
                Activation::Tanh => ActivationContainer {
                    function: tanh,
                    derivative: tanh_,
                },
                Activation::Relu => ActivationContainer {
                    function: relu,
                    derivative: relu_,
                },
            },
            method: activation,
            ..self
        }
    }
    pub fn activation(&self, bias: &f32) -> f32 {
        (self.activation.function)(*bias)
    }
    pub fn deactivation(&self, output: &f32) -> f32 {
        (self.activation.derivative)(*output)
    }
    pub fn thumb(input: i32, output: i32, samples: i32, factor: i32) -> Neural {
        let hidden = max(samples / (factor * (input + output)), 3) as usize;
        // let hidden = max(samples / (factor * (input + output)), 2) as usize;

        let mut layers = vec![input as usize];
        for _ in 0..hidden {
            layers.push(max(max(input, output), 2) as usize)
            // layers.push(min(input, output) as usize)
        }
        layers.push(output as usize);
        println!("layers: {:?}", layers);
        Neural::new(layers)
    }
    pub fn forward(&self, chunk: &Vec<f32>) -> Vec<Vec<f32>> {
        let mut outputs = vec![chunk.to_vec()];
        let mut cursor = self.nodes.iter().enumerate().peekable();

        while let Some((layer, _nodes)) = cursor.next() {
            if let Some((peek_layer, peek_nodes)) = cursor.peek() {
                outputs.insert(*peek_layer, vec![]);
                for (idx, node) in peek_nodes.iter().enumerate() {
                    let mut bias = node.bias;
                    for (_idx, edge) in node.edges.iter().enumerate() {
                        bias += edge.weight * outputs[layer][_idx];
                        println!(
                            "layer: {:?}    peek layer: {:?}    peek node: {:?}  edge: {:?}  input: {:?}    weight: {:?} bias: {:?}",
                            layer, peek_layer, idx, _idx, outputs[layer][_idx], edge.weight, bias
                        );
                    }
                    outputs[*peek_layer].insert(idx, self.activation(&bias));
                }
            };
        }

        outputs
    }
    fn backward(&mut self, outputs: &[Vec<f32>], target: &Vec<f32>) -> Vec<Vec<f32>> {
        let last = self.nodes.len() - 1;
        let mut corrections = vec![vec![]; self.nodes.len()];
        let mut binding = self.nodes.to_vec();
        let mut cursor = self.nodes.iter().enumerate().rev();

        while let Some((layer, nodes)) = cursor.next() {
            for (idx, _node) in nodes.iter().enumerate() {
                let output = outputs[layer][idx];
                let mut error = 0.0;
                if layer == last {
                    error = output - target[idx];
                    corrections[layer].insert(idx, error);
                } else {
                    for alpha in binding[layer + 1].iter() {
                        for (_idx, edge) in alpha.edges.iter().enumerate() {
                            error += alpha.delta * edge.weight;
                        }
                    }
                }
                binding[layer][idx].delta = error * self.deactivation(&output);
                corrections[layer].insert(idx, error);
                println!(
                    "layer: {:?}   target: {:?}    corrections: {:?}    output: {:?}    error: {:?}    delta: {:?}",
                    layer, target, corrections, output, corrections[layer][idx], binding[layer][idx].delta
                );
            }
        }

        self.nodes = binding;
        corrections
    }

    fn update(&mut self, outputs: &[Vec<f32>]) {
        let mut binding = self.nodes.to_vec();
        let mut cursor = self.nodes.iter().enumerate().peekable();

        while let Some((layer, _nodes)) = cursor.next() {
            if let Some((peek_layer, peek_nodes)) = cursor.peek() {
                for (idx, _node) in peek_nodes.iter().enumerate() {
                    for (_idx, alpha) in outputs[layer].iter().enumerate() {
                        let change = self.rate
                            * binding[*peek_layer][idx].delta
                            * alpha
                            * self.momentum
                            * binding[*peek_layer][idx].edges[_idx].change;

                        binding[*peek_layer][idx].edges[_idx].change = change;
                        binding[*peek_layer][idx].edges[_idx].weight += change;
                        println!(
                            "layer: {:?}    peek layer: {:?}    input: {:?} node: {:?}  edge: {:?}  change: {:?}    weight: {:?}",
                            layer, peek_layer, alpha, idx, _idx, binding[*peek_layer][idx].edges[_idx].change, binding[*peek_layer][idx].edges[_idx].weight
                        );
                    }

                    binding[*peek_layer][idx].bias += self.rate * binding[*peek_layer][idx].delta;
                }
            }
        }
        self.nodes = binding;
    }
    pub fn fit(&mut self, chunk: &Vec<f32>, target: &Vec<f32>) -> f32 {
        let output = &self.forward(chunk);
        let corrections = &self.backward(&output, &target);
        self.update(&output);
        error(corrections)
    }
}
