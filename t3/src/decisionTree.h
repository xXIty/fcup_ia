#ifndef DECISION_TREE_H_
#define DECISION_TREE_H_


#include <iostream>
#include <fstream>
#include <string>
#include <sstream>
#include <vector>
#include <map>
#include <cmath>

using namespace std;

class Attribute {
    public:
        string label;
        map<string, vector<int>> values; 
        Attribute(string name);
        void insert(int i, string value);
};

class DataSet {
    private:
        string filename;
        string plurality_value;
        vector<Attribute> attributes;
        vector<string> classifications;
        map<string, int> classes;
        float entropy;
        void parseDataset(string filename);
        void setEntropy();
    public:
        DataSet(string filename);
        void updateClassifications(string label);
        float importance(Attribute attr);
        //bool unique_class();
        void debug();
        
};

class DecisionTreeNode {
    public:
        string label;
        unsigned int attribute;
        vector<DecisionTreeNode> children;


};

#endif

