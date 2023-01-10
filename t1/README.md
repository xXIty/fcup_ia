# Summary

## SYSTEM INFORMATION
- Language used: Python 3.10.2
- Operating System used for development and testing:
    - Linux 5.16.12-arch1-1 #1 SMP PREEMPT Wed, 02 Mar 2022 12:22:51 +0000 x86_64 GNU/Linux
    - Linux 5.7.0-kali1-amd64 #1 SMP Debian 5.7.6-1kali2 (2020-07-01) x86_64 GNU/Linux


## INSTALLATION

python3.10 -m venv .venv
chmod +x .venv/bin/activate
./.venv/bin/activate
python -m pip install -r requirements.txt

**NOTE: The virtual environment should be activated to execute the program**
./venv/bin/activate 



## USAGE

`python jogodo15.py -h`



## EXAMPLES

`python jogodo15.py -r 100 -d 15 -a ASTAR_MAN BFS`

- `-r 100`: Use 100 random configurations as a data set.
- `-d 15`: Optimal solution should not have more than 15 steps.
- `-a ASTAR_MAN BFS`: Use the algorithms A* with Manhattan distance heuristic and BFS to solve the configurations.

`python3 jogodo15.py -i 9 12 13 7 0 14 5 2 6 1 4 8 10 15 3 11 -f 9 5 12 7 14 13 0 8 1 3 2 4 6 10 15 11 -a IDFS -t 30 -p`

- `-i 9 12 13 7 0 14 5 2 6 1 4 8 10 15 3 11`: Set the initial configuration to use.
- `-f 9 5 12 7 14 13 0 8 1 3 2 4 6 10 15 11`: Set the final configuration to use. 
- `-a IDFS`: Use the algorithm IDFS.
- `-t 30`: Set a timeout of 30 seconds.
- `-p`: Show the path for each solutions.
