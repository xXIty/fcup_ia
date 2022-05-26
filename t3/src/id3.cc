#include "id3.hh"

#include "dataset.hh"
#include "decisiontreenode.hh"

#include <string>
#include <vector>
#include <iostream>

using namespace std;


unique_ptr<DecisionTreeNode> id3(vector<int>  examples,         
                                 vector<int>  attributes,       
                                 vector<int>  examples_parent,  
                                 DataSet      &data_set)        {

    unique_ptr<DecisionTreeNode> tree = make_unique<DecisionTreeNode>();

    if (examples.empty()) {
        pair<string,int> plurality_value = data_set.plurality_value(examples_parent); 
        tree->set_classification(plurality_value.first, 0);
    }
    
    else if (data_set.classEq(examples)) {
        tree->set_classification(data_set.get_class(examples[0]), examples.size());
    }
    
    else if (attributes.empty()) {
        pair<string,int> plurality_value = data_set.plurality_value(examples); 
        tree->set_classification(plurality_value.first, plurality_value.second);
    }

    else {

        vector<int>::iterator              attr                   =  attributes.begin();
        float                              attr_importance;          
        unordered_map<string,vector<int>>  attr_subsets_rows      =  {};
        vector<int>::iterator              attr_max               =  attr;
        float                              attr_max_importance    =  IMPORTANCE_MIN;
        unordered_map<string,vector<int>>  attr_max_subsets_rows  =  {};
        int                                attr_choosed;


        // Find attribute with max importance and its subsets of rows.
        float split_point;
        for (; attr != attributes.end(); attr++) {

            // Calculate IMPORTANCE
            float sp;
            if (data_set.is_attr_continuous(*attr)) 
                attr_importance = data_set.importance_continuous(*attr, examples, sp, attr_subsets_rows);
            else 
                attr_importance = data_set.importance_discrete(*attr, examples, attr_subsets_rows);

            if (attr_importance > attr_max_importance) {
                attr_max               =  attr;
                attr_max_importance    =  attr_importance;
                attr_max_subsets_rows  =  attr_subsets_rows;
                split_point            =  sp;
            }

            attr_subsets_rows.clear();
        }

        attr_choosed = *attr_max;
        if (data_set.is_attr_continuous(attr_choosed) ) 
            tree->set_splitting_point(split_point);
        tree->set_attribute(attr_choosed);
        attributes.erase(attr_max);

        for (string attr_max_val : data_set.get_attribute_values(attr_choosed)) {

            vector<int> exs = attr_max_subsets_rows[attr_max_val];

            unique_ptr<DecisionTreeNode> subtree = id3(exs, attributes, examples, data_set);
            tree->add_branch(attr_max_val, move(subtree));
        }
    }
    
    return tree;
}
