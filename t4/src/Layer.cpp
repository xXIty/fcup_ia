#include "Layer.hpp"

#include <random>
#include <iostream>
#include <iomanip>


typedef std::vector<float> VF;
typedef std::vector<std::vector<float>> VVF;


constexpr int WEIGHT_INI_MIN = -1;
constexpr int WEIGHT_INI_MAX =  1;


Layer::Layer() { }


Layer::Layer(std::size_t size, IActivationFunction* activation_function){
    this->size                 =  size;
    this->prior                =  NULL;
    //this->derivatives          =  VF(size);
    this->activation_function  =  activation_function;
}


// Layer::~Layer() { }



void Layer::set_prior(Layer* layer) {
    std::random_device                     rd;                    
    std::default_random_engine             eng(rd());             
    std::uniform_real_distribution<float>  distr(WEIGHT_INI_MIN,  WEIGHT_INI_MAX);

    this->prior             = layer;
    this->prior_activation  = VF(this->prior->get_size());
    this->weights           = VVF(this->size, VF(prior->get_size()+1));

    for (std::size_t j = 0; j < this->size; j++)
        for (std::size_t k = 0; k < prior->get_size() + 1; k++)
            this->weights[j][k] = distr(eng);
}


std::size_t Layer::get_size() {
    return this->size;
}

VF Layer::transfer_and_activate(VF prior_activation) {
    // For input layer do nothing
    if (!this->prior)
        return prior_activation;
    
    VF    transfer(this->size,0);
    VF    activation(this->size);
    this->prior_activation = prior_activation;
    this->derivatives = VF(this->size);

    for (size_t j = 0; j < this->size; ++j) {

        // Being j the nodes of this layer and k the nodes of previous layer:
        // input of nodes j: Zj = for all k, SUM(Wjk * Ak)
        for (size_t k = 0; k < this->prior->get_size(); ++k) 
            transfer[j] += this->weights[j][k] * this->prior_activation[k];

        // Add bias to node j
        transfer[j] += this->weights[j].back();

        // Prepare layer activation to return for the next layer
        activation[j]         =  this->activation_function->activate(transfer[j]);

        // Store derivatives of the layer for backpropagation
        this->derivatives[j]  =  this->activation_function->activate_prime(activation[j]);
    }

    return activation;
}


VF Layer::backpropagate(VF& next_delta_x_weights, float learning_rate) {
    // If no weights, do nothing
    if (this->prior == NULL)
        return next_delta_x_weights;

    float gradient_j_k;
    VF    deltas(this->size);
    VF    delta_x_weights(this->prior->get_size(), 0);


    for (size_t j = 0; j < this->size; ++j) {
        // Compute delta of layer nodes
        deltas[j] = this->derivatives[j] * next_delta_x_weights[j];

        
        for (size_t k = 0; k < this->prior_activation.size(); ++k) {
            // Update vector for prior layer
            // for each node k of previous layer:
            //            delta_x_weights_k = for all k: SUM(weight_j_k * delta_j)
            delta_x_weights[k] += this->weights[j][k] * deltas[j];
            
            // Calculate gradient for weight_j_k = delta_j * A_k
            gradient_j_k = deltas[j] * this->prior_activation[k];

            // Update weight
            float descent = learning_rate * gradient_j_k;
            this->weights[j][k] -= descent;
        }
        // Adjust bias
        this->weights[j].back() -= learning_rate * deltas[j];
    }

    return delta_x_weights;
}

void Layer::print() {
    std::cout << "Layer info:" << std::endl;
    std::cout << "\tSize: " << this->size << std::endl;
    std::cout << "\tDerivatives: ";
    for (auto x: this->derivatives)
        std::cout << x << " ";
    std::cout << std::endl;
    std::cout << "\tPrior act: ";
    for (auto x: this->prior_activation)
        std::cout << x << " ";
    std::cout << std::endl;
    std::cout << "\tWeights: " << std::endl;
    for (auto x: this->weights) {
        for (auto y: x)
            std::cout << std::setw(10) << y ;
        std::cout << std::endl;
    }
    std::cout << std::endl;
}
