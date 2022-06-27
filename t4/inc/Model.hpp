#ifndef MODEL
#define MODEL

#include "ILossFunction.hpp"
#include "Layer.hpp"

#include <vector>
#include <string>


class Model {

    ILossFunction*      loss_function;
    std::vector<Layer>  layers;
    float               learning_rate;

    public:
        Model(std::string                loss_func,
              std::string                activation_func,
              std::vector<std::size_t>&  hidden_layers_sizes,
              size_t                     input_size,
              size_t                     output_size,
              float                      learning_rate);

        ~Model();

        void set_ativation_function(std::string loss_func);

        void resize(size_t                     input_size,
                    std::vector<std::size_t>&  hidden_layers_sizes,
                    size_t                     output_size);
        
        std::vector<float> evaluate(std::vector<float>& input);       

        std::vector<float> train(std::vector<float>& in, std::vector<float>& out);

};

#endif
