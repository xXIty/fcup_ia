#include "model.hpp"

typedef std::vector<float> VF;
typedef std::vector<std::vector<float>> VVF;

Model::Model(std::string               loss_func,
             std::string               activation_func,
             std::vector<std::size_t>  hidden_layers,
             std::size_t               input_size,
             std::size_t               output_size){

    // Input layer
    this->layers.push_back(Layer(NULL, input_size, NULL));

//    for (std::size_t layer_size: hidden_layers) {
//        
//    }
}
