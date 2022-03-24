import argparse
import  signal              

from    time        import  time
from    config      import  Config
from    jogodo15    import  *
from    contextlib  import  contextmanager

@contextmanager
def timeout(time):
    # Register a function to raise a TimeoutError on the signal.
    signal.signal(signal.SIGALRM, raise_timeout)
    # Schedule the signal to be sent after ``time``.
    signal.alarm(time)

    try:
        yield
    except TimeoutError:
        print("TIMEOUT REACHED!!!")
        pass
    finally:
        # Unregister the signal so it won't be triggered
        # if the timeout is not reached.
        signal.signal(signal.SIGALRM, signal.SIG_IGN)


def raise_timeout(signum, frame):
    raise TimeoutError

def evaluate(configInicial, configFinal, queueingFunction, timeoutSeconds):

    algorithmIndex  =  list(algoritmsDict.values()).index(queueingFunction)
    algorithmKey    =  list(algoritmsDict.keys())[algorithmIndex]

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

        if isinstance(searchResult, str):
            print(searchResult)
        else:
            searchLength = printPath(searchResult)

            print("Solution information")
            print("\tOtimalidade: " + str(searchLength))
            print("\tTempo ate solução: " + str(searchTime) + " s")
            print("\tEspaco: " + str(getNodeCount()) + " nos")
            print("\tProfundidade: " + str(searchResult.depth) )
            print("\tAlgoritmo: " + algorithmKey)
            print("\tConfigInicial: " + str(configInicial.board))
            print("\tConfigFinal: " + str(configFinal.board))

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
            required = True,
            metavar  = 'R',
            type     = int,
            help     = 'Try to solve R random configurations.')

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

    parser.add_argument('-a',
            required = False,
            metavar  = 'A',
            nargs    = '*',
            dest     = 'algorithms',
            type     = str,
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
    FSC        =  Config(N,  [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0])

    # confA      =  Config(N,  [1,2,3,4,5,6,8,12,13,9,0,7,14,11,10,15])
    # confB      =  Config(N,  [1,2,3,4,13,6,8,12,5,9,0,7,14,11,10,15])
    # confLink1  =  Config(N,  [12,1,10,2,7,11,4,14,5,0,9,15,8,13,6,3])
    # confEasy   =  Config(N,  [1,2,3,4,5,6,7,8,9,10,11,12,13,14,0,15])
    # confMail   =  Config(N,  [1,2,3,4,9,5,7,8,13,6,10,12,0,14,11,15])

    # C1       =  (Config(N,  [1,2,3,4,9,5,7,8,13,6,10,12,0,14,11,15]),
                 # FSC)        
    # C2       =  (Config(N,  [9,12,13,7,0,14,5,2,6,1,4,8,10,15,3,11]),
                 # Config(N,  [9,5,12,7,14,13,0,8,1,3,2,4,6,10,15,11]))
    # C3       =  (Config(N,  [6,12,5,9,14,2,4,11,0,7,8,13,3,10,1,15]),
                 # Config(N,  [14,6,12,9,7,2,5,11,8,4,13,15,3,10,1,0]))
    # CsemSol  =  (Config(N,  [1,2,3,4,9,5,7,8,13,6,10,12,0,11,14,15]),
                 # FSC)        
    # configsIAMail = [C1, C2, C3, CsemSol]

    configsRandom  =  []
    algorithms     =  []

    # Generate <args.configs> random solvable configurations
    for i in range(args.configs):
        configRandom = Config(FSC.N, FSC.board)
        configRandom.scramble(args.maxOptimal)

        configsRandom.append(configRandom)

    # Prepare algorithms to use from args.algoritms or use all if not specified
    if args.algorithms is None:
        algorithms = list(algoritmsDict.values())
    else:
        for algorithm in args.algorithms:
            algorithms.append(algoritmsDict[algorithm])


    # Execute each random configuration using the specified algorithms
    for config in configsRandom:
        configInicial = config
        configFinal   = FSC
        for algorithm in algorithms:
            evaluate(configInicial, configFinal, algorithm, args.timeoutSeconds)
