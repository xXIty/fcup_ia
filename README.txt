# INSTALLATION

python3 -m venv .venv
chmod +x .venv/bin/activate
./.venv/bin/activate
python -m pip install -r requirements.txt

# NOTE: The virtual environment should be activated to execute the program
./venv/bin/activate 



# USAGE

python jogodo15.py -h



# EXAMPLES

$ python jogodo15.py -r 100 -d 15 -a ASTAR_MAN BFS

# '-r 100': Use 100 random configurations as a data set.
# '-d 15': Optimal solution should not have more than 15 steps.
# '-a ASTAR_MAN BFS': Use the algorithms A* with Manhattan distance heuristic and BFS to solve the configurations.

$ python3 jogodo15.py -i 9 12 13 7 0 14 5 2 6 1 4 8 10 15 3 11 -f 9 5 12 7 14 13 0 8 1 3 2 4 6 10 15 11 -a IDFS -t 30 -p

# '-i 9 12 13 7 0 14 5 2 6 1 4 8 10 15 3 11': Set the initial configuration to use.
# '-f 9 5 12 7 14 13 0 8 1 3 2 4 6 10 15 11': Set the final configuration to use. 
# '-a IDFS': Use the algorithm IDFS.
# '-t 30': Set a timeout of 30 seconds.
# '-p': Show the path for each solutions.
