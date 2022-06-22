#include "layer.hpp"

#include <iostream>
#include <string>
#include <vector>

typedef std::vector<float> VF;

class Model {

    std::string name;
    std::string trainFile;
    std::string testFile;

    private:
        void  trainSetPreProcess();                
        VF    evaluate(VF& input);       
        void  train(VF& input, VF& output);

    public:
        void   setUp();
        float  trainEpoch();
        void   test(std::string testFile);

};
