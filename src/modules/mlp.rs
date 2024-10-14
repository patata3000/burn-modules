use burn::prelude::*;
use nn::{Linear, LinearConfig, Relu};

#[derive(Config, Debug)]
pub struct MlpConfig {
    input_size: usize,
    hidden_size: usize,
    output_size: usize,
}

#[derive(Module, Debug)]
pub struct Mlp<B: Backend> {
    pub linear1: Linear<B>,
    pub activation: Relu,
    pub linear2: Linear<B>,
}

impl MlpConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> Mlp<B> {
        Mlp {
            linear1: LinearConfig::new(self.input_size, self.hidden_size).init(device),
            linear2: LinearConfig::new(self.hidden_size, self.output_size).init(device),
            activation: Relu::new(),
        }
    }
}

impl<B: Backend> Mlp<B> {
    pub fn forward<const D: usize>(&self, input: Tensor<B, D>) -> Tensor<B, D> {
        let x = self.linear1.forward(input);
        let x = self.activation.forward(x);
        self.linear2.forward(x)
    }
}
