#include "layer.hpp"

#include <random>

typedef std::vector<float> VF;
typedef std::vector<std::vector<float>> VVF;

constexpr int WEIGHT_INI_MIN = -1;
constexpr int WEIGHT_INI_MAX =  1;

Layer::Layer(Layer* prior, std::size_t size, std::string activation_func){
    this->prior = prior;
    this->derivatives = VF(size);
    
    if (prior) {
        std::random_device                     rd;                    
        std::default_random_engine             eng(rd());             
        std::uniform_real_distribution<float>  distr(WEIGHT_INI_MIN,  WEIGHT_INI_MAX);

        this->weights = VVF(size, VF(prior->size()+1));

        for (std::size_t j = 0; j < size; j++)
            for (std::size_t k = 0; k < prior->size() + 1; k++)
                weights[j][k] = distr(eng);

    }
        
}



std::size_t Layer::size() {
    return this->derivatives.size();
}
