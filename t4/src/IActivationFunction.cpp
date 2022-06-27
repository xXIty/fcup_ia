#include "IActivationFunction.hpp"

#include <cmath>


// Sigmoid
// --------
float ActivationFunctionSigmoid::activate(float input) {
    return 1 / (1 + exp(-input));
}

float ActivationFunctionSigmoid::activate_prime(float activate) {
    return activate * (1 - activate);
}


// Nothing
// --------
float ActivationFunctionNothing::activate(float input) {

    return input;
}

float ActivationFunctionNothing::activate_prime(float activate) {
    return activate;
}
