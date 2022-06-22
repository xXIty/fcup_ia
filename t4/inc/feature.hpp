#include <vector>
#include <string>

typedef std::vector<float> VF;

class Feature {
    std::string name;
    std::string type_name;

    public:
        Feature(std::string name);

        std::string  get_name() const;
        size_t       size() const;
        std::string  type() const;
        VF           raw_to_model(std::string value) const;  
        std::string  model_to_raw(VF feature) const;
};
