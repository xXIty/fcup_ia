#ifndef DATA_SET
#define DATA_SET

#include <map>
#include <string>
#include <vector>
#include <cmath>
#include <sstream>
#include <iostream>
#include <fstream>

using namespace std;

#define IMPORTANCE_MIN 0

class Attribute {
    public:
        // Attributes
        string label;
        map<string, vector<int>> values; 

        // Methods
        Attribute(string name);
        void insert(int i, string value);
};

class DataSet {

    string  filename;
    float   entropy;
    string  plurality;

    private:
        map<string, int> classes;
        vector<string> classifications;

        vector<Attribute> attributes;

        // Methods
        void parseDataset(string filename);
        void setEntropy();

    public:
        // Constructor
        DataSet(string filename);

        // Processing Classification
        string get_class(int row);
        string plurality_value();
        string plurality_value(vector<int> &rows);
        bool classEq(vector<int> rows);

        // Processing Attributes
        float importance(int attribute, vector<int> rows);

        void updateClassifications(string label);
        //bool unique_class();
        void debug();
        
};

#endif
