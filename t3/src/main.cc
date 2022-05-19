using namespace std;

vector<vector<string>> parse_dataset(char* filename);
float entropy(vector<vector<string>> dataset);

int main(int argc, char* argv[]) {
  vector<vector<string>> dataset; 
  dataset = parse_dataset(argv[1]);

  entropy(dataset);
  return 0;
}





float entropy(vector<vector<string>> dataset){
  int rows = dataset.size();
  int cols = dataset[0].size();
  vector<string> classes;
  vector<int> count;
  // Determine all the possible values of class and how many for each value
  for(int i = 1; i < rows; i++) {
    string classtag = dataset[i][cols-1];
    bool insert = true;
    for(int j = 0; j < classes.size(); j++) {
      if(classes[j] == classtag) { // Already seen, increment counter
        count[j] += 1;
        insert = false;
        break;
      }
    } 
    if (insert) { // Not seen yet, add value in classes vector, set counter to 1
      classes.push_back(classtag);
      count.push_back(1);
    }
  }

  // Entropy of the dataset
  float entropy = 0;
  for(int j = 0; j < classes.size(); j++) {
    float p = float(count[j])/(rows-1);
    entropy -= p * log2(p);
  }
  cout << "Entropy(Goal): " <<  entropy << endl;


  // test per conseguir information gain
  //
  
  vector<int> empty;
  for(int i = 0; i < classes.size(); i++) empty.push_back(0);

  //
  vector<string> attr_values;
  vector<int> attr_count;
  vector<vector<int>> goals_count;
  int j = 1;
  for(int i = 1; i < rows; i++) {
    string attr_val = dataset[i][j];
    bool insert = true;
    for(int j = 0; j < attr_values.size(); j++) {
      if(attr_values[j] == attr_val) {
        attr_count[j] += 1;
        for (int k = 0; k < classes.size(); k++) {
          if (dataset[i][cols-1] == classes[k]) {
            goals_count[j][k] += 1;
            break;
          }
        }
        insert = false;
        break;
      }
    } 
    if (insert) {
      attr_values.push_back(attr_val);
      attr_count.push_back(1);
      goals_count.push_back(empty);
    }

  }

  // gain =  S(goal) - SUM(((nk+pk)/(n+p) * S(attri = k )))
  float gain = entropy;
  for(int i = 0; i < attr_values.size(); i++) {
    int total = 0;
    float kentropy = 0;
    for(int j = 0; j < classes.size(); j++) total += goals_count[i][j];
    for(int j = 0; j < classes.size(); j++) {
      float p = float(goals_count[i][j])/total;
      kentropy -= p * log2(p);
    }
    gain -= (attr_count[i]/float(rows-1)) * kentropy; 
    cout << "Entropy(ki): " << kentropy << "  GAIN: " << gain << endl;
  }


   return 0.0;

}
