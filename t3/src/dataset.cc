#include "dataset.hh"

#include <fstream>
#include <iostream>
#include <iterator>
#include <cmath>
#include <numeric>
#include <sstream>
#include <string>

using namespace std;


// TO DO
bool DataSet::classEq(vector<int> rows) { return false; }
string DataSet::get_class(int row) { return ""; }




DataSet::DataSet(string filename) {
    this->file_name = filename;
    this->load(this->file_name);
}



void DataSet::load(string filename) {
    vector<string>  row;
    string          line;
    string          word;
    ifstream        fd(filename);

    if(fd.is_open()) {

        // Parse headers
        getline(fd, line);
        row.clear();
        stringstream str(line);

        // Initialize the Attributes map vector.
        while(getline(str, word, ','))  
            this->attributes.push_back(unordered_map<string,pair<int,UMSI>>());

        // Read actual data
        while(getline(fd, line)) {
            row.clear();
            stringstream str(line);
            while(getline(str, word, ',')) row.push_back(word);
            this->content.push_back(row);

        }
    }
    fd.close();

    this->row_size     =  content[0].size();
    this->col_size     =  content.size();
    this->class_index  =  row_size-1;

    // Generate classes map.
    for (size_t row = 0; row < this->content.size(); row++)
        this->classes.insert({this->content[row][class_index],0});

    // Generate Attribute maps.
    for (size_t row = 0; row < this->content.size(); row++) {
        for (size_t col = 0; col < this->content[0].size()-1; col++) {
            string attr_val = content[row][col];
            this->attributes[col].insert({attr_val, pair<int,UMSI>(0,this->classes)});
        }
    }
}



float DataSet::entropy(int count, unordered_map<string,int>& values) {
    float entropy = 0;

    for (auto const& value : values) {
        if (value.second == 0)
            continue;

        float  value_count   =  float(value.second);
        float  value_probab  =  float(value_count) / float(count);

        entropy -= value_probab * log2(value_probab);
    }
    return entropy;
}



float DataSet::importance(int attr_index, vector<int>& examples) { 

    unordered_map<string,pair<int,UMSI>>  attr_subsets;
    pair<int,UMSI>*                       subset_k;
    pair<int,UMSI>                        classes;
    float                                 examples_entropy;
    float                                 information_gain;
    float                                 attr_reminder;

    // Gather the subsets for the attribute and class
    classes       =  pair<int,UMSI>(0,this->classes);
    attr_subsets  =  this->attributes[attr_index];

    // Fill each subset with data from input data and pick classifications.
    for (int row : examples) {
        // Pick the attribute and classification value of the row.
        string  row_attr_val   =  this->content[row][attr_index];
        string  row_class_val  =  this->content[row][this->class_index];

        // Count class values of examples.
        ++classes.second.at(row_class_val);
        
        // Add row to the corresponding subset
        subset_k = &attr_subsets[row_attr_val];
        subset_k->first += 1;
        if (subset_k->second.count(row_class_val) == 0) 
            subset_k->second.insert({row_class_val, 1});        
        else 
            ++subset_k->second.at(row_class_val);

    }

    // Entropy(examples)
    classes.first    = examples.size();
    examples_entropy = DataSet::entropy(classes.first, classes.second);
    
    // Reminder(attribute)
    attr_reminder = 0;
    for (auto s : attr_subsets) {
        subset_k = &s.second;

        float subset_k_entropy = DataSet::entropy(subset_k->first, subset_k->second);
        float subset_k_weight  = float(subset_k->first) / float(classes.first);

        attr_reminder += subset_k_weight * subset_k_entropy;
    }

    // Information Gain(examples, attr) = Entropy(examples) - Remainder(attr)
    information_gain = examples_entropy - attr_reminder;

    return information_gain;
}

string DataSet::plurality_value(vector<int>& rows) {
    unordered_map<string,int>  classifications;          
    string                     plurality_value;          
    int                        plurality_value_count  =  0;
    int                        content_class_index    =  this->content[0].size()-1;

    // Populate classifications map with the rows selected from dataset.
    for (size_t i = 0; i < rows.size(); i++) {
        string value = content[rows[i]][content_class_index];

        if (classifications.count(value) == 0) {
            classifications.insert({value, 1});        

        } else {
            classifications.at(value) += 1;
        }
    }

    // Go through classifications to find the most popular classification value.
    for (auto value : classifications) {
       if (value.second > plurality_value_count) {
           plurality_value_count = value.second;
           plurality_value = value.first;
       }
    }
    
    return plurality_value;
}
