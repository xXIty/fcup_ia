#include  "cxxopts.hh"
#include  "dataset.hh"
#include  "id3.hh"
#include  "decisiontreenode.hh"

#include <fstream>
#include <iostream>
#include <sstream>
#include <string>


using namespace std;

int main(int argc, char* argv[]) {

    cxxopts::Options     options("ID3 algorithm", "ID3 algorithm that can be trained with continuous and multivalued input with a multivalued output.");
    cxxopts::ParseResult op_parsed;

    options.add_options()
        ("h,help", "Print usage")
        ("v,verbose", "Print the decision tree.", cxxopts::value<bool>()->default_value("false"))
        ("f,train_file", "Input file for training.", cxxopts::value<std::string>())
        ("t,test_file", "Input file for testing. Only attributes (no ID) with comma separated values.", cxxopts::value<std::string>())
        ("i,interactive", "Classify space sparated rows provided by standard input.", cxxopts::value<bool>()->default_value("false"));

    options.allow_unrecognised_options();

    try {
        op_parsed = options.parse(argc, argv);
    } catch (const cxxopts::OptionParseException &x) {
        std::cerr << "ID3 algorithm: " << x.what() << '\n';
        cout << options.help() << endl;
        return EXIT_FAILURE;
    }


    if (op_parsed.count("help")) {
      cout << options.help() << endl;
      return EXIT_SUCCESS;
    }


    if (!op_parsed.count("train_file")) {
        cerr << "[!] ID3: algorithm: Missing mandatory input file." << endl;
        cerr << options.help() << endl;
        exit(0);
    }


    string       file_train = op_parsed["train_file"].as<std::string>();
    DataSet      dataset(file_train);                 

    vector<int>  examples = dataset.get_row_identifiers();       
    vector<int>  attr_ids = dataset.get_attribute_identifiers();

    unique_ptr<DecisionTreeNode> tree = id3(examples,
                                            attr_ids,
                                            vector<int>(),
                                            dataset);


    if (op_parsed.count("verbose")) {
        vector<string> attribute_headers = dataset.get_attribute_headers();
        tree->print(attribute_headers, 0);
    }


    if (op_parsed.count("test_file")) {
        typedef vector<string>         VS;
        typedef vector<vector<string>> VVS;


        string    file_test       =  op_parsed["test_file"].as<std::string>();
        VVS       rows;
        VS        row;               
        string    line;              
        string    word;              
        ifstream  fd(file_test);     

        if(fd.is_open()) {
            while(getline(fd, line)) {
                row.clear();
                stringstream str(line);
                while (getline(str, word, ',')) 
                    row.push_back(word);
                rows.push_back(row);
            }
        }
        fd.close();

        for (VS row : rows)
            cout << tree->decide(row) << endl;

    }

    if (op_parsed.count("interactive")) {
        while (true) {
            vector<string> row(attr_ids.size());
            for (size_t i = 0; i < row.size(); i++)
                cin >> row[i];
            cout << tree->decide(row) << endl;
        }

    }
}
