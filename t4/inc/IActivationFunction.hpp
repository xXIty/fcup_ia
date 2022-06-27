#ifndef IACTIVATION_FUNCTION_H
#define IACTIVATION_FUNCTION_H

class IActivationFunction {
    public:
        virtual ~IActivationFunction() = default;
        virtual float activate(float input) = 0;
        virtual float activate_prime(float activate) = 0;
};


class ActivationFunctionSigmoid : public IActivationFunction {
    public:
        float activate(float input);
        float activate_prime(float activate);
};


class ActivationFunctionNothing : public IActivationFunction {
    public:
        float activate(float input);
        float activate_prime(float activate);
};

#endif
