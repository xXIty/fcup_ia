#include "id3.hh"

#include "dataset.hh"
#include "decisiontreenode.hh"

#include <string>
#include <vector>

using namespace std;


unique_ptr<DecisionTreeNode> id3(vector<int> examples, vector<int> attributes, DataSet &data_set) {

    if (examples.empty())  
        return make_unique<DecisionTreeNode>(data_set.plurality_value());
    
    else if (data_set.classEq(examples)) 
        return make_unique<DecisionTreeNode>(data_set.get_class(examples[0]));
    
    else if (attributes.empty()) 
        return make_unique<DecisionTreeNode>(data_set.plurality_value(examples));

    else {

        // Find the attr from attributes that maximizes IMPORTANCE function 
        // for the examples.
        vector<int>::iterator  attr           =  attributes.begin();
        vector<int>::iterator  attr_max       =  attr;
        int                    importance_max =  IMPORTANCE_MIN;

        for (; attr != attributes.end(); attr++) {

            // Calculate IMPORTANCE
            float importance_tmp = data_set.importance(*attr, examples);

            if (importance_tmp > importance_max) {
                importance_max  =  importance_tmp;
                attr_max = attr;
            }

        }

        attributes.erase(attr_max);

        
    }
    
    return make_unique<DecisionTreeNode>();
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




