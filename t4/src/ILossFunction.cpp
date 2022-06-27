#include "ILossFunction.hpp"

#include <iostream>

typedef std::vector<float> VF;

VF LossFunctionSoftMax::loss(std::vector<float> value,
                             std::vector<float> value_predicted) {
    return std::vector<float>(value.size());
} 

VF LossFunctionSoftMax::loss_prime(std::vector<float> value,
                                   std::vector<float> value_predicted) {
    return std::vector<float>(value.size());
}

VF LossFunctionSquaredError::loss(std::vector<float> value,
                                  std::vector<float> value_predicted) {
    VF squared_error(value.size());
    for (std::size_t i = 0; i < value.size(); ++i) {
        float error = value[i] - value_predicted[i];
        squared_error[i] = error * error;
    }
    return squared_error;
} 

VF LossFunctionSquaredError::loss_prime(std::vector<float> value,
                                        std::vector<float> value_predicted) {
    VF loss_prime(value.size());

    for (std::size_t i = 0; i < value.size(); ++i) {
         loss_prime[i] = 2 * value_predicted[i] * (value_predicted[i] - value[i]);
    }
    
    return loss_prime;
}
