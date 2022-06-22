#include <utility>
#include <vector>
#include <string>

typedef std::vector<float> VF;
typedef std::vector<std::vector<float>> VVF;

class DataSet {
    std::string file_path;

    public:
        DataSet(std::string file_path);
        void print_info();
        size_t size();
        size_t get_input_size();
        size_t get_output_size();
        std::pair<VF,VF> get_in_out(size_t row_id);
};
