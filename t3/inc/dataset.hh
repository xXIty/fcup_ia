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
    int                                           row_size;
    int                                           col_size;
    int                                           class_index;

    vector<unordered_map<string,pair<int,UMSI>>>  attributes;

    vector<vector<string>>                        attribute_values;
    vector<string>                                attribute_labels;
    unordered_map<string,int>                     classes;

    private:
        float entropy(int count, unordered_map<string,int>& values);

    public:
        DataSet(string filename);

        bool            classEq(vector<int> &rows);
        int             get_col_size();
        int             get_row_size();
        string          get_class(int row);
        vector<string>  get_attribute_values(int attribute);
        vector<string>  get_attribute_labels();
        string          plurality_value(vector<int> &rows);
        void            load(string filename);
        float           importance(int                                 attribute,
                                   const vector<int>&                  rows,
                                   unordered_map<string,vector<int>>&  attr_subsets_rows);
};

#endif
