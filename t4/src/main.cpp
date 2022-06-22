#include "dataset.hpp"
#include "model.hpp"

#include <boost/program_options.hpp>
#include <iostream>
#include <filesystem>


namespace po = boost::program_options;
namespace fs = std::filesystem;

int main(int argc, const char **argv) {

    fs::path       train_set_file;
    std::vector<int>  hidden_layers;

    // Declare the supported options.
    po::options_description desc("Allowed options");
    desc.add_options()

        ("help",
         "Produce help message")

        ("hidden,H",
         po::value< std::vector<int> >(&hidden_layers)->value_name("N"),
         "Add a hidden layer with N number of neurons")

        ("train,t",
         po::value< fs::path >(&train_set_file)->value_name("path"),
         "Train a new model with the data set provided. The data set must be in .csv format.")
    ;

    po::variables_map vm;
    po::store(po::parse_command_line(argc, argv, desc), vm);
    po::notify(vm);

    // Display help message.
    if (vm.count("help")) {
        std::cout << desc << "\n";
        return EXIT_SUCCESS;
    }

    // Train a new model.
    if (vm.count("train")) {

        std::cout << "[+] Pre processing the training set." << std::endl;

        // Check for needed information.
        if (!fs::exists(train_set_file)) {
            std::string error_msg = "\t[!] Error: File "
                                    + std::string(train_set_file)
                                    + " does not exist.";
            std::cout << error_msg << std::endl;
            return EXIT_FAILURE;
        } 

        DataSet train_set(train_set_file);
        train_set.print_info();

        std::cout << "[+] Setting up model" << std::endl;

        if (hidden_layers.empty()) {
            std::string error_msg = "\t[!] Error: No hidden layers where specified. "
                                    "Please, enter at least one.";
            std::cout << error_msg << std::endl;
            return EXIT_FAILURE;
        }
    }
}
