#include "dataset.hh"
#include "decisiontreenode.hh"

using namespace std;

DecisionTreeNode::DecisionTreeNode() {}

DecisionTreeNode::DecisionTreeNode(int attribute) {
    this->attribute= attribute;
}

DecisionTreeNode::DecisionTreeNode(string classification) { this->classification = classification; }

void DecisionTreeNode::set_attribute(int attribute) {
    this->attribute = attribute;
}

void DecisionTreeNode::set_classification(string classification) {
    this->classification = classification;
}

void DecisionTreeNode::add_branch(string label, unique_ptr<DecisionTreeNode> subtree) {
    auto branch = make_pair(label, move(subtree));
    this->children.push_back(branch);
}
