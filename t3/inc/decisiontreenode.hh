#ifndef DECISION_TREE_NODE
#define DECISION_TREE_NODE

#include <string>
#include <vector>
#include <memory>

using namespace std;

class DecisionTreeNode {
    int attribute;
    string classification;

    vector<pair<string,unique_ptr<DecisionTreeNode>>> children;

    public:
        DecisionTreeNode();
        DecisionTreeNode(int attribute);
        DecisionTreeNode(string label);

        void set_attribute(int attribute);
        void set_classification(string label);

        void add_branch(string label, unique_ptr<DecisionTreeNode> subtree);
};

#endif
