import argparse
import signal              
import sys

from    time        import  time
from    tabulate    import  tabulate
from    config      import  Config
from    jogodo15lib    import  *
from    contextlib  import  contextmanager

class SolutionInfo:
    def __init__(self,
                solved,
                depth          =  0,
                executionTime  =  0,
                nodes          =  0,
                pathStr        =  None,
                configFinal    =  None,
                configInicial  =  None):

        self.solved         =  solved
        self.depth          =  depth
        self.executionTime  =  executionTime
        self.nodes          =  nodes
        self.pathStr        =  pathStr
        self.configInicial  =  configInicial
        self.configFinal    =  configFinal

@contextmanager
def timeout(time):
    # Register a function to raise a TimeoutError on the signal.
    signal.signal(signal.SIGALRM, raise_timeout)
    # Schedule the signal to be sent after ``time``.
    signal.alarm(time)

    try:
        yield
    except TimeoutError:
        solutionInfo = SolutionInfo(solved = False)
        return solutionInfo
        pass
    finally:
        # Unregister the signal so it won't be triggered
        # if the timeout is not reached.
        signal.signal(signal.SIGALRM, signal.SIG_IGN)


def raise_timeout(signum, frame):
    raise TimeoutError

def evaluate(configInicial, configFinal, algorithmName, timeoutSeconds):

    queueingFunction = algoritmsDict[algorithmName]

    with timeout(timeoutSeconds):
        start_time = None

        if queueingFunction == iterativeDepthFirstSearch:
            start_time = time()
            searchResult  = iterativeDepthFirstSearch(configInicial,
                                                      configFinal)
        else:
            start_time = time()
            searchResult  = GeneralSearchAlgorithm(queueingFunction,
                                                   configInicial,
                                                   configFinal)
        # Measure time execution of the algorithms
        searchTime = time() - start_time

        # Fill solutionInfo object with data
        solutionInfo = None
        if isinstance(searchResult, str):
            solutionInfo = SolutionInfo(solved = False)
        else:
            solutionInfo                =  SolutionInfo(solved = True)
            solutionInfo.executionTime  =  searchTime                    
            solutionInfo.depth          =  searchResult.depth
            solutionInfo.pathStr        =  getPathString(searchResult)     
            solutionInfo.nodes          =  getNodeCount()
            solutionInfo.configInicial  =  configInicial
            solutionInfo.configFinal    =  configFinal

        return solutionInfo


if __name__ == "__main__":

    #
    # Config comand arguments parser
    # =================================
    #

    parser = argparse.ArgumentParser(
            description     = "The 15 Puzzle solver.",
            formatter_class = argparse.RawTextHelpFormatter)

    parser.add_argument('-r' ,
            dest = 'configs',
            required = False,
            metavar  = 'R',
            type     = int,
            help     = 'Try to solve R random configurations.')

    parser.add_argument('-s' ,
            dest = 'summary',
            required = False,
            default  = False,
            action   = 'store_true',
            help     = 'Hide indivdual solutions and show only the results summary')
    
    parser.add_argument('-p' ,
            dest = 'path',
            required = False,
            default  = False,
            action   = 'store_true',
            help     = 'Show the path to the solution')

    parser.add_argument('-i' ,
            dest = 'configInitial',
            required = False,
            metavar  = 'N',
            choices  = range(16),
            nargs    = 16,
            action   = 'append',
            type     = int,
            help     = 'Solve given configuration (blank tile is represented with a 0).')

    parser.add_argument('-f' ,
            dest = 'configFinal',
            required = False,
            metavar  = 'N',
            default  = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0],
            choices  = range(16),
            nargs    = 16,
            type     = int,
            help     = 'Set goal configuration (blank tile is represented with a 0).')

    parser.add_argument('-d' ,
            dest = 'maxOptimal',
            metavar  = 'D',
            default  = 10,
            type     = int,
            help     = 'Maximum depth for the optimal solution. Default = 10')

    parser.add_argument('-t' ,
            dest = 'timeoutSeconds',
            metavar  = 'T',
            default  = 10,
            type     = int,
            help     = 'Timeout each search after T seconds. Default = 10')

    parser.add_argument('-o' ,
            dest = 'format',
            required = False,
            default  = False,
            action   = 'store_true',
            help     = 'Change table format from grid to tsv (tab separated values)')

    parser.add_argument('-a',
            required = False,
            metavar  = 'A',
            nargs    = '*',
            dest     = 'algorithms',
            type     = str,
            choices  = ['DFS', 'BFS', 'IDFS', 'GREEDY', 'ASTAR'],
            help     = 'Algorithms to use for solving the configruations.\n'+
                       '    - DFS\n'    +
                       '    - BFS\n'    +
                       '    - IDFS\n'   +
                       '    - GREEDY\n' +
                       '    - ASTAR'     )

    args = parser.parse_args()

    #
    # Start of main program
    # =======================
    #

    N = 4

    # Final Standard Configuration of The 15 puzzle
    FSC        =  Config(N,  args.configFinal)


    # C1       =  (Config(N,  [1,2,3,4,9,5,7,8,13,6,10,12,0,14,11,15]),
                 # FSC)        
    # C2       =  (Config(N,  [9,12,13,7,0,14,5,2,6,1,4,8,10,15,3,11]),
                 # Config(N,  [9,5,12,7,14,13,0,8,1,3,2,4,6,10,15,11]))
    # C3       =  (Config(N,  [6,12,5,9,14,2,4,11,0,7,8,13,3,10,1,15]),
                 # Config(N,  [14,6,12,9,7,2,5,11,8,4,13,15,3,10,1,0]))
    # CsemSol  =  (Config(N,  [1,2,3,4,9,5,7,8,13,6,10,12,0,11,14,15]),
                 # FSC)        
    # configsIAMail = [C1, C2, C3, CsemSol]
    frmtgrid = ''
    if args.format:
        frmtgrid = 'tsv'
    else:
        frmtgrid = 'grid'


    configurations  =  []
    algorithms     =  []

    # Generate <args.configs> random solvable configurations
    if args.configInitial is not None:
        for config in args.configInitial:
            configurations.append(Config(N, config))
        
    else:
        if args.configs is None:
            print(sys.argv[0] + ': error: argument -r or -i  must be present')
            exit(2)
        if args.configs < 1:
            print(sys.argv[0] + ': error: argument -r: must be greater than 0')
            exit(2)
        for i in range(args.configs):
            configRandom = Config(N, args.configFinal)
            configRandom.scramble(args.maxOptimal)

            configurations.append(configRandom)

    # Prepare algorithms to use from args.algoritms or all if not specified
    if args.algorithms is None:
        algorithms = list(algoritmsDict.keys())
    else:
        algorithms = args.algorithms


    # Execute each random configuration using the specified algorithms

    outputTableHeaders = ["Strategy", "Time (s)", "Space (nodes)", "Sol. found", "Depth/Cost"]
    outputTableData    = []

    for algorithm in algorithms:
        if algorithm[-4:] == "_MAN":
            setHeuristic(0)
        if algorithm[-4:] == "_MIS":
            setHeuristic(1)

        configsSolved = 0
        solutionInfoMean = SolutionInfo(solved = True)

        for config in configurations:
            configInicial = config
            configFinal   = FSC

            solutionInfo = evaluate(configInicial, configFinal, algorithm, args.timeoutSeconds)

            if solutionInfo is not None and solutionInfo.solved:
                configsSolved += 1

                solutionInfoMean.depth          +=  solutionInfo.depth
                solutionInfoMean.executionTime  +=  solutionInfo.executionTime
                solutionInfoMean.nodes          +=  solutionInfo.nodes

                if not args.summary:

                    print("Solution information")
                    print("\tTime untill solution: " + str(solutionInfo.executionTime) + " s")
                    print("\tSpace: " + str(solutionInfo.nodes) + " nos")
                    print("\tDepth: " + str(solutionInfo.depth) )
                    print("\tAlgorithm: " + algorithm)
                    print("\tConfigInitial: " + str(configInicial.board))
                    print("\tConfigFinal: " + str(configFinal.board))
                    if args.path:
                        print("\tPath to solution: \n" + solutionInfo.pathStr)


        if configsSolved:
            solutionInfoMean.depth         /= configsSolved
            solutionInfoMean.executionTime /= configsSolved
            solutionInfoMean.nodes         /= configsSolved

        outputTableData.append([algorithm,
            solutionInfoMean.executionTime,
            solutionInfoMean.nodes,
            configsSolved,
            solutionInfoMean.depth])
    print()
    print("Results Sumary:")
    print("\tMaximum optimal solution depth: " + str(args.maxOptimal))
    print("\tNumber of configurations tested: " + str(len(configurations)))
    print("\tTimeout used: " + str(args.timeoutSeconds) + ' s')
    print(tabulate(outputTableData, headers=outputTableHeaders, tablefmt=frmtgrid))





