#ifndef DATA_SET
#define DATA_SET

#include "feature.hpp"

#include <utility>
#include <vector>
#include <string>
#include <filesystem>


namespace fs = std::filesystem;

class DataSet {
    fs::path                               file_path;
    std::vector<Feature>                   features;
    std::vector<std::vector<std::string>>  data_set_raw;

    private:
        std::vector<std::string> csv_line_parse(std::string); 
        void                     csv_process();

    public:
        DataSet(fs::path file_path);
        void                                              print_info();       
        size_t                                            size();             
        size_t                                            get_input_size();   
        size_t                                            get_output_size();  
        std::pair<std::vector<float>,std::vector<float>>  get_in_out(size_t row_id);
};

#endif
