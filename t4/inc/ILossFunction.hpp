#ifndef ILOSS_FUNCTION_H
#define ILOSS_FUNCTION_H

#include <vector>

class ILossFunction {
    public:
        virtual ~ILossFunction() = default;
        virtual std::vector<float> loss(std::vector<float> value,
                                        std::vector<float> value_predicted) = 0; 
};

class LossFunctionSquaredError : public ILossFunction {
    public:
        std::vector<float> loss(std::vector<float> value,
                                std::vector<float> value_predicted); 
};

class LossFunctionSoftMax : public ILossFunction {
    public:
        std::vector<float> loss(std::vector<float> value,
                                std::vector<float> value_predicted); 
};

#endif
