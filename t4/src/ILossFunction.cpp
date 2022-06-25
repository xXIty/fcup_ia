#include "ILossFunction.hpp"

std::vector<float> LossFunctionSoftMax::loss(std::vector<float> value,
                                             std::vector<float> value_predicted) {
    return std::vector<float>(value.size());
} 

std::vector<float> LossFunctionSquaredError::loss(std::vector<float> value,
                                                   std::vector<float> value_predicted) {
    return std::vector<float>(value.size());
} 
