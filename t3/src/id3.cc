#include "id3.hh"

#include "dataset.hh"
#include "decisiontreenode.hh"

#include <string>
#include <vector>

using namespace std;


unique_ptr<DecisionTreeNode> id3(vector<int>  examples,         
                                 vector<int>  attributes,       
                                 vector<int>  examples_parent,  
                                 DataSet      &data_set)        {

    unique_ptr<DecisionTreeNode> tree = make_unique<DecisionTreeNode>();

    if (examples.empty())  
        tree->set_classification(data_set.plurality_value(examples_parent));
    
    else if (data_set.classEq(examples)) 
        tree->set_classification(data_set.get_class(examples[0]));
    
    else if (attributes.empty()) 
        tree->set_classification(data_set.plurality_value(examples));


    else {

        vector<int>::iterator              attr                   =  attributes.begin();
        float                              attr_importance;          
        unordered_map<string,vector<int>>  attr_subsets_rows      =  {};
        vector<int>::iterator              attr_max               =  attr;
        float                              attr_max_importance    =  IMPORTANCE_MIN;
        unordered_map<string,vector<int>>  attr_max_subsets_rows  =  {};

        // Find attribute with max importance and its subsets of rows.
        for (; attr != attributes.end(); attr++) {

            // Calculate IMPORTANCE
            attr_importance = data_set.importance(*attr, examples, attr_subsets_rows);

            if (attr_importance > attr_max_importance) {
                attr_max               =  attr;
                attr_max_importance    =  attr_importance;
                attr_max_subsets_rows  =  attr_subsets_rows;
            }

            attr_subsets_rows.clear();
        }

        attributes.erase(attr_max);

        for (string attr_val : data_set.get_attribute_values(*attr_max)) {
            vector<int> exs = attr_subsets_rows[attr_val];
            unique_ptr<DecisionTreeNode> subtree = id3(exs, attributes, examples, data_set);
            tree->add_branch(attr_val, move(subtree));
        }
    }
    
    return tree;
}

//DecisionTreeNode ID3 (DataSet dataset) {
//    
//    if (dataset.unique_class()) {
//        return DecisionTreeNode(dataset.classifications[0]); // Leaf, all examples classified 
//    }
//    if (dataset.attributes.size() == 0) {
//        return DecisionTreeNode(dataset.most_common_class()); // Leaf, no more attribute to split examples
//    }
//
//    Attribute attribute = dataset.max_importance_attr();
//    DecisionTreeNode root = DecisionTreeNode(attribute.label);
//    for (int i = 0; i < attribute.values.size(); i++) {
//        DataSet examples = DataSet(dataset, attribute.label, attribute.values[i]);
//        if (examples.size == 0) {
//            root.add_child(DecisionTreeNode(dataset.most_common_class());
//        } else {
//            root.add_child(ID3(examples));
//
//        }
//
//    }
//
//    return root;
//
//
//
//}
//




