#include "dataset.hh"
#include "decisiontreenode.hh"

#include <iostream>

using namespace std;

DecisionTreeNode::DecisionTreeNode() { cout << "uau" << endl;}

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
    this->children.push_back(make_pair(label, move(subtree)));
}
