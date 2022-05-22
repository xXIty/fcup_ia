#include "dataset.hh"

#include <fstream>
#include <iostream>
#include <iterator>
#include <cmath>
#include <numeric>
#include <sstream>
#include <string>

using namespace std;

typedef unordered_map<string,int> UMSI;

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

        // Skip headers
        getline(fd, line);

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
}

float DataSet::entropy(int count, unordered_map<string,int> values) {
    float entropy = 0;
    for (auto const& value : values) {
        float  value_count   =  float(value.second);     
        float  value_probab  =  float(value_count) / float(count);
        entropy -= value_probab * log2(value_probab);
    }
    return entropy;
}

float DataSet::importance(int attr_index, vector<int> examples) { 

    unordered_map<string,pair<int,UMSI>>  attr_subsets;
    pair<int,UMSI>*                       subset_k;
    pair<int,UMSI>                        classes;
    float                                 examples_entropy;
    float                                 information_gain;
    float                                 reminder;

    // Set the total rows class_count will have.
    classes.first = examples.size();

    // 1. Generate subset that group the class of those rows with same attribute value.
    for (int row : examples) {

        // Pick the attribute and classification value of the row.
        string  row_attr_val   =  this->content[row][attr_index];
        string  row_class_val  =  this->content[row][this->class_index];

        // Count asside all the class values encountered.
        if (classes.second.count(row_class_val) == 0)
            classes.second.insert({row_class_val, 1});
        else
            ++classes.second.at(row_class_val);
        
        // Pick the subset of attribute identified by the row attribute value. 
        if (attr_subsets.count(row_attr_val) == 0)
            attr_subsets.insert({row_attr_val, pair<int,UMSI>(0,UMSI())});

        subset_k = &attr_subsets[row_attr_val];

        // Increment the number of rows of the subset.
        subset_k->first += 1;

        // Add the class value into the subset.
        if (subset_k->second.count(row_class_val) == 0) 
            subset_k->second.insert({row_class_val, 1});        
        else 
            ++subset_k->second.at(row_class_val);

    }

    examples_entropy = DataSet::entropy(classes.first, classes.second);

    // Calculate reminder of the attribute
    reminder = 0;
    for (auto s : attr_subsets) {
        subset_k = &s.second;
        float subset_k_entropy = DataSet::entropy(subset_k->first, subset_k->second);
        float subset_k_weight  = float(subset_k->first) / float(classes.first);

        // cout << s.first << endl;
        // cout << "\trows:    " << subset_k->first << endl;
        // cout << "\tEntropy: " << subset_k_entropy << endl;
        // cout << "\tWeight:  " << subset_k_weight << endl; 
        reminder += subset_k_weight * subset_k_entropy;
    }

    information_gain = examples_entropy - reminder;

    return information_gain;
}

//float DataSet::importance(Attribute attr) {
//    map<string, int> classes;
//    float gain = entropy;
//    float total = classifications.size();
//
//    for (auto const& v : attr.values) {
//        float totalk = v.second.size();
//        for (int i = 0; i < totalk; i++) {
//            string label = classifications[v.second[i]];
//            if (classes.count(label) == 0) {
//                classes.insert({label, 1});        
//
//            } else {
//                classes.at(label) += 1;
//            }
//        }
//        float entropyk = 0.0;
//        for (auto const& v : classes) {
//            float p = v.second/totalk;
//            entropyk -= p * log2(p);
//        }
//        gain -= (totalk/total) * entropyk;
//    }
//
//    return gain;
//    
//}

bool DataSet::classEq(vector<int> rows) { return false; }

string DataSet::get_class(int row) { return ""; }



string DataSet::plurality_value() { 

    // Calculate the value if not already done.
    if (plurality_val.length() == 0) {
        // Generate a vector with all rows.
        vector<int> rows(this->content.size());
        iota(++rows.begin(), rows.end(), 1);

        this->plurality_val = DataSet::plurality_value(rows);
    }

    return this->plurality_val;
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
