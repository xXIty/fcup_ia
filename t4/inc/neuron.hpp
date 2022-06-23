#ifndef NEURON
#define NEURON

#include <string>

class Neuron {
    public:
        Neuron(std::string activation_func);
        float activate(float input);
};

#endif
