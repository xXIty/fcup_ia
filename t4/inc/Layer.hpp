#ifndef LAYER
#define LAYER

#include "IActivationFunction.hpp"

#include <vector>
#include <string>

class Layer {


    IActivationFunction*             activation_function;
    std::vector<float>               derivatives;
    Layer*                           prior;
    std::vector<float>               prior_activation;
    size_t                           size;

    //  Weights_j_k is the weight between the k-th node in prior layer and
    //  the j-th node in this layer. Last weight of j-th node is its bias.
    std::vector<std::vector<float>>  weights;

    public:
        Layer();
        // ~Layer();
        Layer(std::size_t size, IActivationFunction* activation_function);

        std::size_t         get_size();
        void                set_prior(Layer* layer);
        void                print();

        std::vector<float>  transfer_and_activate(std::vector<float> prior_activation);
        std::vector<float>  backpropagate(std::vector<float>& next_delta_x_weights, float learning_rate);

};

#endif
