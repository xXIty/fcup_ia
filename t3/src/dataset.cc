#include "dataset.hh"

#include <string>

using namespace std;

Attribute::Attribute(string name) {
    label = name; 
}

void Attribute::insert(int i, string value) {
    
    if (values.count(value) == 0) {
        values.insert({value, {}});        

    } else {
        values.at(value).push_back(i);
    }

}


DataSet::DataSet(string filename) {
    parseDataset(filename);
}

bool DataSet::classEq(vector<int> rows) { return false; }

string DataSet::get_class(int row) { return ""; }

string DataSet::plurality_value() { return ""; }

string DataSet::plurality_value(vector<int>& row) { return ""; }


void DataSet::parseDataset(string filename) {
    ifstream fd(filename);
    vector<string> row;
    string line, word;
    vector<vector<string>> content;

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
    rows = content.size();  
    cols = content[0].size();

    // Update attributes property
    for (int i = 1; i < cols-1; i++) {

        Attribute attr(content[0][i]); 
      
        for (int j = 1; j < rows; j++) {
           attr.insert(j-1, content[j][i]); 
        }

        attributes.push_back(attr);

    }

    // Update classification and classes property
    for (int j = 1; j < rows; j++) {
        updateClassifications(content[j][cols-1]);
    }

    setEntropy();

    

}


void DataSet::updateClassifications(string label) {
    // update classes map
    classifications.push_back(label);

    if (classes.count(label) == 0) {
        classes.insert({label, 1});        

    } else {
        classes.at(label) += 1;
    }

}

void DataSet::setEntropy() {
    float total;
    float p;
    float s;

    s = 0.0;
    total = classifications.size(); 

    for (auto const& x : classes) {
        float p = x.second/total;
        s -= p * log2(p);
    }

    entropy = s;

}

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

void DataSet::debug() {

    cout << "Entropy: " << entropy << endl;
//    for (int i = 0; i < attributes.size(); i++) {
//        float  gain = importance(attributes[i]);
//        cout <<  "GAIN(" << attributes[i].label << ") = " << gain << endl; 
//
//    }


}
