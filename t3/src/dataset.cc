#include "dataset.hh"

#include <fstream>
#include <iostream>
#include <iterator>
#include <numeric>
#include <map>
#include <sstream>
#include <string>

using namespace std;

DataSet::DataSet(string filename) {
    this->file_name = filename;
}

void DataSet::load() {
    DataSet::load(this->file_name);
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
}

//void DataSet::setEntropy() {
//    float total;
//    float p;
//    float s;
//
//    s = 0.0;
//    total = classifications.size(); 
//
//    for (auto const& x : classes) {
//        float p = x.second/total;
//        s -= p * log2(p);
//    }
//
//    entropy = s;
//}

float DataSet::importance(int attr, vector<int> examples) { return 0.0f; }
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
    map<string,int>  classifications;              
    string           plurality_value;               
    int              plurality_value_count = 0;                
    int              content_class_index   =  this->content[0].size()-1;

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
