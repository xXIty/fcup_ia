#include "IActivationFunction.hpp"

#include <cmath>

float ActivationFunctionSigmoid::activate(float input) {
    return 1 / (1 + exp(-input));
}


float ActivationFunctionNothing::activate(float input) {
    return input;
}
