#include  "dataset.hh"
#include  "id3.hh"
#include  "decisiontreenode.hh"

#include <iostream>

using namespace std;

int main(int argc, char* argv[]) {

    DataSet      dataset(argv[1]);                 

    vector<int>  examples = dataset.get_row_identifiers();       
    vector<int>  attr_ids = dataset.get_attribute_identifiers();

    unique_ptr<DecisionTreeNode> tree = id3(examples,
                                            attr_ids,
                                            vector<int>(),
                                            dataset);

    vector<string> attribute_headers = dataset.get_attribute_headers();

    tree->print(attribute_headers, 0);
}
