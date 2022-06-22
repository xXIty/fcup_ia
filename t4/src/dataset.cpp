#include "dataset.hpp"

#include <iostream>
#include <fstream>
#include<iomanip>

typedef  std::vector<float>                     VF;
typedef  std::vector<std::string>               VS;
typedef  std::vector<std::vector<std::string>>  VVS;


DataSet::DataSet(fs::path file_path) {
    this->file_path = file_path;
    this->csv_process();
}


VS DataSet::csv_line_parse(std::string line) {
    std::string               cell;
    std::stringstream         line_stream(line);
    std::vector<std::string>  result;

    while(std::getline(line_stream,cell, ',')) {
        result.push_back(cell);
    }
    // This checks for a trailing comma with no data after it.
    if (!line_stream && cell.empty()) {
        // If there was a trailing comma then add an empty element.
        result.push_back("");
    }
    return result;
}

void DataSet::csv_process() {
    std::ifstream             file(this->file_path);
    std::string               line_content;
    std::vector<std::string>  feature_names;
    std::vector<std::string>  feature_values;

    // Parse feature names.
    file >> line_content;
    feature_names = this->csv_line_parse(line_content);

    // Create features
    for (std::string name: feature_names) 
        features.push_back(Feature(name));

    // Process .csv lines with feature values
    while (file >> line_content) {
        feature_values = this->csv_line_parse(line_content);
        this->data_set_raw.push_back(feature_values);
    }
}

size_t DataSet::get_input_size() {
    size_t input_size = 0;
    for (size_t i = 0; i < this->features.size()-1; i++)
        input_size += features[i].size();
    return input_size;
}

size_t DataSet::get_output_size() {
    return features.end()->size();
}

void DataSet::print_info() {
    std::cout << "\t[+] Data set features:" << std::endl;
    std::cout << std::setw(15) << "Name";
    std::cout << std::setw(15) << "Type";
    std::cout << std::setw(15) << "#Units";
    std::cout << std::endl;

    for (const auto &feature: this->features) {
        std::cout << std::setw(15) << feature.get_name();
        std::cout << std::setw(15) << feature.type();
        std::cout << std::setw(15) << feature.size(); 
        std::cout << std::endl;
    }
}
