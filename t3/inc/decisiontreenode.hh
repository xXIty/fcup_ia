#ifndef DECISION_TREE_NODE
#define DECISION_TREE_NODE

#include <string>
#include <vector>
#include <memory>

using namespace std;

class DecisionTreeNode {

    int                                                attribute;
    int                                                count;
    float                                              split_point;
    bool                                               continuous;
    string                                             classification;
    vector<pair<string,unique_ptr<DecisionTreeNode>>>  children;

    public:
        DecisionTreeNode();
        DecisionTreeNode(int attribute);
        DecisionTreeNode(string label);

        void    set_attribute(int attribute);
        void    set_classification(string label, int count);

        bool    is_leaf();
        void    set_splitting_point(float sp);
        void    add_branch(string label, unique_ptr<DecisionTreeNode> subtree);
        void    print(vector<string>& attribute_labels, int depth);
        string  decide(vector<string>& row);
};

#endif
