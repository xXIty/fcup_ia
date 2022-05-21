#ifndef DECISION_TREE_NODE
#define DECISION_TREE_NODE

#include <string>
#include <vector>

using namespace std;

class DecisionTreeNode {
    private:
        vector<DecisionTreeNode> children;

    public:
        int attribute;
        string label;

        DecisionTreeNode();
        DecisionTreeNode(string label);
};

#endif
