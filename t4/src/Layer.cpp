#include "Layer.hpp"

#include <random>
#include <iostream>


typedef std::vector<float> VF;
typedef std::vector<std::vector<float>> VVF;


constexpr int WEIGHT_INI_MIN = -1;
constexpr int WEIGHT_INI_MAX =  1;


Layer::Layer() { }


Layer::Layer(std::size_t size, IActivationFunction* activation_function){
    this->size                 =  size;
    this->derivatives          =  VF(size);
    this->activation_function  =  activation_function;
}


// Layer::~Layer() { }


void Layer::set_next(Layer* layer) {
    this->next = layer;
}

void Layer::set_prior(Layer* layer) {
    std::random_device                     rd;                    
    std::default_random_engine             eng(rd());             
    std::uniform_real_distribution<float>  distr(WEIGHT_INI_MIN,  WEIGHT_INI_MAX);

    this->prior_act  = VF(this->prior->get_size());
    this->weights    = VVF(this->size, VF(prior->get_size()+1));

    for (std::size_t j = 0; j < this->size; j++)
        for (std::size_t k = 0; k < prior->size + 1; k++)
            weights[j][k] = distr(eng);
}


std::size_t Layer::get_size() {
    return this->size;
}
