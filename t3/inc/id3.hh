#include "dataset.hh"
#include "decisiontreenode.hh"

#include <memory>

unique_ptr<DecisionTreeNode> id3(vector<int> examples, vector<int> attributes, DataSet &data_set);
