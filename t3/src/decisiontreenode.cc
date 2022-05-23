#include "dataset.hh"
#include "decisiontreenode.hh"

#include <iostream>

using namespace std;

DecisionTreeNode::DecisionTreeNode() {}

DecisionTreeNode::DecisionTreeNode(int attribute) {
    this->attribute= attribute;
}

DecisionTreeNode::DecisionTreeNode(string classification) {
    this->classification = classification;
}

void DecisionTreeNode::set_attribute(int attribute) {
    this->attribute = attribute;
}

void DecisionTreeNode::set_classification(string classification, int count) {
    this->count           =  count;
    this->classification  =  classification;
}

void DecisionTreeNode::add_branch(string label, unique_ptr<DecisionTreeNode> subtree) {
    this->children.push_back(make_pair(label, move(subtree)));
}

bool DecisionTreeNode::is_leaf() {
    return (this->children.size() == 0);
}

void DecisionTreeNode::print(vector<string>& attribute_labels, int depth) {

    cout << string(4*depth, ' ') << "<" << attribute_labels[this->attribute] << ">" << endl;

    ++depth;
    for (auto &child : this->children) {
        if (child.second->is_leaf()) {
            string value = child.first;
            string classification = child.second->classification;
            int count = child.second->count;
            cout << string(4*depth, ' ') 
                 << value << ": " 
                 << classification 
                 << "(" << count << ")" << endl; 
        }
        else { 
           cout << string(4*depth,' ') << child.first << ":" << endl;
           child.second->print(attribute_labels, 1+depth);
        }
    }
}
