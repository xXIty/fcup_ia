#include "DataSet.hpp"
#include "Model.hpp"

#include <boost/program_options.hpp>
#include <iostream>
#include <filesystem>
#include <utility>
#include <algorithm>
#include <limits>


namespace po = boost::program_options;
namespace fs = std::filesystem;

typedef std::vector<float> VF;

int main(int argc, const char **argv) {

    float                     learning_rate;
    float                     train_stop;
    fs::path                  train_set_file;
    std::vector<std::size_t>  hidden_layers;

    // Declare the supported options.
    po::options_description desc("Allowed options");
    desc.add_options()

        ("help",
         "Produce help message")

        ("hidden,H",
         po::value< std::vector<std::size_t> >(&hidden_layers)->value_name("N"),
         "Add a hidden layer with N number of neurons")

        ("train,t",
         po::value< fs::path >(&train_set_file)->value_name("path"),
         "Train a new model with the data set provided. The data set"
         " must be in .csv format.")

        ("learning_rate,l",
         po::value< float >(&learning_rate)->default_value(0.05f),
         "Use the specified learning rate")

        ("a,a",
         po::value< float >(&train_stop)->default_value(0.05f),
         "Stop the learning procedure when an absolute error (difference) is achieved")

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

        float   loss_max = std::numeric_limits<float>::max();
        size_t  model_input_size;       
        size_t  model_output_size;      
        std::string  loss_function        =  "squared-error";
        std::string  activation_function  =  "sigmond";

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

        model_input_size   = train_set.get_input_size();
        model_output_size  = train_set.get_output_size();

        Model model(loss_function,
                    activation_function,
                    hidden_layers,
                    model_input_size,
                    model_output_size,
                    learning_rate);

        std::cout << "[+] Model setted up successfully" << std::endl;

        std::cout << "[+] Starting training..." << std::endl;

        size_t epoch_nubmer = 1;
        while (loss_max > train_stop) {
            loss_max = 0;
            for (size_t row_id = 0; row_id < train_set.size(); ++row_id) {
                std::pair<VF,VF> training_example = train_set.get_in_out(row_id);
                VF loss = model.train(training_example.first, training_example.second);

                // Update maximum loss between training_example and model prediction
                float training_example_loss_max = *std::max_element(std::begin(loss),
                                                                    std::end(loss));
                if (training_example_loss_max > loss_max)
                    loss_max = training_example_loss_max;
            }
            std::cout << "[+] Epoch: " << std::setw(3)   << epoch_nubmer++
                      << "  Loss: "    << std::setw(10)  << loss_max << std::endl;

        }
        
        return EXIT_SUCCESS;
    }
}
