#ifndef DATA_SET
#define DATA_SET

#include <string>
#include <vector>
#include <unordered_map>

using namespace std;

#define IMPORTANCE_MIN 0

class DataSet {

    string                  file_name;
    vector<vector<string>>  content;
    int                     row_size;
    int                     col_size;
    int                     class_index;
    string                  plurality_val;

    private:
        float entropy(int count, unordered_map<string,int> values);

    public:
        DataSet(string filename);

        bool    classEq(vector<int> rows);
        float   importance(int attribute, vector<int> rows);
        string  get_class(int row);
        string  plurality_value();
        string  plurality_value(vector<int> &rows);
        void    load(string filename);

        // void updateClassifications(string label);
        //bool unique_class();
};

#endif
