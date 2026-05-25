use crate::neural::tensor;

/// The Adam Optimizer State: The "Working Memory" for a specific layer.
/// Only needed during Training.
pub struct AdamState<T, const I: usize, const O: usize, const IL: usize> 
where T: Copy 
{
    // We only need moments for the primary weights. 
    // weights_t is updated by mirroring 'weights'.
    pub m_weights: [tensor::Tensor<T, I, IL>; O],
    pub v_weights: [tensor::Tensor<T, I, IL>; O],
    
    pub m_biases: [T; O],
    pub v_biases: [T; O],
    
    pub t: u32, // Timestep for bias correction
}