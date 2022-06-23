#ifndef LAYER
#define LAYER

#include "neuron.hpp"

#include <vector>
#include <string>

class Layer {

    Layer* prior;
    Layer* next;
    
    std::vector<float>               previous_activation;
    std::vector<float>               derivatives;

    
    //  - weights_j_k is the weight between the k-th node in prior layer and
    //    the j-th node in this layer
    //  - weight_j_0 is the bias of neurone j.
    std::vector<std::vector<float>>  weights;

    public:
        Layer(Layer* prior, std::size_t size, std::string activation_func);

        std::size_t size();
        std::vector<float>  tranfer_and_activate(std::vector<float> previous_activation);
        void                backpropagate(std::vector<float> weights_x_delta);

};

#endif
