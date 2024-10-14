use std::collections::HashSet;

use burn::prelude::*;

use crate::dynamic_model::NodeType;

use super::mlp::{Mlp, MlpConfig};

#[derive(Config, Debug)]
pub struct NodeWeightsConfig {
    /// The input size of raw embeddings of a node. Used in "MLP-b"
    b_input_size: usize,

    /// The hidden sized of "MLP-b".
    b_hidden_size: usize,

    /// Used as:
    /// - the output size of "MLP-b"
    /// - the input size of "MLP-w"
    /// - the output size of "MLP-w"
    ///
    /// We need to be able to do "MLP-b output" + "MLP-w output"
    /// And the "MLP-b output" is used as "MLP-w input".
    /// That's why we use same size for these 3.
    output_size: usize,

    /// The hidden size of "MLP-w".
    w_hidden_size: usize,
}

#[derive(Module, Debug)]
pub struct NodeWeights<B: Backend> {
    pub mlp_b: Mlp<B>,
    pub mlp_w: Mlp<B>,
}

impl NodeWeightsConfig {
    fn init<B: Backend>(&self, device: &B::Device) -> NodeWeights<B> {
        NodeWeights {
            mlp_b: MlpConfig::new(
                self.b_input_size,
                self.b_hidden_size,
                self.output_size,
            )
            .init(device),
            mlp_w: MlpConfig::new(
                self.output_size,
                self.w_hidden_size,
                self.output_size,
            )
            .init(device),
        }
    }
}

#[derive(Module, Debug)]
pub struct NodeMapping<B: Backend> {
    node_weights: Vec<NodeWeights<B>>,
}

impl<B: Backend> NodeMapping<B> {
    pub fn get_node_type_weights(&self, node_type: &NodeType) -> NodeWeights<B> {
        let node_idx: usize = (*node_type).into();
        self.node_weights
            .get(node_idx)
            .expect("node weights should exist in node mapping")
            .clone()
    }

    pub fn forward_w<const D: usize>(
        &self,
        input: Tensor<B, D>,
        node_type: NodeType,
    ) -> Tensor<B, D> {
        let node_weights = self.get_node_type_weights(&node_type);
        node_weights.mlp_w.forward(input)
    }

    pub fn forward_b<const D: usize>(
        &self,
        input: Tensor<B, D>,
        node_type: &NodeType,
    ) -> Tensor<B, D> {
        let node_weights = self.get_node_type_weights(node_type);
        node_weights.mlp_b.forward(input)
    }
}

#[derive(Config, Debug)]
pub struct NodeMappingConfig {
    node_types: HashSet<NodeType>,
    node_weights: NodeWeightsConfig,
}

impl NodeMappingConfig {
    pub fn init<B: Backend>(&self, device: &B::Device) -> NodeMapping<B> {
        let mut node_weights = Vec::new();
        for _node_type in self.node_types.iter() {
            node_weights.push(self.node_weights.init(device))
        }
        NodeMapping { node_weights }
    }
}
