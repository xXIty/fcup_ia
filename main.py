from  config    import  Config
from  jogodo15  import  *

if __name__ == "__main__":
    N = 4

    FSC        =  Config(N,  [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,0])
    confA      =  Config(N,  [1,2,3,4,5,6,8,12,13,9,0,7,14,11,10,15])
    confB      =  Config(N,  [1,2,3,4,13,6,8,12,5,9,0,7,14,11,10,15])
    confLink1  =  Config(N,  [12,1,10,2,7,11,4,14,5,0,9,15,8,13,6,3])
    confEasy   =  Config(N,  [1,2,3,4,5,6,7,8,9,10,11,12,13,14,0,15])

    #queueingFunction = QueueingFunction.DFS
    queueingFunction = depthFirstSearch
    configInicial = confA
    configFinal   = FSC
    searchResult  = GeneralSearchAlgorithm(queueingFunction,
                                           configInicial,
                                           configFinal)

    if isinstance(searchResult, str):
        print(searchResult)
    else:
        printPath(searchResult)


