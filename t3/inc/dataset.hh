#ifndef DATA_SET
#define DATA_SET

#include <string>
#include <vector>
#include <unordered_map>

using namespace std;

typedef unordered_map<string,int> UMSI;

#define IMPORTANCE_MIN 0

class DataSet {

    string                                        file_name;
    vector<vector<string>>                        content;

    vector<unordered_map<string,pair<int,UMSI>>>  attributes;
    vector<bool>                                  attribute_continous;
    vector<vector<string>>                        attribute_values;
    vector<string>                                attribute_headers;

    unordered_map<string,int>                     classes;
    int                                           class_index;

    private:
        float entropy(int count, unordered_map<string,int>& values);
        bool              is_continous(string value);

    public:

        DataSet(string filename);

        string            get_class(int row);
        vector<int>       get_attribute_identifiers();  
        vector<int>       get_row_identifiers();        
        vector<string>    get_attribute_headers();      
        vector<string>    get_attribute_values(int attribute);
        bool              classEq(vector<int> &rows);
        bool              is_attr_continuous(int attr);
        pair<string,int>  plurality_value(vector<int> &rows);
        void              load(string filename);
        float             importance_continuous(int                      attribute,
                                     const vector<int>&                  rows,
                                     float&                              split_point,
                                     unordered_map<string,vector<int>>&  attr_subsets_rows);
        float             importance_discrete(int                        attribute,
                                     const vector<int>&                  rows,
                                     unordered_map<string,vector<int>>&  attr_subsets_rows);
};

#endif
