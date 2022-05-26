#include "dataset.hh"
#include "decisiontreenode.hh"

#include <iostream>

using namespace std;

DecisionTreeNode::DecisionTreeNode() {
    this->continuous = false;
}

DecisionTreeNode::DecisionTreeNode(int attribute) {
    this->attribute= attribute;
}

DecisionTreeNode::DecisionTreeNode(string classification) {
    this->classification = classification;
    this->continuous = false;
}

void DecisionTreeNode::set_attribute(int attribute) {
    this->attribute = attribute;
}

void DecisionTreeNode::set_classification(string classification, int count) {
    this->count           =  count;
    this->classification  =  classification;
}

void DecisionTreeNode::set_splitting_point(float sp) {
    this->continuous = true;
    this->split_point = sp;
}

void DecisionTreeNode::add_branch(string label, unique_ptr<DecisionTreeNode> subtree) {
    this->children.push_back(make_pair(label, move(subtree)));
}

bool DecisionTreeNode::is_leaf() {
    return (this->children.size() == 0);
}

void DecisionTreeNode::print(vector<string>& attribute_labels, int depth) {

    if (this->is_leaf()) {
        cout << this->classification << " (" << this->count << endl;
    }

    cout << string(4*depth, ' ') << "<" << attribute_labels[this->attribute] << ">" << endl;

    ++depth;
    for (auto &child : this->children) {
        string value;
        if ( this->continuous ) 
            value = child.first + to_string(this->split_point);
        else 
            value = child.first;

        if (child.second->is_leaf()) {
            
            string classification = child.second->classification;
            int count = child.second->count;
            cout << string(4*depth, ' ') 
                 << value << ": " 
                 << classification 
                 << " (" << count << ")" << endl; 
        }
        else { 
           cout << string(4*depth,' ') << value << ":" << endl;
           child.second->print(attribute_labels, 1+depth);
        }
    }
}



string DecisionTreeNode::decide(vector<string>& row) {

    if (this->is_leaf()) {
        return this->classification;
    }
    else {
        string label;
        if ( this->continuous ) {
            float value = stof(row[this->attribute]);
            label = (value >= this->split_point)? ">=" : "<";
            
        } else {
            label = row[this->attribute];
        }
        for (size_t i = 0; i < children.size(); i++) {
            if (this->children[i].first == label){
                return children[i].second->decide(row);
            }
        }
        return "Unknown attr value detected.";
        
    }
}
