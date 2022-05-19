#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <map>
#include <cmath>

template <class T>
class Attribute {
    public:
        string label;
        map<T, vector<int>> values; 
}

class DataSet {
    private:
        string filename;
        string plurality_value;
        vector<Attribute> attributes;
        vector<string> classifications;
        map<string, int> classes;
        void parseDataset(string filename);
    public:
        vector<vector<string> examples;
        DataSet(string filename);
        
};

DataSet::DataSet(string filename) {

}
void DataSet::parseDataset(string filename) {
  ifstream fd(filename);
  vector<string> row;
  string line, word;
  vector<vector<string> content;

  if(fd.is_open())
    {
        while(getline(fd, line))
        {
            row.clear();
 
            stringstream str(line);
 
            while(getline(str, word, ',')) row.push_back(word);
            content.push_back(row);
        }
    }
 
  

  // Close the file
  fd.close();
    int cols, rows;
    cols = content.size();  
    rows = content[0].size();

    // Update attributes property
    for (int i = 1; i < cols-1; i++) {

        Attribute<string> attr(content[i][0]); 
      
        for (int j = 1; j < rows; j++) {
           attr.insert(j-1, content[i][j]); 
        }

    }

    // Update classification and classes property
    for (int j = 1; j < rows; j++) {
        updateClassifications(content[cols-1][j]);
    }

    

}


void DataSet::updateClassifications(string classification) {
    classifications.push_back(classification);
    // update classes map

}

class DecisionTreeNode {
    public:
        string label;
        unsigned int attribute;
        vector<DecisionTreeNode> children;


}


DecisionTreeNode ID3 (DataSet dataset) {
    
    if (dataset.unique_class()) {
        return DecisionTreeNode(dataset.classes[0]); // Leaf, all examples classified 
    }
    if (dataset.attributes.size() == 0) {
        return DecisionTreeNode(dataset.most_common_class()); // Leaf, no more attribute to split examples
    }

    Attribute attribute = dataset.max_importance_attr();
    DecisionTreeNode root = DecisionTreeNode(attribute.label);
    for (int i = 0; i < attribute.values.size(); i++) {
        DataSet examples = DataSet(dataset, attribute.label, attribute.values[i]);
        if (examples.size == 0) {
            root.add_child(DecisionTreeNode(dataset.most_common_class());
        } else {
            root.add_child(ID3(examples));

        }

    }

    return root;



}





