#include "Model.hpp"

#include <unordered_map>
#include <iostream>

typedef std::vector<float> VF;
typedef std::vector<size_t> VS_T;
typedef std::vector<std::vector<float>> VVF;


Model::Model(std::string               loss_func,
             std::string               activation_func,
             std::vector<std::size_t>  hidden_layers_sizes,
             std::size_t               input_size,
             std::size_t               output_size){

    if (loss_func == "squared-error") {
        this->loss_function = new LossFunctionSquaredError;
    }
    else if (loss_func == "soft-max") {
        this->loss_function = new LossFunctionSoftMax;
    }
    else {
        this->loss_function = new LossFunctionSoftMax;
    }
    this->resize(input_size, hidden_layers_sizes, output_size);
}


Model::~Model() {
    delete this->loss_function;
}


void Model::resize(size_t               input_size,           
                   std::vector<size_t>  hidden_layers_sizes,  
                   size_t               output_size) {

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
        this->layers[layer-1].set_next(&this->layers[layer]);
        ++layer;
    }

    // Set output layer
    this->layers[layer] = Layer(output_size, new ActivationFunctionSigmoid);
    this->layers[layer-1].set_next(&this->layers[layer]);
}
