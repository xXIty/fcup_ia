#include "dataset.hh"

#include <fstream>
#include <iostream>
#include <iterator>
#include <cmath>
#include <sstream>
#include <string>
#include <numeric>

using namespace std;





DataSet::DataSet(string filename) {
    this->file_name = filename;
    this->load(this->file_name);
}



string DataSet::get_class(int row) {
    return this->content[row][this->class_index];
}



vector<string> DataSet::get_attribute_values(int attribute) {
    return this->attribute_values[attribute];
}



vector<int> DataSet::get_attribute_identifiers() {
    vector<int>  attribute_identifiers(this->content[0].size()-1);
    iota(attribute_identifiers.begin(),  attribute_identifiers.end(),  0);
    return attribute_identifiers;
}



vector<int> DataSet::get_row_identifiers() {
    vector<int> row_identifiers(this->content.size());
    iota(row_identifiers.begin(), row_identifiers.end(), 0);
    return row_identifiers;
}



vector<string> DataSet::get_attribute_headers() {
    return this->attribute_headers;
}



void DataSet::load(string filename) {
    vector<string>  row;
    string          line;
    string          word;
    ifstream        fd(filename);

    if(fd.is_open()) {

        // Parse attribute_headers
        getline(fd, line);
        row.clear();
        stringstream str(line);

        // Headers
        getline(str, word, ','); // Skip ID
        while(getline(str, word, ','))  {
            this->attribute_headers.push_back(word);
        }

        this->attribute_headers.pop_back(); // Remove class
        this->attributes = vector<unordered_map<string,pair<int,UMSI>>>(this->attribute_headers.size());

        // Read actual data
        while(getline(fd, line)) {
            row.clear();
            stringstream str(line);

            // Skip ID
            getline(str,word,',');

            while(getline(str, word, ',')) row.push_back(word);
            this->content.push_back(row);

        }
    }
    fd.close();

    this->class_index  =  content[0].size()-1;

    // Generate Class map.
    for (size_t row = 0; row < this->content.size(); row++)
        this->classes.insert({this->content[row][class_index],0});

    // Generate Attribute map.
    for (size_t row = 0; row < this->content.size(); row++) {
        for (size_t col = 0; col < this->content[0].size()-1; col++) {
            string attr_val = content[row][col];
            this->attributes[col].insert({attr_val, pair<int,UMSI>(0,this->classes)});
        }
    }

    // Generate list of values for each attr.
    for (auto attribute : attributes) {
        vector<string> attribute_values;
        for (auto attribute_val : attribute) {
            attribute_values.push_back(attribute_val.first);
        }

        this->attribute_values.push_back(attribute_values);
    }
}



bool DataSet::classEq(vector<int> &rows) {
    bool    equal           =  true;
    string  classification  =  this->content[rows[0]][class_index];

    for (size_t i = 1; i < rows.size(); i++) {
        if (classification != this->content[rows[i]][class_index]) {
            equal = false;
            break;
        }
    }

    return equal;
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



float DataSet::importance(int                                 attr_index,    
                          const vector<int>&                  examples,      
                          unordered_map<string,vector<int>>&  attr_subsets_rows)  {

    unordered_map<string,pair<int,UMSI>>  attr_subsets_class;
    pair<int,UMSI>*                       subset_k;
    pair<int,UMSI>                        classes;
    float                                 examples_entropy;
    float                                 information_gain;
    float                                 attr_reminder;

    // Gather the subsets for the attribute and class
    classes             =  pair<int,UMSI>(0,this->classes);
    attr_subsets_class  =  this->attributes[attr_index];

    // Fill each subset with data from input data and pick classifications.
    for (int row : examples) {
        // Pick the attribute and classification value of the row.
        string  row_attr_val   =  this->content[row][attr_index];
        string  row_class_val  =  this->content[row][this->class_index];

        // Count class values of examples.
        ++classes.second.at(row_class_val);
        
        // Add row to the  subset
        attr_subsets_rows[row_attr_val].push_back(row);

        // Add the row class to the subset
        subset_k = &attr_subsets_class[row_attr_val];

        ++subset_k->first;

        ++subset_k->second.at(row_class_val);
    }

    // Entropy(examples)
    classes.first    = examples.size();
    examples_entropy = DataSet::entropy(classes.first, classes.second);
    
    // Reminder(attribute)
    attr_reminder = 0;
    for (auto s : attr_subsets_class) {
        subset_k = &s.second;

        float subset_k_entropy = DataSet::entropy(subset_k->first, subset_k->second);
        float subset_k_weight  = float(subset_k->first) / float(classes.first);

        attr_reminder += subset_k_weight * subset_k_entropy;
    }

    // Information Gain(examples, attr) = Entropy(examples) - Remainder(attr)
    information_gain = examples_entropy - attr_reminder;

    return information_gain;
}



pair<string,int> DataSet::plurality_value(vector<int>& rows) {
    UMSI                       classifications = this->classes;          
    string                     plurality_value;          
    int                        plurality_value_count  =  0;
    int                        content_class_index    =  this->content[0].size()-1;

    // Populate classifications map with the rows selected from dataset.
    for (size_t i = 0; i < rows.size(); i++) {
        string value = content[rows[i]][content_class_index];

        classifications.at(value) += 1;
    }

    // Go through classifications to find the most popular classification value.
    for (auto value : classifications) {
       if (value.second > plurality_value_count) {
           plurality_value_count = value.second;
           plurality_value = value.first;
       }
    }
    
    return make_pair(plurality_value, plurality_value_count);
}
