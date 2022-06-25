#ifndef LAYER
#define LAYER

#include "IActivationFunction.hpp"

#include <vector>
#include <string>

class Layer {


    IActivationFunction*             activation_function;
    std::vector<float>               derivatives;
    Layer*                           next;
    Layer*                           prior;
    std::vector<float>               prior_act;
    size_t                           size;

    //  Weights_j_k is the weight between the k-th node in prior layer and
    //  the j-th node in this layer. Weight_j_0 is the bias of neurone j.
    std::vector<std::vector<float>>  weights;

    public:
        Layer();
        // ~Layer();
        Layer(std::size_t size, IActivationFunction* activation_function);

        std::size_t         get_size();
        void                set_next(Layer* layer);
        void                set_prior(Layer* layer);

        std::vector<float>  tranfer_and_activate(std::vector<float> prior_act);
        void                backpropagate(std::vector<float> prior_w_x_delta);

};

#endif
