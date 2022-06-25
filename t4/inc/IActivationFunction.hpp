#ifndef IACTIVATION_FUNCTION_H
#define IACTIVATION_FUNCTION_H

class IActivationFunction {
    public:
        virtual ~IActivationFunction() = default;
        virtual float activate(float input) = 0;
};


class ActivationFunctionSigmoid : public IActivationFunction {
    public:
        float activate(float input);
};


class ActivationFunctionNothing : public IActivationFunction {
    public:
        float activate(float input);
};

#endif
