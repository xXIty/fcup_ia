#include "Model.hpp"

#include <unordered_map>
#include <iostream>
#include <iomanip>
#include <algorithm>

typedef std::vector<float> VF;
typedef std::vector<size_t> VS_T;
typedef std::vector<std::vector<float>> VVF;


Model::Model(std::string                loss_func,
             std::string                activation_func,
             std::vector<std::size_t>&  hidden_layers_sizes,
             std::size_t                input_size,
             std::size_t                output_size,
             float                      learning_rate){

    if (loss_func == "squared-error") {
        this->loss_function = new LossFunctionSquaredError;
    }
    else if (loss_func == "soft-max") {
        this->loss_function = new LossFunctionSoftMax;
    }
    else {
        this->loss_function = new LossFunctionSoftMax;
    }
    this->learning_rate = learning_rate;
    this->resize(input_size, hidden_layers_sizes, output_size);
}


Model::~Model() {
    delete this->loss_function;
}


void Model::resize(size_t                input_size,           
                   std::vector<size_t>&  hidden_layers_sizes,  
                   size_t                output_size) {

    int layer = 0;

    // Input layer + hidden layers + output layer
    this->layers = std::vector<Layer>(2+hidden_layers_sizes.size());

    // Set input layer
    this->layers[layer++] = Layer(input_size, new ActivationFunctionNothing);


    // Set hidden layers
    for (size_t layer_size: hidden_layers_sizes) {
        Layer layer_hidden(layer_size, new ActivationFunctionSigmoid);
        this->layers[layer] = layer_hidden;

        this->layers[layer].set_prior(&this->layers[layer-1]);
        ++layer;
    }

    // Set output layer
    this->layers[layer] = Layer(output_size, new ActivationFunctionSigmoid);
    this->layers[layer].set_prior(&this->layers[layer-1]);
}

VF Model::train(VF& in, VF& out) {

    VF loss_prime;
    VF prediction;
    VF layer_activation = in;
    VF layer_deltas_x_weights;
    VF loss;

    // Feed-forward
    for (Layer& layer: this->layers)
        layer_activation = layer.transfer_and_activate(layer_activation);

    prediction = layer_activation;
    loss = this->loss_function->loss(out, prediction);

    // Compute derivative of loss
    layer_deltas_x_weights = this->loss_function->loss_prime(out, prediction);

    // Back propagate loss
    for (size_t l = this->layers.size()-1; l > 0; --l)
        layer_deltas_x_weights = this->layers[l].backpropagate(layer_deltas_x_weights, this->learning_rate);

    return loss;
}
