#ifndef MODEL
#define MODEL

#include "layer.hpp"

#include <vector>
#include <string>




class Model {

    std::vector<Layer> layers;

    public:
        Model(std::string               loss_func,
              std::string               activation_func,
              std::vector<std::size_t>  hidden_layers,
              std::size_t               input_size,
              std::size_t               output_size);

        std::vector<float>    evaluate(std::vector<float>& input);       
        float                 train(std::vector<float>& input, std::vector<float>& output);

};

#endif
