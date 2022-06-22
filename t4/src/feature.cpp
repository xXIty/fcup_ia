#include "feature.hpp"

typedef  std::vector<float> VF;

Feature::Feature(std::string name) {
    this->name      = name;
    this->type_name = "Continuous";
}

std::string  Feature::get_name() const {
    return this->name;
}

size_t Feature::size() const {
    return size_t(1);
}

std::string Feature::type() const {
    return this->type_name;
}

VF Feature::raw_to_model(std::string value) const {
    return VF (1, std::stof(value));
}

std::string Feature::model_to_raw(VF value) const {
    return std::to_string(value[0]);
}
