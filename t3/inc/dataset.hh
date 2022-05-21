#ifndef DATA_SET
#define DATA_SET

#include <string>
#include <vector>

using namespace std;

#define IMPORTANCE_MIN 0

class DataSet {

    string                  file_name;
    string                  plurality_val;
    vector<vector<string>>  content;

    private:

        //void setEntropy();

    public:
        DataSet(string filename);

        void load();
        void load(string filename);

        string get_class(int row);

        string plurality_value();
        string plurality_value(vector<int> &rows);

        bool classEq(vector<int> rows);

        float importance(int attribute, vector<int> rows);

        // void updateClassifications(string label);
        //bool unique_class();
};

#endif
