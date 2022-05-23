#include  "dataset.hh"
#include  "id3.hh"
#include  "decisiontreenode.hh"

#include <iostream>
#include <numeric>

using namespace std;

int main(int argc, char* argv[]) {

    DataSet      dataset(argv[1]);                 
    int          dataset_col_size               =  dataset.get_col_size();
    int          dataset_row_size               =  dataset.get_row_size();

    vector<int>  examples(dataset_col_size);       
    vector<int>  attributes(dataset_row_size-1);     

    iota(examples.begin(),    examples.end(),    0);
    iota(attributes.begin(),  attributes.end(),  0);

    unique_ptr<DecisionTreeNode> tree = id3(examples,
                                            attributes,
                                            vector<int>(),
                                            dataset);
    vector<string> attribute_labels = dataset.get_attribute_labels();

    cout << endl;
    cout << "PRINT FUNCTION..." << endl;
    tree->print(attribute_labels, 0);
}


